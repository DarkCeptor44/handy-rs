use crate::errors::ConcurrentCollectionError;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    sync::{Arc, RwLock},
};

/// A map that can be used to store key-value pairs.
pub trait Map<K, V>: Default {
    /// Creates a new empty map
    fn new() -> Self;

    /// Inserts a key-value pair into the map.
    ///
    /// ## Errors
    ///
    /// - [`ConcurrentCollectionError::Poison`]: The lock is poisoned
    fn insert(&self, key: K, value: V) -> Result<(), ConcurrentCollectionError>;

    /// Retrieves a value from the map.
    fn get(&self, key: &K) -> Option<V>;

    /// Removes a key-value pair from the map.
    fn remove(&self, key: &K) -> Option<V>;

    /// Returns the number of key-value pairs in the map
    fn len(&self) -> usize;

    /// Returns true if the map contains no key-value pairs
    fn is_empty(&self) -> bool;

    /// Returns true if the map contains the specified key
    fn contains_key(&self, key: &K) -> bool;
}

/// A concurrent map that can be used to store key-value pairs.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::collections::{ConcurrentHashMap, Map};
///
/// let map: ConcurrentHashMap<u32, &'static str> = ConcurrentHashMap::new();
/// ```
///
/// ## Errors
///
/// - [`ConcurrentCollectionError::Poison`]: The lock is poisoned
#[derive(Debug)]
pub struct ConcurrentHashMap<K, V> {
    map: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> Default for ConcurrentHashMap<K, V> {
    fn default() -> Self {
        ConcurrentHashMap {
            map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// impl<K, V> ConcurrentHashMap<K, V>
impl<K, V> Map<K, V> for ConcurrentHashMap<K, V>
where
    K: Eq + Hash + Send + Sync,
    V: Copy,
{
    /// Creates a new empty [`crate::collections::ConcurrentHashMap`]
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    /// ```
    fn new() -> Self {
        ConcurrentHashMap::default()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    ///
    /// map.insert("key", "value").unwrap();
    /// ```
    ///
    /// ## Errors
    ///
    /// - [`ConcurrentCollectionError::Poison`]: The lock is poisoned
    fn insert(&self, key: K, value: V) -> Result<(), ConcurrentCollectionError> {
        match self.map.write() {
            Ok(mut guard) => {
                guard.insert(key, value);
                Ok(())
            }
            Err(_) => Err(ConcurrentCollectionError::Poison),
        }
    }

    /// Retrieves a value from the map.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    ///
    /// map.get(&"key").unwrap();
    /// ```
    fn get(&self, key: &K) -> Option<V> {
        match self.map.read() {
            Ok(guard) => guard.get(key).copied(),
            Err(_) => None,
        }
    }

    /// Removes a key-value pair from the map.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    ///
    /// map.insert("key", "value").unwrap();
    /// map.remove(&"key").unwrap();
    /// ```
    fn remove(&self, key: &K) -> Option<V> {
        match self.map.write() {
            Ok(mut guard) => guard.remove(key),
            Err(_) => None,
        }
    }

    /// Returns the number of key-value pairs in the map.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    ///
    /// map.insert("key", "value").unwrap();
    /// assert_eq!(map.len(), 1);
    fn len(&self) -> usize {
        match self.map.read() {
            Ok(guard) => guard.len(),
            Err(_) => 0,
        }
    }

    /// Returns true if the map contains no key-value pairs.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    ///
    /// assert!(map.is_empty());
    ///
    /// map.insert("key", "value").unwrap();
    /// assert!(!map.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        match self.map.read() {
            Ok(guard) => guard.is_empty(),
            Err(_) => false,
        }
    }

    /// Returns true if the map contains the specified key.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentHashMap, Map};
    ///
    /// let map: ConcurrentHashMap<&'static str, &'static str> = ConcurrentHashMap::new();
    ///
    /// map.insert("key", "value").unwrap();
    /// assert!(map.contains_key(&"key"));
    /// ```
    fn contains_key(&self, key: &K) -> bool {
        match self.map.read() {
            Ok(guard) => guard.contains_key(key),
            Err(_) => false,
        }
    }
}

/// A concurrent map that can be used to store key-value pairs in a sorted order.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::collections::{ConcurrentBTreeMap, Map};
///
/// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
/// ```
///
/// ## Errors
///
/// - [`ConcurrentCollectionError::Poison`]: The lock is poisoned
#[derive(Debug)]
pub struct ConcurrentBTreeMap<K, V> {
    map: Arc<RwLock<BTreeMap<K, V>>>,
}

impl<K, V> Default for ConcurrentBTreeMap<K, V> {
    fn default() -> Self {
        ConcurrentBTreeMap {
            map: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}

impl<K, V> Map<K, V> for ConcurrentBTreeMap<K, V>
where
    K: Eq + Ord + Send + Sync,
    V: Copy,
{
    /// Creates a new concurrent map.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    /// ```
    fn new() -> Self {
        ConcurrentBTreeMap::default()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    ///
    /// map.insert(1, 2).unwrap();
    /// ```
    ///
    /// ## Errors
    ///
    /// - [`ConcurrentCollectionError::Poison`]: The lock is poisoned
    fn insert(&self, key: K, value: V) -> Result<(), ConcurrentCollectionError> {
        match self.map.write() {
            Ok(mut guard) => {
                guard.insert(key, value);
                Ok(())
            }
            Err(_) => Err(ConcurrentCollectionError::Poison),
        }
    }

    /// Retrieves the value associated with the specified key.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    ///
    /// if let Some(value) = map.get(&1) {
    ///     // do something
    /// }
    /// ```
    fn get(&self, key: &K) -> Option<V> {
        match self.map.read() {
            Ok(guard) => guard.get(key).copied(),
            Err(_) => None,
        }
    }

    /// Checks if the map contains the specified key.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    ///
    /// if map.contains_key(&1) {
    ///     // do something
    /// }
    /// ```
    fn contains_key(&self, key: &K) -> bool {
        match self.map.read() {
            Ok(guard) => guard.contains_key(key),
            Err(_) => false,
        }
    }

    /// Removes the value associated with the specified key.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    ///
    /// map.remove(&1).unwrap();
    /// ```
    fn remove(&self, key: &K) -> Option<V> {
        match self.map.write() {
            Ok(mut guard) => guard.remove(key),
            Err(_) => None,
        }
    }

    /// Returns the number of key-value pairs in the map.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    ///
    /// map.insert(1, 2).unwrap();
    /// assert_eq!(map.len(), 1);
    /// ```
    fn len(&self) -> usize {
        match self.map.read() {
            Ok(guard) => guard.len(),
            Err(_) => 0,
        }
    }

    /// Checks if the map is empty.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use handy::collections::{ConcurrentBTreeMap, Map};
    ///
    /// let map: ConcurrentBTreeMap<usize, u32> = ConcurrentBTreeMap::new();
    ///
    /// map.insert(1, 2).unwrap();
    /// assert!(!map.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        match self.map.read() {
            Ok(guard) => guard.is_empty(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rayon::prelude::*;

    #[test]
    fn test_concurrent_hash_map() {
        assert_map_works::<ConcurrentHashMap<_, _>>();
    }

    #[test]
    fn test_concurrent_btree_map() {
        assert_map_works::<ConcurrentBTreeMap<_, _>>();
    }

    /// Asserts that a map works as expected.
    ///
    /// This function is only used for testing.
    #[allow(clippy::cast_possible_truncation)]
    fn assert_map_works<M>()
    where
        M: Map<usize, u32> + Send + Sync,
    {
        const NUM_THREADS: usize = 10;
        const OPS_PER_THREAD: usize = 1000;
        const CONTENTION_RANGE: usize = 100;

        let map = M::new();

        // insertions
        (0..NUM_THREADS).into_par_iter().for_each(|thread_id| {
            for i in 0..OPS_PER_THREAD {
                let key = thread_id * OPS_PER_THREAD + i;
                let value = (key * 2) as u32;
                map.insert(key, value).unwrap();
            }
        });

        assert_eq!(map.len(), NUM_THREADS * OPS_PER_THREAD);

        for thread_id in 0..NUM_THREADS {
            for i in 0..OPS_PER_THREAD {
                let key = thread_id * OPS_PER_THREAD + i;
                let value = (key * 2) as u32;
                assert_eq!(map.get(&key), Some(value));
            }
        }

        // reads and writes
        (0..NUM_THREADS).into_par_iter().for_each(|_| {
            let mut rng = rand::rng();
            for _ in 0..OPS_PER_THREAD {
                let key = rng.random_range(0..1000);
                let operation = rng.random_range(0..3);

                match operation {
                    0 => if let Some(_value) = map.get(&key) {},
                    1 => {
                        let value: u32 = rng.random();
                        map.insert(key, value).unwrap();
                    }
                    2 => if let Some(_value) = map.remove(&key) {},
                    _ => unreachable!(),
                }
            }
        });

        // high contention
        (0..NUM_THREADS).into_par_iter().for_each(|_| {
            let mut rng = rand::rng();
            for _ in 0..OPS_PER_THREAD {
                let key = rng.random_range(0..CONTENTION_RANGE);
                let operation = rng.random_range(0..2);

                match operation {
                    0 => if let Some(_value) = map.get(&key) {},
                    1 => {
                        let value: u32 = rng.random();
                        map.insert(key, value).unwrap();
                    }
                    _ => unreachable!(),
                }
            }
        });
    }
}
