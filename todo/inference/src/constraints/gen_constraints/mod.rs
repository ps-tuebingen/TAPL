use super::{constraint::Constraint, TypingEnv};
use crate::types::{Type, TypeVar};
use common::Var;
use std::collections::HashSet;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod optional;
pub mod pair;
pub mod sum;
pub mod term;
pub mod unit;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct GenState {
    pub env: TypingEnv,
    pub constrs: Vec<Constraint>,
    pub used_vars: HashSet<TypeVar>,
}

impl GenState {
    pub fn add_constraint(&mut self, ty1: Type, ty2: Type) {
        self.constrs.push(Constraint {
            left: ty1,
            right: ty2,
        });
    }

    pub fn fresh_var(&mut self) -> TypeVar {
        let mut cnt = 0;
        while self
            .used_vars
            .contains(&("X".to_owned() + &cnt.to_string()))
        {
            cnt += 1
        }
        let fresh_var = "X".to_owned() + &cnt.to_string();
        self.used_vars.insert(fresh_var.clone());
        fresh_var
    }
}

pub trait GenConstraints {
    fn gen_constraints(self, st: &mut GenState) -> Type;
    fn gen_with(self, v: Var, ty: Type, st: &mut GenState) -> Type
    where
        Self: Sized,
    {
        let mut new_st = st.clone();
        new_st.constrs = vec![];
        new_st.env.insert(v, ty);
        let ty = self.gen_constraints(&mut new_st);
        st.constrs.extend(new_st.constrs);
        st.used_vars.extend(new_st.used_vars);
        ty
    }
}

impl GenConstraints for Var {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        match st.env.get(&self) {
            None => st.fresh_var().into(),
            Some(ty) => ty.clone(),
        }
    }
}

#[cfg(test)]
mod gen_constraints_tests {
    use super::{GenConstraints, GenState};
    use crate::types::Type;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn fresh_univar() {
        let result = GenState {
            env: HashMap::new(),
            constrs: vec![],
            used_vars: HashSet::from([
                "X0".to_owned(),
                "X1".to_owned(),
                "Y".to_owned(),
                "Z".to_owned(),
            ]),
        }
        .fresh_var();
        let expected = "X2";
        assert_eq!(result, expected)
    }

    #[test]
    fn gen_var() {
        let mut st = GenState::default();
        let result = "X".to_owned().gen_constraints(&mut st);
        let expected = Type::Var("X0".to_owned());
        assert_eq!(result, expected)
    }
}
