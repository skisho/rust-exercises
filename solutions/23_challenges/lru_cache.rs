// lru_cache.rs (challenge) - solution
//
// The HashMap stores the values; the VecDeque stores keys from least
// (front) to most (back) recently used. "Using" a key moves it to the
// back; eviction pops from the front.
//
// `get` checks `contains_key` before touching, because a `&V` borrowed
// from the map can't be held while `touch` mutably borrows `self`. The
// lookup happens again at the end.
//
// For the O(1) stretch goal, see how the `lru` crate does it:
// https://docs.rs/lru

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be at least 1");
        LruCache { capacity, map: HashMap::new(), order: VecDeque::new() }
    }

    /// Moves `key` to the most-recently-used end of the queue.
    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            let k = self.order.remove(pos).expect("position is in range");
            self.order.push_back(k);
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if !self.map.contains_key(key) {
            return None;
        }
        self.touch(key);
        self.map.get(key)
    }

    fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        if self.map.contains_key(&key) {
            self.touch(&key);
            self.map.insert(key, value);
            return None;
        }

        let evicted = if self.map.len() == self.capacity {
            let oldest = self.order.pop_front().expect("cache is full, so not empty");
            let value = self.map.remove(&oldest).expect("queue and map agree");
            Some((oldest, value))
        } else {
            None
        };

        self.order.push_back(key.clone());
        self.map.insert(key, value);
        evicted
    }

    fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    fn len(&self) -> usize {
        self.map.len()
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
