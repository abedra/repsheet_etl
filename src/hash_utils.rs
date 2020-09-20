use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::hash;

pub fn create_or_increment<A: Eq + hash::Hash>(hash: &mut HashMap<A, i64>, key: A) {
    match hash.entry(key) {
        Vacant(e) => { e.insert(1); },
        Occupied(mut e) => { *e.get_mut() += 1 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacant() {
        let mut map = HashMap::<String, i64>::new();
        create_or_increment(&mut map, "test".into());

        match map.get("test") {
            Some(&value) => assert_eq!(value, 1),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_occupied() {
        let mut map = HashMap::<String, i64>::new();
        map.insert("test".into(), 100);
        create_or_increment(&mut map, "test".into());

        match map.get("test") {
            Some(&value) => assert_eq!(value, 101),
            _ => assert!(false)
        }
    }
}
