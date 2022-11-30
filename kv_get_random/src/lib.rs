//  design a struct that has the following functions
//  1. insert a value (no duplicates)
//      in time O(1) | space O(n)
//  2. removing a value
//      in time O(1) | space O(1)
//  3. get_random a value that is already inserted (with equal probability)
//      in time O(1) | space O(1)
//  follow up questions:
//  1. accept duplicate values
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct KeyAlreadyExistsError;

pub struct KVStore {
    store: HashMap<i32, i32>,
    values: Vec<i32>,
}

impl KVStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            values: Vec::new(),
        }
    }

    // time O(1) | space O(n)
    pub fn put(&mut self, value: i32) -> Result<i32, KeyAlreadyExistsError> {
        if let Some(_) = self.store.get(&value) {
            return Err(KeyAlreadyExistsError);
        }
        self.values.push(value);
        let idx = self.values.len() - 1;
        match self.store.insert(value, idx as i32) {
            Some(_) => Err(KeyAlreadyExistsError),
            None => Ok(value),
        }
    }

    // time O(1) | space O(1)
    pub fn delete(&mut self, value: i32) -> bool {
        let last = match self.values.last() {
            Some(last) => last.clone(),
            None => return false,
        };
        let idx = match self.store.get_key_value(&value) {
            Some((_, v)) => v,
            None => return false,
        };
        let last_idx = self.values.len() - 1;
        self.values[last_idx as usize] = value;
        self.values[*idx as usize] = last;
        // TODO update the map=idx
        self.values.pop();
        match self.store.remove(&value) {
            Some(_) => true,
            None => false,
        }
    }

    // time O(1) | space O(1)
    pub fn get_random(&mut self) -> i32 {
        let mut rng = thread_rng();
        match self.values.choose(&mut rng) {
            Some(value) => *value,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_remove_getrandom() {
        let mut kv = KVStore::new();
        kv.put(10).unwrap();
        assert_eq!(kv.get_random(), 10);
        assert_eq!(kv.delete(10), true);
        assert_eq!(kv.delete(2), false);

        kv.put(11).unwrap();
        assert_eq!(kv.delete(11), true);
    }
}
