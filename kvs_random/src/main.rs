use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

type Key = i32;
type Value = i32;

struct KVStore {
    store: HashMap<Key, Value>,
    values: Vec<i32>,
}

impl KVStore {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
            values: Vec::new(),
        }
    }

    // time O(1) | space O(n)
    fn put(&mut self, value: &i32) {
        match self.store.get(value) {
            Some(_) => (),
            None => {
                self.values.push(*value);
                let idx = self.values.len() - 1;
                self.store.insert(*value, idx as i32);
            }
        }
    }

    // time O(1) | space O(1)
    fn delete(&mut self, value: &i32) -> bool {
        if self.values.is_empty() {
            return false;
        }
        let last = self.values.len() - 1;
        let idx = match self.store.get_key_value(value) {
            Some((_, v)) => *v,
            None => -1,
        };
        self.values[last] = *value;
        self.values[idx as usize] = last as i32;
        self.values.pop();
        match self.store.remove(value) {
            Some(_) => true,
            None => false,
        }
    }
    
    // time O(1) | space O(1)
    fn get_random(&mut self) -> &i32 {
        let mut rng = thread_rng();
        self.values.choose(&mut rng).unwrap()
    }
}

// design a struct that has the following functions
// 1. insert a value (no duplicates)
// 2. removing a value
// 3. get_random a value that is already inserted (with equal probability)
// follow up questions:
// 1. accept duplicate values
fn main() {
    let mut kvs = KVStore::new();
    kvs.put(&10);
    assert_eq!(kvs.get_random(), &10);
    assert_eq!(kvs.delete(&10), true);
    assert_eq!(kvs.delete(&2), false);

    kvs.put(&11);
    assert_eq!(kvs.delete(&11), true);
}
