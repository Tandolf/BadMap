use std::{collections::hash_map::DefaultHasher, usize};
use std::hash::{
    Hash,
    Hasher,
};
use std::mem;

const INITIAL_N_BUCKETS: usize = 1;
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl <K, V> HashMap<K, V> 
where
    K: Hash + Eq
{
    fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {

        // If there are no bucket, or hashmap is 3/4 full call a resize before inserting.
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        // Create a default hasher
        let mut hasher = DefaultHasher::new();

        // Add the key to the hasher
        key.hash(&mut hasher);

        // Fetch the finished hash, then locate what bucket has room
        let bucket= (hasher.finish() % self.buckets.len() as u64) as usize;

        // Fetch that bucket
        let bucket = &mut self.buckets[bucket];

        // Loop through and deconstruct each tuple and check if the key exists alread, 
        // if so replace that value, and return the replaced value.
        for &mut (ref e_key, ref mut evalue) in bucket.iter_mut() {
            if e_key == &key {
                return Some(mem::replace(evalue, value))
            }
        }

        // If no key duplicate, just simply push they new value and return none.
        bucket.push((key, value));
        None
    }

    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_N_BUCKETS,
            n => 2 * n 
        };

        // Create a veck with our new size
        let mut new_buckets = Vec::with_capacity(target_size);

        // Iterate over target size and create a vec in each iteration 
        // and extend the new buckets with this new vec.
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut()
            .flat_map(|bucket| bucket.drain(..)) 
        {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket= (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        let _ = mem::replace(&mut self.buckets, new_buckets);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insert() {
        let mut map = HashMap::new();
        map.insert("foo", "42");
    }
}
