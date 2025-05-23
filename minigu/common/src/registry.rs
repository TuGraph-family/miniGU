use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use crate::increment::Increment;

#[derive(Debug, Default)]
pub struct Registry<K, V> {
    next_key: K,
    object_map: HashMap<K, V>,
}

impl<K, V> Registry<K, V> {
    pub fn new() -> Self
    where
        K: Default,
    {
        Self {
            next_key: K::default(),
            object_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, object: V) -> Option<K>
    where
        K: Increment + Hash + Eq,
    {
        let key = self.next_key.increment()?;
        self.object_map.insert(key.clone(), object);
        Some(key)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Hash + Eq,
        Q: Hash + Eq + ?Sized,
    {
        self.object_map.remove(key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Hash + Eq,
        Q: Hash + Eq + ?Sized,
    {
        self.object_map.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.object_map.iter()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_simple_registry() {
        let mut registry = Registry::new();
        let key1: u32 = registry.insert(1).unwrap();
        let key2 = registry.insert(2).unwrap();

        assert_eq!(registry.get(&key1), Some(&1));
        assert_eq!(registry.get(&key2), Some(&2));

        assert_eq!(registry.remove(&key1), Some(1));
        assert_eq!(registry.get(&key1), None);

        let kv = registry.iter().collect_vec();
        assert_eq!(kv, vec![(&key2, &2)]);
    }
}
