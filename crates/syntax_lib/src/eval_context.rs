use crate::{program::Program, terms::Term, types::Type, values::Value, Location, Name};
use common::errors::{FreeVariable, UndefinedLocation};
use std::collections::HashMap;

pub struct EvalContext<T, V>
where
    V: Value,
    T: Term,
{
    locs: HashMap<Location, V>,
    defs: HashMap<Name, T>,
}

impl<V, T> EvalContext<T, V>
where
    V: Value,
    T: Term,
{
    pub fn new() -> EvalContext<T, V> {
        EvalContext {
            locs: HashMap::new(),
            defs: HashMap::new(),
        }
    }

    pub fn from_prog<Ty>(prog: &Program<T, Ty>) -> EvalContext<T, V>
    where
        Ty: Type,
    {
        EvalContext {
            locs: HashMap::new(),
            defs: prog
                .definitions
                .iter()
                .map(|def| (def.name.clone(), def.body.clone()))
                .collect(),
        }
    }

    pub fn fresh_location(&self) -> Location {
        let mut new_loc = 0;
        while self.locs.contains_key(&new_loc) {
            new_loc += 1;
        }
        new_loc
    }

    pub fn save_name(&mut self, n: Name, t: T) {
        self.defs.insert(n, t);
    }

    pub fn get_name(&self, n: &Name) -> Result<T, FreeVariable> {
        self.defs.get(n).cloned().ok_or(FreeVariable::new(n))
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

impl<T, V> Default for EvalContext<T, V>
where
    T: Term,
    V: Value,
{
    fn default() -> EvalContext<T, V> {
        EvalContext::new()
    }
}
