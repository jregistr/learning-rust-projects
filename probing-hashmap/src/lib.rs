use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::{RandomState, DefaultHasher};
use std::ptr::null;

const INITIAL_BUCKETS_COUNT: usize = 1;

pub struct LinkedHashMap<K, V, S = RandomState> {
    keys: Vec<K>,
    values: Vec<V>,
    size: usize
}

impl <K,V> LinkedHashMap<K, V> where K : Eq + Hash {
    pub fn new() -> Self {
        LinkedHashMap {
            keys: Vec::new(),
            values: Vec::new(),
            size: 0
        }
    }

    fn resize(&mut self) {
        let target_size = match self.keys.len() {
            0 => INITIAL_BUCKETS_COUNT,
            n => 2 * n
        };


    }

    fn mk_base_hash_index(new_key: K, keys_bucket_size: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        new_key.hash(&mut hasher);
        let has_value = hasher.finish();
        (has_value % (keys_bucket_size as u64)) as usize
    }

    fn probe_for_insert_index(initial_hash_index: usize, keys: Vec<K>) -> Option<usize> {
        let a = keys[initial_hash_index]
        // if keys[initial_hash_index] != Nil {
        //
        // }
    }

    // fn probe_for_empty_index(initial_hash_index: usize, keys: Vec<K>) -> Option<usize> {
    //     if let
    //     None
    // }
}

// impl <K,V> LinkedHashMap<K,V> where K: Eq + Hash {
//     pub fn new() -> Self {
//         LinkedHashMap {
//             buckets: Vec::new(),
//             size: 0
//         }
//     }
//
//     pub fn insert(&mut self, key: K, value: V) -> Option<V> {
//         if self.buckets.is_empty() || self.size >= 3 * (self.size / 4) {
//             self.resize();
//         }
//
//         let mut hasher = DefaultHasher::new();
//         key.hash(&mut hasher);
//         let bucket_index = (hasher.finish() % self.buckets.len() as u64) as usize ;
//         let bucket = &mut self.buckets[bucket_index];
//
//         use std::mem;
//         for &mut (ref bkey,ref mut bval) in bucket.items.iter_mut() {
//             if *bkey == key {
//                 let old = mem::replace(bval, value);
//                 return Some(old);
//             }
//         }
//
//         self.size +=1;
//         bucket.items.push((key, value));
//         return None;
//     }
//
//     fn resize(&mut self) {
//         let target_size = match self.buckets.len() {
//             0 => INITIAL_BUCKETS_COUNT,
//             n => 2 * n
//         };
//
//         let items_vec: Vec<(K,V)> = Vec::new();
//         let mut new_buckets = vec![Bucket {items: items_vec}; target_size];
//
//     }
// }
