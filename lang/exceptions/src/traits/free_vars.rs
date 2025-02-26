use crate::syntax::{App, Error, Lambda, Raise, Term, Try, TryWithVal, Unit, Var};
use std::collections::HashSet;

pub trait FreeVars {
    fn free_vars(&self) -> HashSet<Var>;
    fn fresh_var(&self) -> Var {
        let used = self.free_vars();
        let mut fresh = 0;
        while used.contains(&format!("x{fresh}")) {
            fresh += 1;
        }
        format!("x{fresh}")
    }
}

impl FreeVars for Term {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            Term::Var(v) => HashSet::from([v.clone()]),
            Term::Lambda(lam) => lam.free_vars(),
            Term::App(app) => app.free_vars(),
            Term::Unit(u) => u.free_vars(),
            Term::Error(err) => err.free_vars(),
            Term::Try(t) => t.free_vars(),
            Term::Raise(r) => r.free_vars(),
            Term::TryWithVal(t) => t.free_vars(),
        }
    }
}

impl FreeVars for Lambda {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.body.free_vars();
        vars.remove(&self.var);
        vars
    }
}

impl FreeVars for App {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.fun.free_vars();
        vars.extend(self.arg.free_vars());
        vars
    }
}

impl FreeVars for Try {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.term.free_vars();
        vars.extend(self.handler.free_vars());
        vars
    }
}

impl FreeVars for Error {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}

impl FreeVars for Raise {
    fn free_vars(&self) -> HashSet<Var> {
        self.exception.free_vars()
    }
}

impl FreeVars for TryWithVal {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.term.free_vars();
        vars.extend(self.handler.free_vars());
        vars
    }
}

impl FreeVars for Unit {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}
