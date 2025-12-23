use crate::constraints::Constraint;
use std::collections::{HashMap, VecDeque};
use syntax::{TypeVar, language::Language};

pub struct SolveState<Lang>
where
    Lang: Language,
{
    remaining_constraints: VecDeque<Constraint<Lang>>,
    pub var_tys: HashMap<TypeVar, Lang::Type>,
    pub tyvar_super: HashMap<TypeVar, Lang::Type>,
}

impl<Lang> SolveState<Lang>
where
    Lang: Language,
{
    pub fn new(constraints: Vec<Constraint<Lang>>) -> Self {
        Self {
            remaining_constraints: constraints.into_iter().collect(),
            var_tys: HashMap::new(),
            tyvar_super: HashMap::new(),
        }
    }

    pub fn add_tyvar<Ty>(&mut self, var: &str, super_ty: Ty)
    where
        Ty: Into<Lang::Type>,
    {
        self.tyvar_super.insert(var.to_string(), super_ty.into());
    }

    pub fn add_constraint<C>(&mut self, c: C)
    where
        C: Into<Constraint<Lang>>,
    {
        self.remaining_constraints.push_back(c.into());
    }

    pub fn next_constraint(&mut self) -> Option<Constraint<Lang>> {
        self.remaining_constraints.pop_front()
    }
}
