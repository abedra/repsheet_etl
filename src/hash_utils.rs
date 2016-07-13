use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

pub fn create_or_increment(hash: &mut HashMap<String, i64>, key: String) {
    match hash.entry(key) {
        Vacant(e) => { e.insert(1); },
        Occupied(mut e) => { *e.get_mut() += 1 },
    }
}
