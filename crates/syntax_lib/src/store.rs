use crate::{Location, values::Value};
use common::errors::UndefinedLocation;
use std::collections::HashMap;

pub struct Store<V>
where
    V: Value,
{
    locs: HashMap<Location, V>,
}

impl<V> Store<V>
where
    V: Value,
{
    pub fn new() -> Store<V> {
        Store {
            locs: HashMap::new(),
        }
    }

    pub fn fresh_location(&self) -> Location {
        let mut new_loc = 0;
        while self.locs.contains_key(&new_loc) {
            new_loc += 1;
        }
        new_loc
    }

    pub fn save_location(&mut self, loc: Location, v: V) {
        self.locs.insert(loc, v);
    }

    pub fn get_location(&self, loc: Location) -> Result<V, UndefinedLocation> {
        self.locs
            .get(&loc)
            .cloned()
            .ok_or(UndefinedLocation::new(loc))
    }
}

impl<V> Default for Store<V>
where
    V: Value,
{
    fn default() -> Store<V> {
        Store::new()
    }
}
