use crate::{Location, Name, language::Language, program::Program};
use errors::{FreeVariable, UndefinedLocation};
use std::collections::HashMap;

pub struct EvalContext<Lang>
where
    Lang: Language,
{
    locs: HashMap<Location, Lang::Value>,
    defs: HashMap<Name, Lang::Term>,
}

impl<Lang> EvalContext<Lang>
where
    Lang: Language,
{
    #[must_use] pub fn new() -> Self {
        Self {
            locs: HashMap::new(),
            defs: HashMap::new(),
        }
    }

    pub fn from_prog(prog: &Program<Lang>) -> Self
    where
        Lang: Language,
    {
        Self {
            locs: HashMap::new(),
            defs: prog
                .definitions
                .iter()
                .map(|def| (def.name.clone(), def.body.clone()))
                .collect(),
        }
    }

    #[must_use] pub fn fresh_location(&self) -> Location {
        let mut new_loc = 0;
        while self.locs.contains_key(&new_loc) {
            new_loc += 1;
        }
        new_loc
    }

    pub fn save_name(&mut self, n: Name, t: Lang::Term) {
        self.defs.insert(n, t);
    }

    pub fn get_name(&self, n: &Name) -> Result<Lang::Term, FreeVariable> {
        self.defs.get(n).cloned().ok_or(FreeVariable::new(n))
    }

    pub fn save_location(&mut self, loc: Location, v: Lang::Value) {
        self.locs.insert(loc, v);
    }

    pub fn get_location(&self, loc: Location) -> Result<Lang::Value, UndefinedLocation> {
        self.locs
            .get(&loc)
            .cloned()
            .ok_or(UndefinedLocation::new(loc))
    }
}

impl<Lang> Default for EvalContext<Lang>
where
    Lang: Language,
{
    fn default() -> Self {
        Self::new()
    }
}
