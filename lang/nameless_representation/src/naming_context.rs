use super::named_terms::Term as NamedTerm;
use common::Var;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct NamingContext {
    pub mappings: Vec<(Var, usize)>,
}

impl NamingContext {
    pub fn dom(&self) -> HashSet<Var> {
        self.mappings.iter().map(|(var, _)| var).cloned().collect()
    }

    pub fn max_ind(&self) -> Option<usize> {
        self.mappings.iter().map(|(_, ind)| ind).max().copied()
    }

    pub fn shift_up(&mut self) {
        let mut new_map = vec![];
        for (var, ind) in self.mappings.iter() {
            new_map.push((var.clone(), *ind + 1));
        }
        self.mappings = new_map;
    }

    pub fn add_var(&mut self, v: Var) {
        let mut new_vars = vec![(v.clone(), 0)];

        for (var, ind) in self.mappings.iter() {
            if *var == v {
                continue;
            }
            new_vars.push((var.clone(), (ind + 1)));
        }
        self.mappings = new_vars;
    }

    pub fn remove_var(&mut self, v: Var) {
        let ind = self
            .mappings
            .iter()
            .enumerate()
            .find(|(_, (var, _))| *var == v)
            .map(|(ind, _)| ind);
        match ind {
            None => (),
            Some(i) => {
                self.mappings.remove(i);
            }
        }
    }
}

impl From<Vec<(Var, usize)>> for NamingContext {
    fn from(map: Vec<(Var, usize)>) -> NamingContext {
        NamingContext { mappings: map }
    }
}

impl From<NamedTerm> for NamingContext {
    fn from(t: NamedTerm) -> NamingContext {
        match t {
            NamedTerm::Var(v) => vec![(v, 0)].into(),
            NamedTerm::Lambda(v, t) => {
                let mut inner: NamingContext = (*t).into();
                inner.remove_var(v);
                inner
            }
            NamedTerm::App(t1, t2) => {
                let mut ctx1: NamingContext = (*t1).into();
                let ctx1_vars = ctx1.dom();
                let offset = ctx1.max_ind().unwrap_or(0) + 1;
                let ctx2: NamingContext = (*t2).into();
                for (var, ind) in ctx2.mappings.iter() {
                    if !ctx1_vars.contains(var) {
                        ctx1.mappings.push((var.clone(), (*ind) + offset));
                    }
                }
                ctx1
            }
        }
    }
}
