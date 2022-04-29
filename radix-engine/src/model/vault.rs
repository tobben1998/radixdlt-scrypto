use sbor::*;
use scrypto::buffer::scrypto_decode;
use scrypto::engine::types::*;
use scrypto::rust::cell::{Ref, RefCell, RefMut};
use scrypto::rust::collections::BTreeSet;
use scrypto::rust::collections::HashMap;
use scrypto::rust::rc::Rc;
use scrypto::rust::string::String;
use scrypto::rust::string::ToString;
use scrypto::rust::vec::Vec;
use scrypto::values::ScryptoValue;
use crate::engine::SystemApi;

use crate::model::{
    Bucket, Proof, ProofError, ResourceContainer, ResourceContainerError, ResourceContainerId,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VaultError {
    InvalidRequestData(DecodeError),
    ResourceContainerError(ResourceContainerError),
    MethodNotFound(String),
    CouldNotCreateBucket,
    CouldNotTakeBucket,
    ProofError(ProofError),
    CouldNotCreateProof,
}

/// A persistent resource container.
#[derive(Debug, TypeId, Encode, Decode)]
pub struct Vault {
    container: Rc<RefCell<ResourceContainer>>,
}

impl Vault {
    pub fn new(container: ResourceContainer) -> Self {
        Self {
            container: Rc::new(RefCell::new(container)),
        }
    }

    pub fn put(&mut self, other: Bucket) -> Result<(), ResourceContainerError> {
        self.borrow_container_mut().put(other.into_container()?)
    }

    fn take(&mut self, amount: Decimal) -> Result<ResourceContainer, VaultError> {
        let container = self
            .borrow_container_mut()
            .take_by_amount(amount)
            .map_err(VaultError::ResourceContainerError)?;
        Ok(container)
    }

    fn take_non_fungibles(&mut self, ids: &BTreeSet<NonFungibleId>) -> Result<ResourceContainer, VaultError> {
        let container = self
            .borrow_container_mut()
            .take_by_ids(ids)
            .map_err(VaultError::ResourceContainerError)?;
        Ok(container)
    }

    pub fn create_proof(&mut self, container_id: ResourceContainerId) -> Result<Proof, ProofError> {
        match self.resource_type() {
            ResourceType::Fungible { .. } => {
                self.create_proof_by_amount(self.total_amount(), container_id)
            }
            ResourceType::NonFungible => {
                self.create_proof_by_ids(&self.total_ids().unwrap(), container_id)
            }
        }
    }

    pub fn create_proof_by_amount(
        &mut self,
        amount: Decimal,
        container_id: ResourceContainerId,
    ) -> Result<Proof, ProofError> {
        // lock the specified amount
        let locked_amount_or_ids = self
            .borrow_container_mut()
            .lock_by_amount(amount)
            .map_err(ProofError::ResourceContainerError)?;

        // produce proof
        let mut evidence = HashMap::new();
        evidence.insert(
            container_id,
            (self.container.clone(), locked_amount_or_ids.clone()),
        );
        Proof::new(
            self.resource_address(),
            self.resource_type(),
            locked_amount_or_ids,
            evidence,
        )
    }

    pub fn create_proof_by_ids(
        &mut self,
        ids: &BTreeSet<NonFungibleId>,
        container_id: ResourceContainerId,
    ) -> Result<Proof, ProofError> {
        // lock the specified id set
        let locked_amount_or_ids = self
            .borrow_container_mut()
            .lock_by_ids(ids)
            .map_err(ProofError::ResourceContainerError)?;

        // produce proof
        let mut evidence = HashMap::new();
        evidence.insert(
            container_id,
            (self.container.clone(), locked_amount_or_ids.clone()),
        );
        Proof::new(
            self.resource_address(),
            self.resource_type(),
            locked_amount_or_ids,
            evidence,
        )
    }

    pub fn resource_address(&self) -> ResourceAddress {
        self.borrow_container().resource_address()
    }

    pub fn resource_type(&self) -> ResourceType {
        self.borrow_container().resource_type()
    }

    pub fn total_amount(&self) -> Decimal {
        self.borrow_container().total_amount()
    }

    pub fn total_ids(&self) -> Result<BTreeSet<NonFungibleId>, ResourceContainerError> {
        self.borrow_container().total_ids()
    }

    pub fn is_locked(&self) -> bool {
        self.borrow_container().is_locked()
    }

    pub fn is_empty(&self) -> bool {
        self.borrow_container().is_empty()
    }

    fn borrow_container(&self) -> Ref<ResourceContainer> {
        self.container.borrow()
    }

    fn borrow_container_mut(&mut self) -> RefMut<ResourceContainer> {
        self.container.borrow_mut()
    }

    pub fn main<S: SystemApi>(
        &mut self,
        vault_id: VaultId,
        function: &str,
        args: Vec<ScryptoValue>,
        system_api: &mut S
    ) -> Result<ScryptoValue, VaultError> {
        match function {
            "put_into_vault" => {
                let bucket: scrypto::resource::Bucket =
                    scrypto_decode(&args[0].raw).map_err(|e| VaultError::InvalidRequestData(e))?;
                let bucket = system_api.take_bucket(bucket.0).map_err(|_| VaultError::CouldNotTakeBucket)?;
                self.put(bucket).map_err(VaultError::ResourceContainerError)?;
                Ok(ScryptoValue::from_value(&()))
            }
            "take_from_vault" => {
                let amount: Decimal =
                    scrypto_decode(&args[0].raw).map_err(|e| VaultError::InvalidRequestData(e))?;
                let container = self.take(amount)?;
                let bucket_id = system_api.create_bucket(container).map_err(|_| VaultError::CouldNotCreateBucket)?;
                Ok(ScryptoValue::from_value(&scrypto::resource::Bucket(bucket_id)))
            }
            "take_non_fungibles_from_vault" => {
                let non_fungible_ids: BTreeSet<NonFungibleId> =
                    scrypto_decode(&args[0].raw).map_err(|e| VaultError::InvalidRequestData(e))?;
                let container = self.take_non_fungibles(&non_fungible_ids)?;
                let bucket_id = system_api.create_bucket(container).map_err(|_| VaultError::CouldNotCreateBucket)?;
                Ok(ScryptoValue::from_value(&scrypto::resource::Bucket(bucket_id)))
            }
            "get_vault_amount" => {
                let amount = self.total_amount();
                Ok(ScryptoValue::from_value(&amount))
            }
            "get_vault_resource_address" => {
                let resource_address = self.resource_address();
                Ok(ScryptoValue::from_value(&resource_address))
            }
            "get_non_fungible_ids_in_vault" => {
                let ids = self.total_ids().map_err(VaultError::ResourceContainerError)?;
                Ok(ScryptoValue::from_value(&ids))
            }
            "create_vault_proof" => {
                let proof = self.create_proof(ResourceContainerId::Vault(vault_id)).map_err(VaultError::ProofError)?;
                let proof_id = system_api.create_proof(proof).map_err(|_| VaultError::CouldNotCreateProof)?;
                Ok(ScryptoValue::from_value(&scrypto::resource::Proof(proof_id)))
            }
            "create_vault_proof_by_amount" => {
                let amount: Decimal =
                    scrypto_decode(&args[0].raw).map_err(|e| VaultError::InvalidRequestData(e))?;
                let proof = self.create_proof_by_amount(amount, ResourceContainerId::Vault(vault_id)).map_err(VaultError::ProofError)?;
                let proof_id = system_api.create_proof(proof).map_err(|_| VaultError::CouldNotCreateProof)?;
                Ok(ScryptoValue::from_value(&scrypto::resource::Proof(proof_id)))
            }
            "create_vault_proof_by_ids" => {
                let ids = scrypto_decode(&args[0].raw).map_err(|e| VaultError::InvalidRequestData(e))?;
                let proof = self.create_proof_by_ids(&ids, ResourceContainerId::Vault(vault_id)).map_err(VaultError::ProofError)?;
                let proof_id = system_api.create_proof(proof).map_err(|_| VaultError::CouldNotCreateProof)?;
                Ok(ScryptoValue::from_value(&scrypto::resource::Proof(proof_id)))
            }
            _ => Err(VaultError::MethodNotFound(function.to_string())),
        }
    }
}
