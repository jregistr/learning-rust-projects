use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::{RandomState, DefaultHasher};

const INITIAL_BUCKETS_COUNT: usize = 1;

struct Bucket<K, V> {
    items: Vec<(K,V)>
}

pub struct LinkedHashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

impl <K,V> LinkedHashMap<K,V> where K: Eq + Hash {
    pub fn new() -> Self {
        LinkedHashMap {
            buckets: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> V {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = hasher.finish() % self.buckets.len() as u64;
        let bucket = &mut self.buckets[bucket];

        unimplemented!()
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS_COUNT,
            n => 2 * n
        };

        // TODO
    }
}
