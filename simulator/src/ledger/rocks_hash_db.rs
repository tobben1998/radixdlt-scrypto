use hash_db::{AsHashDB, HashDB, HashDBRef, Hasher as KeyHasher, Prefix};
use rocksdb::{DBWithThreadMode, SingleThreaded, DB};
use std::{cmp::Eq, collections::hash_map::Entry, collections::HashMap, hash, marker::PhantomData};

pub struct RocksHashDb<H, KF, T>
where
    H: KeyHasher,
    KF: KeyFunction<H>,
{
    db: DBWithThreadMode<SingleThreaded>,
    data: HashMap<KF::Key, (T, i32)>,
    hashed_null_node: H::Out,
    null_node_data: T,
    _kf: PhantomData<KF>,
}

pub trait KeyFunction<H: KeyHasher> {
    type Key: Send + Sync + Clone + hash::Hash + Eq + AsRef<[u8]>;

    fn key(hash: &H::Out, prefix: Prefix) -> Self::Key;
}

/// Key function that only uses the hash
pub struct HashKey<H>(PhantomData<H>);

impl<H> Clone for HashKey<H> {
    fn clone(&self) -> Self {
        Self(Default::default())
    }
}

impl<H> core::fmt::Debug for HashKey<H> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "HashKey")
    }
}

impl<H: KeyHasher> KeyFunction<H> for HashKey<H> {
    type Key = H::Out;

    fn key(hash: &H::Out, prefix: Prefix) -> H::Out {
        hash_key::<H>(hash, prefix)
    }
}

/// Make database key from hash only.
pub fn hash_key<H: KeyHasher>(key: &H::Out, _prefix: Prefix) -> H::Out {
    *key
}

/// Key function that concatenates prefix and hash.
pub struct PrefixedKey<H>(PhantomData<H>);

impl<H> Clone for PrefixedKey<H> {
    fn clone(&self) -> Self {
        Self(Default::default())
    }
}

impl<H> core::fmt::Debug for PrefixedKey<H> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "PrefixedKey")
    }
}

impl<H: KeyHasher> KeyFunction<H> for PrefixedKey<H> {
    type Key = Vec<u8>;

    fn key(hash: &H::Out, prefix: Prefix) -> Vec<u8> {
        prefixed_key::<H>(hash, prefix)
    }
}

/// Derive a database key from hash value of the node (key) and  the node prefix.
pub fn prefixed_key<H: KeyHasher>(key: &H::Out, prefix: Prefix) -> Vec<u8> {
    let mut prefixed_key = Vec::with_capacity(key.as_ref().len() + prefix.0.len() + 1);
    prefixed_key.extend_from_slice(prefix.0);
    if let Some(last) = prefix.1 {
        prefixed_key.push(last);
    }
    prefixed_key.extend_from_slice(key.as_ref());
    prefixed_key
}

/// Create a new `RocksHashDb` from a given null key/data
impl<H, KF, T> RocksHashDb<H, KF, T>
where
    H: KeyHasher,
    T: Default,
    KF: KeyFunction<H>,
{
    /// Remove an element and delete it from storage if reference count reaches zero.
    /// If the value was purged, return the old value.
    pub fn remove_and_purge(&mut self, key: &<H as KeyHasher>::Out, prefix: Prefix) -> Option<T> {
        if key == &self.hashed_null_node {
            return None;
        }
        let key = KF::key(key, prefix);
        self.db.get(&key).unwrap(); // YULONG TESTING
        match self.data.entry(key) {
            Entry::Occupied(mut entry) => {
                if entry.get().1 == 1 {
                    let (value, _) = entry.remove();

                    Some(value)
                } else {
                    entry.get_mut().1 -= 1;
                    None
                }
            }
            Entry::Vacant(entry) => {
                let value = T::default();

                entry.insert((value, -1));
                None
            }
        }
    }
}

impl<H, KF, T> RocksHashDb<H, KF, T>
where
    H: KeyHasher,
    T: for<'a> From<&'a [u8]>,
    KF: KeyFunction<H>,
{
    pub fn new(root: std::path::PathBuf) -> Self {
        Self::from_null_node(root, &[0u8][..], [0u8][..].into())
    }

    /// Create a new `RocksHashDb` from a given null key/data
    pub fn from_null_node(root: std::path::PathBuf, null_key: &[u8], null_node_data: T) -> Self {
        RocksHashDb {
            db: DB::open_default(root.as_path()).unwrap(),
            data: HashMap::default(),
            hashed_null_node: H::hash(null_key),
            null_node_data,

            _kf: Default::default(),
        }
    }

    /// Grab the raw information associated with a key. Returns None if the key
    /// doesn't exist.
    ///
    /// Even when Some is returned, the data is only guaranteed to be useful
    /// when the refs > 0.
    pub fn raw(&self, key: &<H as KeyHasher>::Out, prefix: Prefix) -> Option<(&T, i32)> {
        if key == &self.hashed_null_node {
            return Some((&self.null_node_data, 1));
        }
        let key = KF::key(key, prefix);
        self.db.get(&key).unwrap(); // YULONG TESTING
        self.data.get(&key).map(|(value, count)| (value, *count))
    }
}

impl<H, KF, T> HashDB<H, T> for RocksHashDb<H, KF, T>
where
    H: KeyHasher,
    T: Default + PartialEq<T> + AsRef<[u8]> + for<'a> From<&'a [u8]> + Clone + Send + Sync,
    KF: KeyFunction<H> + Send + Sync,
{
    fn get(&self, key: &H::Out, prefix: Prefix) -> Option<T> {
        if key == &self.hashed_null_node {
            return Some(self.null_node_data.clone());
        }

        let key = KF::key(key, prefix);
        self.db.get(&key).unwrap(); // YULONG TESTING
        match self.data.get(&key) {
            Some(&(ref d, rc)) if rc > 0 => Some(d.clone()),
            _ => None,
        }
    }

    fn contains(&self, key: &H::Out, prefix: Prefix) -> bool {
        if key == &self.hashed_null_node {
            return true;
        }

        let key = KF::key(key, prefix);
        self.db.get(&key).unwrap(); // YULONG TESTING
        match self.data.get(&key) {
            Some(&(_, x)) if x > 0 => true,
            _ => false,
        }
    }

    fn emplace(&mut self, key: H::Out, prefix: Prefix, value: T) {
        if value == self.null_node_data {
            return;
        }

        let key = KF::key(&key, prefix);
        self.db.put(&key, &value).unwrap(); // YULONG TESTING
        match self.data.entry(key) {
            Entry::Occupied(mut entry) => {
                let &mut (ref mut old_value, ref mut rc) = entry.get_mut();
                if *rc <= 0 {
                    *old_value = value;
                }
                *rc += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert((value, 1));
            }
        }
    }

    fn insert(&mut self, prefix: Prefix, value: &[u8]) -> H::Out {
        if T::from(value) == self.null_node_data {
            return self.hashed_null_node;
        }

        let key = H::hash(value);
        HashDB::emplace(self, key, prefix, value.into());
        key
    }

    fn remove(&mut self, key: &H::Out, prefix: Prefix) {
        if key == &self.hashed_null_node {
            return;
        }

        let key = KF::key(key, prefix);
        // DON'T DELETE ENTRY AT ALL // YULONG TESTING
        match self.data.entry(key) {
            Entry::Occupied(mut entry) => {
                let &mut (_, ref mut rc) = entry.get_mut();
                *rc -= 1;
            }
            Entry::Vacant(entry) => {
                let value = T::default();
                entry.insert((value, -1));
            }
        }
    }
}

impl<H, KF, T> HashDBRef<H, T> for RocksHashDb<H, KF, T>
where
    H: KeyHasher,
    T: Default + PartialEq<T> + AsRef<[u8]> + for<'a> From<&'a [u8]> + Clone + Send + Sync,
    KF: KeyFunction<H> + Send + Sync,
{
    fn get(&self, key: &H::Out, prefix: Prefix) -> Option<T> {
        HashDB::get(self, key, prefix)
    }
    fn contains(&self, key: &H::Out, prefix: Prefix) -> bool {
        HashDB::contains(self, key, prefix)
    }
}

impl<H, KF, T> AsHashDB<H, T> for RocksHashDb<H, KF, T>
where
    H: KeyHasher,
    T: Default + PartialEq<T> + AsRef<[u8]> + for<'a> From<&'a [u8]> + Clone + Send + Sync,
    KF: KeyFunction<H> + Send + Sync,
{
    fn as_hash_db(&self) -> &dyn HashDB<H, T> {
        self
    }
    fn as_hash_db_mut(&mut self) -> &mut dyn HashDB<H, T> {
        self
    }
}
