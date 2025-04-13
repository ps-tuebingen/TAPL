use super::FreeVars;
use crate::syntax::{Record, RecordProj};
use common::Var;
use std::collections::HashSet;

impl FreeVars for Record {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = HashSet::new();
        for (_, t) in self.records.iter() {
            vars.extend(t.free_vars())
        }
        vars
    }
}

impl FreeVars for RecordProj {
    fn free_vars(&self) -> HashSet<Var> {
        self.record.free_vars()
    }
}
