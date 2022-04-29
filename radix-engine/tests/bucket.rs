#[rustfmt::skip]
pub mod test_runner;

use crate::test_runner::TestRunner;
use radix_engine::errors::*;
use radix_engine::ledger::*;
use radix_engine::model::{BucketError, ResourceContainerError};
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[test]
fn test_bucket() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, true);
    let (_, _, account) = executor.new_account();
    let package = executor
        .publish_package(&compile_package!(format!("./tests/{}", "bucket")))
        .unwrap();

    let transaction = TransactionBuilder::new()
        .call_function(package, "BucketTest", "combine", args![])
        .call_function(package, "BucketTest", "split", args![])
        .call_function(package, "BucketTest", "borrow", args![])
        .call_function(package, "BucketTest", "query", args![])
        .call_function(package, "BucketTest", "test_restricted_transfer", args![])
        .call_function(package, "BucketTest", "test_burn", args![])
        .call_function(package, "BucketTest", "test_burn_freely", args![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([]))
        .sign([]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    assert!(receipt.result.is_ok());
}

#[test]
fn test_bucket_of_badges() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut executor = TransactionExecutor::new(&mut ledger, true);
    let (_, _, account) = executor.new_account();
    let package = executor
        .publish_package(&compile_package!(format!("./tests/{}", "bucket")))
        .unwrap();

    let transaction = TransactionBuilder::new()
        .call_function(package, "BadgeTest", "combine", args![])
        .call_function(package, "BadgeTest", "split", args![])
        .call_function(package, "BadgeTest", "borrow", args![])
        .call_function(package, "BadgeTest", "query", args![])
        .call_method_with_all_resources(account, "deposit_batch")
        .build(executor.get_nonce([]))
        .sign([]);
    let receipt = executor.validate_and_execute(&transaction).unwrap();
    assert!(receipt.result.is_ok());
}

#[test]
fn test_take_with_invalid_granularity() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(&mut substate_store);
    let (pk, sk, account) = test_runner.new_account();
    let resource_address = test_runner.create_fungible_resource(100.into(), 2, account);
    let package_address = test_runner.publish_package("bucket");

    // Act
    let transaction = test_runner
        .new_transaction_builder()
        .call_function_with_abi(
            package_address,
            "BucketTest",
            "take_from_bucket",
            vec![format!("100,{}", resource_address), "1.123".to_owned()],
            Some(account),
            &test_runner.export_abi(package_address, "BucketTest"),
        )
        .unwrap()
        .build(test_runner.get_nonce([pk]))
        .sign([&sk]);
    let receipt = test_runner.validate_and_execute(&transaction);
    println!("{:?}", receipt);

    // Assert
    assert_eq!(
        receipt.result,
        Err(RuntimeError::BucketError(
            BucketError::ResourceContainerError(ResourceContainerError::InvalidAmount(
                dec!("1.123"),
                2
            ))
        ))
    );
}

#[test]
fn test_take_with_negative_amount() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(&mut substate_store);
    let (pk, sk, account) = test_runner.new_account();
    let resource_address = test_runner.create_fungible_resource(100.into(), 2, account);
    let package_address = test_runner.publish_package("bucket");

    // Act
    let transaction = test_runner
        .new_transaction_builder()
        .call_function_with_abi(
            package_address,
            "BucketTest",
            "take_from_bucket",
            vec![format!("100,{}", resource_address), "-2".to_owned()],
            Some(account),
            &test_runner.export_abi(package_address, "BucketTest"),
        )
        .unwrap()
        .build(test_runner.get_nonce([pk]))
        .sign([&sk]);
    let receipt = test_runner.validate_and_execute(&transaction);
    println!("{:?}", receipt);

    // Assert
    assert_eq!(
        receipt.result,
        Err(RuntimeError::BucketError(
            BucketError::ResourceContainerError(ResourceContainerError::InvalidAmount(
                dec!("-2"),
                2
            ))
        ))
    );
}

#[test]
fn create_empty_bucket() {
    // Arrange
    let mut substate_store = InMemorySubstateStore::with_bootstrap();
    let mut test_runner = TestRunner::new(&mut substate_store);
    let (pk, sk, account) = test_runner.new_account();

    // Act
    let transaction = test_runner
        .new_transaction_builder()
        .take_from_worktop(scrypto::prelude::RADIX_TOKEN, |builder, _bucket_id| builder)
        .take_from_worktop_by_amount(
            Decimal::zero(),
            scrypto::prelude::RADIX_TOKEN,
            |builder, _bucket_id| builder,
        )
        .take_from_worktop_by_ids(
            &BTreeSet::new(),
            scrypto::prelude::RADIX_TOKEN,
            |builder, _bucket_id| builder,
        )
        .call_method_with_all_resources(account, "deposit_batch")
        .build(test_runner.get_nonce([pk]))
        .sign([&sk]);
    let receipt = test_runner.validate_and_execute(&transaction);
    println!("{:?}", receipt);

    // Assert
    assert!(receipt.result.is_ok());
}
