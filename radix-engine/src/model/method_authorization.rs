use sbor::*;
use scrypto::engine::types::*;
use scrypto::rust::vec::Vec;

use crate::model::method_authorization::MethodAuthorizationError::NotAuthorized;
use crate::model::{AuthZone, Proof};

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum MethodAuthorizationError {
    NotAuthorized,
    UnsupportedMethod,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum HardDecimal {
    Amount(Decimal),
    SoftDecimalNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum HardCount {
    Count(u8),
    SoftCountNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum HardResourceOrNonFungible {
    NonFungible(NonFungibleAddress),
    Resource(ResourceAddress),
    SoftResourceNotFound,
}

impl HardResourceOrNonFungible {
    pub fn proof_matches(&self, proof: &Proof) -> bool {
        match self {
            HardResourceOrNonFungible::NonFungible(non_fungible_address) => {
                let proof_resource_address = proof.resource_address();
                proof_resource_address == non_fungible_address.resource_address()
                    && match proof.total_ids() {
                        Ok(ids) => ids.contains(&non_fungible_address.non_fungible_id()),
                        Err(_) => false,
                    }
            }
            HardResourceOrNonFungible::Resource(resource_address) => {
                let proof_resource_address = proof.resource_address();
                proof_resource_address == *resource_address
            }
            HardResourceOrNonFungible::SoftResourceNotFound => false,
        }
    }

    pub fn check_has_amount(&self, amount: Decimal, auth_zones: &[&AuthZone]) -> bool {
        for auth_zone in auth_zones {
            // FIXME: Need to check the composite max amount rather than just each proof individually
            if auth_zone.proofs
                .iter()
                .any(|p| self.proof_matches(p) && p.total_amount() >= amount)
            {
                return true;
            }
        }

        false
    }

    pub fn check(&self, auth_zones: &[&AuthZone]) -> bool {
        for auth_zone in auth_zones {
            if auth_zone.proofs.iter().any(|p| self.proof_matches(p)) {
                return true;
            }
        }

        false
    }
}

impl From<NonFungibleAddress> for HardResourceOrNonFungible {
    fn from(non_fungible_address: NonFungibleAddress) -> Self {
        HardResourceOrNonFungible::NonFungible(non_fungible_address)
    }
}

impl From<ResourceAddress> for HardResourceOrNonFungible {
    fn from(resource_address: ResourceAddress) -> Self {
        HardResourceOrNonFungible::Resource(resource_address)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum HardProofRuleResourceList {
    List(Vec<HardResourceOrNonFungible>),
    SoftResourceListNotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum HardProofRule {
    This(HardResourceOrNonFungible),
    SomeOfResource(HardDecimal, HardResourceOrNonFungible),
    AllOf(HardProofRuleResourceList),
    AnyOf(HardProofRuleResourceList),
    CountOf(HardCount, HardProofRuleResourceList),
}

impl HardProofRule {
    pub fn check(&self, auth_zones: &[&AuthZone]) -> Result<(), MethodAuthorizationError> {
        match self {
            HardProofRule::This(resource) => {
                if resource.check(auth_zones) {
                    Ok(())
                } else {
                    Err(NotAuthorized)
                }
            }
            HardProofRule::SomeOfResource(HardDecimal::Amount(amount), resource) => {
                if resource.check_has_amount(*amount, auth_zones) {
                    Ok(())
                } else {
                    Err(NotAuthorized)
                }
            }
            HardProofRule::AllOf(HardProofRuleResourceList::List(resources)) => {
                for resource in resources {
                    if !resource.check(auth_zones) {
                        return Err(NotAuthorized);
                    }
                }

                Ok(())
            }
            HardProofRule::AnyOf(HardProofRuleResourceList::List(resources)) => {
                for resource in resources {
                    if resource.check(auth_zones) {
                        return Ok(());
                    }
                }

                Err(NotAuthorized)
            }
            HardProofRule::CountOf(
                HardCount::Count(count),
                HardProofRuleResourceList::List(resources),
            ) => {
                let mut left = count.clone();
                for resource in resources {
                    if resource.check(auth_zones) {
                        left -= 1;
                        if left == 0 {
                            return Ok(());
                        }
                    }
                }
                Err(NotAuthorized)
            }
            _ => Err(NotAuthorized),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum HardAuthRule {
    ProofRule(HardProofRule),
    AnyOf(Vec<HardAuthRule>),
    AllOf(Vec<HardAuthRule>),
}

impl HardAuthRule {
    fn check(&self, auth_zones: &[&AuthZone]) -> Result<(), MethodAuthorizationError> {
        match self {
            HardAuthRule::ProofRule(rule) => rule.check(auth_zones),
            HardAuthRule::AnyOf(rules) => {
                if !rules.iter().any(|r| r.check(auth_zones).is_ok()) {
                    return Err(NotAuthorized);
                }
                Ok(())
            }
            HardAuthRule::AllOf(rules) => {
                if rules.iter().any(|r| r.check(auth_zones).is_err()) {
                    return Err(NotAuthorized);
                }
                Ok(())
            }
        }
    }
}

/// Snode which verifies authorization of a method call
#[derive(Debug, Clone, PartialEq, Eq, Hash, TypeId, Encode, Decode)]
pub enum MethodAuthorization {
    Protected(HardAuthRule),
    AllowAll,
    DenyAll,
    Unsupported,
}

impl MethodAuthorization {
    pub fn check(&self, auth_zones: &[&AuthZone]) -> Result<(), MethodAuthorizationError> {
        match self {
            MethodAuthorization::Protected(rule) => rule.check(auth_zones),
            MethodAuthorization::AllowAll => Ok(()),
            MethodAuthorization::DenyAll => Err(MethodAuthorizationError::NotAuthorized),
            MethodAuthorization::Unsupported => Err(MethodAuthorizationError::UnsupportedMethod),
        }
    }
}
