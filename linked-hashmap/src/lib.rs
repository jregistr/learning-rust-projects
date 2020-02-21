use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::{RandomState, DefaultHasher};

const INITIAL_BUCKETS_COUNT: usize = 1;

struct Bucket<K, V> {
    items: Vec<(K,V)>
}

impl <K,V> Clone for Bucket<K, V> {
    fn clone(&self) -> Self {
        let cloned_items = self.items.clone();
        return Bucket {items: cloned_items};
    }
}

pub struct LinkedHashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
    size: usize
}

impl <K,V> LinkedHashMap<K,V> where K: Eq + Hash {
    pub fn new() -> Self {
        LinkedHashMap {
            buckets: Vec::new(),
            size: 0
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.size >= 3 * (self.size / 4) {
            self.resize();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket_index = (hasher.finish() % self.buckets.len() as u64) as usize ;
        let bucket = &mut self.buckets[bucket_index];

        use std::mem;
        for &mut (ref bkey,ref mut bval) in bucket.items.iter_mut() {
            if *bkey == key {
                let old = mem::replace(bval, value);
                return Some(old);
            }
        }

        self.size +=1;
        bucket.items.push((key, value));
        return None;
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_BUCKETS_COUNT,
            n => 2 * n
        };

        let items_vec: Vec<(K,V)> = Vec::new();
        let new_buckets = vec![Bucket {items: items_vec}; target_size];
    }
}
