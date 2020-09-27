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

        assert_eq!(*map.get("test").unwrap(), 1);
    }

    #[test]
    fn test_occupied() {
        let mut map = HashMap::<String, i64>::new();
        map.insert("test".into(), 100);
        create_or_increment(&mut map, "test".into());

        assert_eq!(*map.get("test").unwrap(), 101);
    }
}
