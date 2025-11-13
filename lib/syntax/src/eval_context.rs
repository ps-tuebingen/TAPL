use crate::{Location, Name, language::Language, program::Program};
use errors::{FreeVariable, UndefinedLocation};
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
    /// # Errors
    /// Returns an error if the name was not found
    pub fn get_name(&self, n: &Name) -> Result<Lang::Term, FreeVariable> {
        self.defs
            .get(n)
            .cloned()
            .ok_or_else(|| FreeVariable::new(n))
    }

    /// Insert a new location into `self`
    pub fn save_location(&mut self, loc: Location, v: Lang::Value) {
        self.locs.insert(loc, v);
    }

    /// Look up a location in the store
    /// # Errors
    /// Returns an error if the location was not present
    pub fn get_location(&self, loc: Location) -> Result<Lang::Value, UndefinedLocation> {
        self.locs
            .get(&loc)
            .cloned()
            .ok_or_else(|| UndefinedLocation::new(loc))
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
