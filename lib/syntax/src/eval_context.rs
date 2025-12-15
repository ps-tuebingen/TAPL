use crate::{Location, Name, language::Language, program::Program};
use std::collections::HashMap;

/// Context during evaluation
pub struct EvalContext<Lang>
where
    Lang: Language,
{
    /// Stored locations (for languages with references)
    locs: HashMap<Location, Lang::Value>,
    /// Definitions in the program
    defs: HashMap<Name, Lang::Term>,
}

impl<Lang> EvalContext<Lang>
where
    Lang: Language,
{
    /// Create a new empty context
    #[must_use]
    pub fn new() -> Self {
        Self {
            locs: HashMap::new(),
            defs: HashMap::new(),
        }
    }

    /// Create a context from a program
    /// copies all definitions
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

    /// Create a fresh location not in the environment yet
    /// This does not add the new location to the environment
    #[must_use]
    pub fn fresh_location(&self) -> Location {
        let mut new_loc = 0;
        while self.locs.contains_key(&new_loc) {
            new_loc += 1;
        }
        new_loc
    }

    /// Insert a new definition to `self`
    pub fn save_name(&mut self, n: Name, t: Lang::Term) {
        self.defs.insert(n, t);
    }

    /// Look up a definition in the context
    #[must_use]
    pub fn get_name(&self, n: &Name) -> Option<Lang::Term> {
        self.defs.get(n).cloned()
    }

    /// Insert a new location into `self`
    pub fn save_location(&mut self, loc: Location, v: Lang::Value) {
        self.locs.insert(loc, v);
    }

    /// Look up a location in the store
    #[must_use]
    pub fn get_location(&self, loc: Location) -> Option<Lang::Value> {
        self.locs.get(&loc).cloned()
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
