// lru_cache.rs (challenge)
//
// A cache has limited room. When it's full and something new arrives, an
// LRU ("least recently used") cache evicts whichever entry was used
// longest ago. Caches like this sit in front of databases, in CPUs,
// and in web browsers.
//
// You design the struct this time: choose its fields yourself. A
// `HashMap` gives fast lookups but has no order; a `VecDeque` keeps order
// but is slow to search. Combining the two is a good first version.
//
// Requirements:
//   1. `new(capacity)` panics if capacity is 0.
//   2. `get` returns the value and marks the key as most recently used.
//   3. `put` inserts or updates. Updating an existing key never evicts,
//      but does mark it as most recently used. Inserting into a full
//      cache evicts the least recently used entry and returns it.
//   4. `contains` checks for a key *without* counting as a use.
//
// Stretch goal: the HashMap + VecDeque version makes `get` O(n), because
// it has to find the key in the queue. Real LRU caches are O(1) for
// everything, using a hash map plus a doubly linked list. Try storing
// nodes in a Vec and linking them by index (instead of with pointers) to
// keep the borrow checker happy.
//
// Further reading:
//   - Cache replacement policies (see "LRU"):
//     https://en.wikipedia.org/wiki/Cache_replacement_policies
//   - Choosing a collection: https://doc.rust-lang.org/std/collections/index.html
//   - HashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//   - VecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
//   - Why keys must be `Eq + Hash`: https://doc.rust-lang.org/std/hash/trait.Hash.html

// I AM NOT DONE

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct LruCache<K, V> {
    // TODO: choose your fields
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        todo!()
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    /// Returns the evicted entry, if inserting caused one.
    fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        todo!()
    }

    fn contains(&self, key: &K) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_get_put() {
        let mut cache = LruCache::new(2);
        assert_eq!(cache.put("a", 1), None);
        assert_eq!(cache.put("b", 2), None);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"z"), None);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn evicts_least_recently_used() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.put("c", 3), Some(("a", 1)));
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn get_counts_as_a_use() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.get(&"a"); // now "b" is the least recently used
        assert_eq!(cache.put("c", 3), Some(("b", 2)));
        assert!(cache.contains(&"a"));
        assert!(!cache.contains(&"b"));
    }

    #[test]
    fn contains_does_not_count_as_a_use() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert!(cache.contains(&"a"));
        assert_eq!(cache.put("c", 3), Some(("a", 1)));
    }

    #[test]
    fn updating_a_key_refreshes_it_without_evicting() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.put("a", 10), None);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.put("c", 3), Some(("b", 2)));
        assert_eq!(cache.get(&"a"), Some(&10));
    }

    #[test]
    fn capacity_one() {
        let mut cache = LruCache::new(1);
        cache.put(1, "one");
        assert_eq!(cache.put(2, "two"), Some((1, "one")));
        assert_eq!(cache.get(&2), Some(&"two"));
    }

    #[test]
    fn owned_keys() {
        let mut cache: LruCache<String, usize> = LruCache::new(3);
        for word in ["apple", "banana", "cherry", "date"] {
            cache.put(word.to_string(), word.len());
        }
        assert_eq!(cache.get(&"apple".to_string()), None);
        assert_eq!(cache.get(&"date".to_string()), Some(&4));
    }

    #[test]
    #[should_panic]
    fn zero_capacity_panics() {
        let _cache: LruCache<i32, i32> = LruCache::new(0);
    }
}
