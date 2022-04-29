use sbor::*;
use scrypto::engine::types::*;
use scrypto::rust::collections::BTreeMap;
use scrypto::rust::collections::BTreeSet;
use scrypto::rust::collections::HashMap;
use scrypto::rust::string::ToString;

/// Represents an error when manipulating resources in a container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceContainerError {
    /// Resource addresses do not match.
    ResourceAddressNotMatching,
    /// The amount is invalid, according to the resource divisibility.
    InvalidAmount(Decimal, u8),
    /// The balance is not enough.
    InsufficientBalance,
    /// Fungible operation on non-fungible resource is not allowed.
    FungibleOperationNotAllowed,
    /// Non-fungible operation on fungible resource is not allowed.
    NonFungibleOperationNotAllowed,
    /// Resource container is locked because there exists proof(s).
    ContainerLocked,
}

#[derive(Debug, TypeId, Encode, Decode)]
pub enum ResourceContainer {
    Fungible {
        /// The resource address.
        resource_address: ResourceAddress,
        /// The resource divisibility.
        divisibility: u8,
        /// The locked amounts and the corresponding times of being locked.
        locked_amounts: BTreeMap<Decimal, usize>,
        /// The liquid amount.
        liquid_amount: Decimal,
    },
    NonFungible {
        /// The resource address.
        resource_address: ResourceAddress,
        /// The locked non-fungible ids and the corresponding times of being locked.
        locked_ids: HashMap<NonFungibleId, usize>,
        /// The liquid non-fungible ids.
        liquid_ids: BTreeSet<NonFungibleId>,
    },
}

/// The locked amount or non-fungible IDs.
///
/// Invariant: always consistent with resource fungibility.
#[derive(Debug, Clone)]
pub enum LockedAmountOrIds {
    Amount(Decimal),
    Ids(BTreeSet<NonFungibleId>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ResourceContainerId {
    Bucket(BucketId),
    Vault(VaultId),
    Worktop(u32, ResourceAddress),
}

impl LockedAmountOrIds {
    pub fn is_empty(&self) -> bool {
        self.amount().is_zero()
    }

    pub fn amount(&self) -> Decimal {
        match self {
            Self::Amount(amount) => amount.clone(),
            Self::Ids(ids) => ids.len().into(),
        }
    }

    pub fn ids(&self) -> Result<BTreeSet<NonFungibleId>, ()> {
        match self {
            Self::Amount(_) => Err(()),
            Self::Ids(ids) => Ok(ids.clone()),
        }
    }
}

impl ResourceContainer {
    pub fn new_fungible(
        resource_address: ResourceAddress,
        divisibility: u8,
        amount: Decimal,
    ) -> Self {
        Self::Fungible {
            resource_address,
            divisibility,
            locked_amounts: BTreeMap::new(),
            liquid_amount: amount,
        }
    }

    pub fn new_non_fungible(
        resource_address: ResourceAddress,
        ids: BTreeSet<NonFungibleId>,
    ) -> Self {
        Self::NonFungible {
            resource_address,
            locked_ids: HashMap::new(),
            liquid_ids: ids.clone(),
        }
    }

    pub fn new_empty(resource_address: ResourceAddress, resource_type: ResourceType) -> Self {
        match resource_type {
            ResourceType::Fungible { divisibility } => {
                Self::new_fungible(resource_address, divisibility, Decimal::zero())
            }
            ResourceType::NonFungible => Self::new_non_fungible(resource_address, BTreeSet::new()),
        }
    }

    pub fn put(&mut self, other: Self) -> Result<(), ResourceContainerError> {
        // check resource address
        if self.resource_address() != other.resource_address() {
            return Err(ResourceContainerError::ResourceAddressNotMatching);
        }

        // Invariant: owned container should always be free
        assert!(!other.is_locked());

        // update liquidity
        match self {
            Self::Fungible { liquid_amount, .. } => {
                *liquid_amount += other.liquid_amount();
            }
            Self::NonFungible { liquid_ids, .. } => {
                liquid_ids.extend(other.liquid_ids()?);
            }
        }
        Ok(())
    }

    pub fn take_by_amount(&mut self, amount: Decimal) -> Result<Self, ResourceContainerError> {
        // check amount granularity
        let divisibility = self.resource_type().divisibility();
        Self::check_amount(amount, divisibility)?;

        // deduct from liquidity pool
        match self {
            Self::Fungible { liquid_amount, .. } => {
                if *liquid_amount < amount {
                    return Err(ResourceContainerError::InsufficientBalance);
                }
                *liquid_amount = *liquid_amount - amount;
                Ok(Self::new_fungible(
                    self.resource_address(),
                    divisibility,
                    amount,
                ))
            }
            Self::NonFungible { liquid_ids, .. } => {
                if Decimal::from(liquid_ids.len()) < amount {
                    return Err(ResourceContainerError::InsufficientBalance);
                }
                let n: usize = amount.to_string().parse().unwrap();
                let ids: BTreeSet<NonFungibleId> = liquid_ids.iter().cloned().take(n).collect();
                self.take_by_ids(&ids)
            }
        }
    }

    pub fn take_by_ids(
        &mut self,
        ids: &BTreeSet<NonFungibleId>,
    ) -> Result<Self, ResourceContainerError> {
        match self {
            Self::Fungible { .. } => Err(ResourceContainerError::NonFungibleOperationNotAllowed),
            Self::NonFungible { liquid_ids, .. } => {
                for id in ids {
                    if !liquid_ids.remove(&id) {
                        return Err(ResourceContainerError::InsufficientBalance);
                    }
                }
                Ok(Self::new_non_fungible(self.resource_address(), ids.clone()))
            }
        }
    }

    pub fn take_all_liquid(&mut self) -> Result<Self, ResourceContainerError> {
        self.take_by_amount(self.liquid_amount())
    }

    pub fn lock_by_amount(
        &mut self,
        amount: Decimal,
    ) -> Result<LockedAmountOrIds, ResourceContainerError> {
        // check amount granularity
        let divisibility = self.resource_type().divisibility();
        Self::check_amount(amount, divisibility)?;

        match self {
            Self::Fungible {
                locked_amounts,
                liquid_amount,
                ..
            } => {
                let max_locked = Self::largest_key(locked_amounts);
                if amount > max_locked {
                    let delta = amount - max_locked;
                    if *liquid_amount >= delta {
                        *liquid_amount -= delta;
                    } else {
                        return Err(ResourceContainerError::InsufficientBalance);
                    }
                }

                locked_amounts.insert(
                    amount,
                    locked_amounts.get(&amount).cloned().unwrap_or(0) + 1,
                );

                Ok(LockedAmountOrIds::Amount(amount))
            }
            Self::NonFungible {
                locked_ids,
                liquid_ids,
                ..
            } => {
                if Decimal::from(locked_ids.len() + liquid_ids.len()) < amount {
                    return Err(ResourceContainerError::InsufficientBalance);
                }

                let n: usize = amount.to_string().parse().unwrap();
                let mut ids: BTreeSet<NonFungibleId> = locked_ids.keys().cloned().take(n).collect();
                if ids.len() < n {
                    ids.extend(liquid_ids.iter().cloned().take(n - ids.len()));
                }

                self.lock_by_ids(&ids)
            }
        }
    }

    pub fn lock_by_ids(
        &mut self,
        ids: &BTreeSet<NonFungibleId>,
    ) -> Result<LockedAmountOrIds, ResourceContainerError> {
        match self {
            Self::NonFungible {
                locked_ids,
                liquid_ids,
                ..
            } => {
                for id in ids {
                    if liquid_ids.remove(id) {
                        // if the non-fungible is liquid, move it to locked.
                        locked_ids.insert(id.clone(), 1);
                    } else if let Some(cnt) = locked_ids.get_mut(id) {
                        // if the non-fungible is locked, increase the ref count.
                        *cnt += 1;
                    } else {
                        return Err(ResourceContainerError::InsufficientBalance);
                    }
                }

                Ok(LockedAmountOrIds::Ids(ids.clone()))
            }
            Self::Fungible { .. } => Err(ResourceContainerError::NonFungibleOperationNotAllowed),
        }
    }

    fn largest_key(map: &BTreeMap<Decimal, usize>) -> Decimal {
        // TODO: remove loop once `last_key_value` is stable.
        map.keys().cloned().max().unwrap_or(Decimal::zero())
    }

    pub fn unlock(&mut self, resource: LockedAmountOrIds) {
        match resource {
            LockedAmountOrIds::Amount(amount) => match self {
                Self::Fungible {
                    locked_amounts,
                    liquid_amount,
                    ..
                } => {
                    let max_locked = Self::largest_key(locked_amounts);
                    let count = locked_amounts
                        .remove(&amount)
                        .expect("Attempted to unlock an amount that is not locked in container");
                    if count > 1 {
                        locked_amounts.insert(amount, count - 1);
                    } else {
                        let new_max_locked = Self::largest_key(locked_amounts);
                        *liquid_amount += max_locked - new_max_locked;
                    }
                }
                Self::NonFungible { .. } => {
                    panic!("Attempted to unlock amount of non-fungible resource")
                }
            },
            LockedAmountOrIds::Ids(ids) => match self {
                Self::NonFungible {
                    locked_ids,
                    liquid_ids,
                    ..
                } => {
                    for id in ids {
                        if let Some(cnt) = locked_ids.remove(&id) {
                            if cnt > 1 {
                                locked_ids.insert(id, cnt - 1);
                            } else {
                                liquid_ids.insert(id);
                            }
                        } else {
                            panic!("Attempted to unlock a non-fungible that is not locked in container");
                        }
                    }
                }
                Self::Fungible { .. } => {
                    panic!("Attempted to unlock non-fungibles of fungible resource")
                }
            },
        }
    }

    pub fn max_locked_amount(&self) -> Decimal {
        match self {
            ResourceContainer::Fungible { locked_amounts, .. } => Self::largest_key(locked_amounts),
            ResourceContainer::NonFungible { locked_ids, .. } => locked_ids.len().into(),
        }
    }

    pub fn max_locked_ids(&self) -> Result<BTreeSet<NonFungibleId>, ResourceContainerError> {
        match self {
            ResourceContainer::Fungible { .. } => {
                Err(ResourceContainerError::NonFungibleOperationNotAllowed)
            }
            ResourceContainer::NonFungible { locked_ids, .. } => {
                Ok(locked_ids.keys().cloned().collect())
            }
        }
    }

    pub fn liquid_amount(&self) -> Decimal {
        match self {
            Self::Fungible { liquid_amount, .. } => *liquid_amount,
            Self::NonFungible { liquid_ids, .. } => liquid_ids.len().into(),
        }
    }

    pub fn liquid_ids(&self) -> Result<BTreeSet<NonFungibleId>, ResourceContainerError> {
        match self {
            Self::Fungible { .. } => Err(ResourceContainerError::NonFungibleOperationNotAllowed),
            Self::NonFungible { liquid_ids, .. } => Ok(liquid_ids.clone()),
        }
    }

    pub fn total_amount(&self) -> Decimal {
        self.max_locked_amount() + self.liquid_amount()
    }

    pub fn total_ids(&self) -> Result<BTreeSet<NonFungibleId>, ResourceContainerError> {
        let mut total = BTreeSet::new();
        total.extend(self.max_locked_ids()?);
        total.extend(self.liquid_ids()?);
        Ok(total)
    }

    pub fn is_locked(&self) -> bool {
        match self {
            Self::Fungible { locked_amounts, .. } => !locked_amounts.is_empty(),
            Self::NonFungible { locked_ids, .. } => !locked_ids.is_empty(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.total_amount().is_zero()
    }

    pub fn resource_address(&self) -> ResourceAddress {
        match self {
            Self::Fungible {
                resource_address, ..
            }
            | Self::NonFungible {
                resource_address, ..
            } => *resource_address,
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        match self {
            Self::Fungible { divisibility, .. } => ResourceType::Fungible {
                divisibility: *divisibility,
            },
            Self::NonFungible { .. } => ResourceType::NonFungible,
        }
    }

    fn check_amount(amount: Decimal, divisibility: u8) -> Result<(), ResourceContainerError> {
        if amount.is_negative() || amount.0 % 10i128.pow((18 - divisibility).into()) != 0.into() {
            Err(ResourceContainerError::InvalidAmount(amount, divisibility))
        } else {
            Ok(())
        }
    }
}
