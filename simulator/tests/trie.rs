use radix_engine::model::*;
use rand::prelude::*;
use scrypto::buffer::*;
use scrypto::engine::types::*;
use scrypto::resource::ResourceDefId;
use simulator::ledger::{HashKey, RocksHashDb};
use std::collections::HashMap;
use std::default::Default;
use std::path::PathBuf;
use std::time::Instant;
use trie_db::{TrieDBMut, TrieHash, TrieLayout, TrieMut};
use trie_root::TrieStream;

#[derive(sbor::TypeId, sbor::Encode, sbor::Decode)]
struct TestComponent {
    lp_resource_def: ResourceDefId,
    lp_mint_badge: VaultId,
    a_pool: VaultId,
    b_pool: VaultId,
    fee: Decimal,
    lp_per_asset_ratio: Decimal,
}

const ITERATIONS: usize = 10000;
const NUM_PACKAGES: usize = 1;
const NUM_COMPONENTS: usize = 5;
const NUM_RESOURCE_DEFS: usize = 100;
const NUM_VAULTS: usize = 1000;

fn bench<L: TrieLayout, S: TrieStream>() {
    println!("{}", "-".repeat(80));
    let package = scrypto_encode(&Package::new(vec![1u8; 200_000]));
    let component = scrypto_encode(&Component::new(
        scrypto::component::PackageId([1u8; 26]),
        "BlueprintName".to_string(),
        scrypto_encode(&TestComponent {
            lp_resource_def: ResourceDefId([1u8; 26]),
            lp_mint_badge: (Hash([1u8; 32]), 1),
            a_pool: (Hash([1u8; 32]), 2),
            b_pool: (Hash([1u8; 32]), 3),
            fee: 1u8.into(),
            lp_per_asset_ratio: 2u8.into(),
        }),
    ));
    let resource_def = scrypto_encode(
        &ResourceDef::new(
            ResourceType::NonFungible,
            HashMap::from([
                ("name".to_string(), "bitcoin".to_string()),
                ("description".to_string(), "Bitcoin is a decentralized digital currency, without a central bank or single administrator, that can be sent from user to user on the peer-to-peer bitcoin network without the need for intermediaries.".to_string()),
                ("symbol".to_string(), "btc".to_string()),
                ("url".to_string(), "https://bitcoin.org/en/".to_string()),
            ]),
            0,
            0,
            HashMap::from([
                (ResourceDefId([1u8; 26]), 0),
                (ResourceDefId([2u8; 26]), 0),
                (ResourceDefId([3u8; 26]), 0),
                (ResourceDefId([4u8; 26]), 0),
                (ResourceDefId([5u8; 26]), 0),
                (ResourceDefId([6u8; 26]), 0),
                (ResourceDefId([7u8; 26]), 0),
                (ResourceDefId([8u8; 26]), 0),
            ]),
            &None,
        )
        .unwrap(),
    );
    let vault = scrypto_encode(&Vault::new(Bucket::new(
        ResourceDefId([5u8; 26]),
        ResourceType::Fungible { divisibility: 0 },
        Resource::Fungible {
            amount: 1000u32.into(),
        },
    )));

    let mut r = rand::thread_rng();
    let mut key = [0u8; 63];
    let mut hash_db = RocksHashDb::<_, HashKey<L::Hash>, _>::new(PathBuf::from("data"));
    let mut root = <TrieHash<L>>::default();
    let mut t = TrieDBMut::<L>::new(&mut hash_db, &mut root);
    for i in 0..ITERATIONS {
        let now = Instant::now();

        for _ in 0..NUM_PACKAGES {
            r.fill_bytes(&mut key);
            t.insert(&key, &package).unwrap();
        }
        for _ in 0..NUM_COMPONENTS {
            r.fill_bytes(&mut key);
            t.insert(&key, &component).unwrap();
        }
        for _ in 0..NUM_RESOURCE_DEFS {
            r.fill_bytes(&mut key);
            t.insert(&key, &resource_def).unwrap();
        }
        for _ in 0..NUM_VAULTS {
            r.fill_bytes(&mut key);
            t.insert(&key, &vault).unwrap();
        }
        t.commit();

        if i % (ITERATIONS / 100) == 0 {
            println!(
                "Progress: {}%, Speed: {} us per insertion",
                100 * i / ITERATIONS,
                now.elapsed().as_micros()
                    / (NUM_PACKAGES + NUM_COMPONENTS + NUM_RESOURCE_DEFS + NUM_VAULTS) as u128
            );
        }
    }
    println!(
        "Entry sizes: package = {}, component = {}, resource_def = {}, vault = {}",
        package.len(),
        component.len(),
        resource_def.len(),
        vault.len(),
    );
    println!(
        "Entity composition: {} / {} / {} / {}",
        NUM_PACKAGES, NUM_COMPONENTS, NUM_RESOURCE_DEFS, NUM_VAULTS
    );
    println!(
        "Total size: {} bytes",
        ITERATIONS
            * (NUM_PACKAGES * package.len()
                + NUM_COMPONENTS * component.len()
                + NUM_RESOURCE_DEFS * resource_def.len()
                + NUM_VAULTS * vault.len())
    );
    println!("{}", "-".repeat(80));
}

#[test]
fn bench_trie_insert_keccak() {
    // const EMPTY_TRIE: u8 = 0;
    // const LEAF_NODE_OFFSET: u8 = 1;
    // const EXTENSION_NODE_OFFSET: u8 = 128;
    // const BRANCH_NODE_NO_VALUE: u8 = 254;
    // const BRANCH_NODE_WITH_VALUE: u8 = 255;
    // const LEAF_NODE_OVER: u8 = EXTENSION_NODE_OFFSET - LEAF_NODE_OFFSET;
    // const EXTENSION_NODE_OVER: u8 = BRANCH_NODE_NO_VALUE - EXTENSION_NODE_OFFSET;
    // const LEAF_NODE_LAST: u8 = EXTENSION_NODE_OFFSET - 1;
    // const EXTENSION_NODE_LAST: u8 = BRANCH_NODE_NO_VALUE - 1;

    bench::<reference_trie::ExtensionLayout, reference_trie::ReferenceTrieStream>();
}
