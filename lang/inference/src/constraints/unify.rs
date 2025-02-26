use super::{
    constraint::{subst_constrs, Constraint},
    errors::Error,
};
use crate::{
    types::{Type, TypeVar},
    Var,
};
use std::collections::{HashMap, VecDeque};

pub type Unifier = HashMap<TypeVar, Type>;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UnifyState {
    pub remaining_constraints: VecDeque<Constraint>,
    pub unifier: Unifier,
    pub equal_vars: Vec<(Var, Var)>,
}

impl UnifyState {
    fn add_constraint(&mut self, ty1: Type, ty2: Type) {
        self.remaining_constraints.push_back(Constraint {
            left: ty1,
            right: ty2,
        })
    }
}

pub fn unify(constraints: Vec<Constraint>) -> Result<Unifier, Error> {
    let mut state = UnifyState::from(constraints);
    while let Some(constr) = state.remaining_constraints.pop_front() {
        unify_equality_constraint(constr.left, constr.right, &mut state)?;
    }
    for (v1, v2) in state.equal_vars.iter() {
        let ty = state
            .unifier
            .get(v1)
            .cloned()
            .unwrap_or(Type::Var(v1.clone()));
        state.unifier.insert(v2.clone(), ty);
    }
    Ok(state.unifier)
}

fn unify_equality_constraint(ty1: Type, ty2: Type, state: &mut UnifyState) -> Result<(), Error> {
    match (ty1, ty2) {
        (Type::Var(v1), Type::Var(v2)) => match (state.unifier.get(&v1), state.unifier.get(&v2)) {
            (Some(ty1), Some(ty2)) => {
                state.add_constraint(ty1.clone(), ty2.clone());
                Ok(())
            }
            (Some(ty1), None) => {
                state.unifier.insert(v2, ty1.clone());
                Ok(())
            }
            (None, Some(ty2)) => {
                state.unifier.insert(v1, ty2.clone());
                Ok(())
            }
            (None, None) => {
                subst_constrs(&mut state.remaining_constraints, &v2, Type::Var(v1.clone()));
                state.equal_vars.push((v1, v2));
                Ok(())
            }
        },
        (Type::Var(v), ty) => {
            match state.unifier.get(&v) {
                None => {
                    state.unifier.insert(v, ty);
                }
                Some(ty2) => state.add_constraint(ty, ty2.clone()),
            }
            Ok(())
        }
        (ty, Type::Var(v2)) => unify_equality_constraint(Type::Var(v2), ty, state),
        (Type::Unit, Type::Unit) => Ok(()),
        (Type::Bool, Type::Bool) => Ok(()),
        (Type::Nat, Type::Nat) => Ok(()),
        (Type::Fun(ty11, ty12), Type::Fun(ty21, ty22)) => {
            state.add_constraint(*ty11, *ty21);
            state.add_constraint(*ty12, *ty22);
            Ok(())
        }
        (Type::Prod(ty11, ty12), Type::Prod(ty21, ty22)) => {
            state.add_constraint(*ty11, *ty21);
            state.add_constraint(*ty12, *ty22);
            Ok(())
        }
        (Type::Sum(ty11, ty12), Type::Sum(ty21, ty22)) => {
            state.add_constraint(*ty11, *ty21);
            state.add_constraint(*ty12, *ty22);
            Ok(())
        }
        (Type::Optional(ty1), Type::Optional(ty2)) => {
            state.add_constraint(*ty1, *ty2);
            Ok(())
        }
        (Type::List(ty1), Type::List(ty2)) => {
            state.add_constraint(*ty1, *ty2);
            Ok(())
        }
        (ty1, ty2) => Err(Error::CannotUnify { ty1, ty2 }),
    }
}

impl From<Vec<Constraint>> for UnifyState {
    fn from(ctrs: Vec<Constraint>) -> UnifyState {
        UnifyState {
            remaining_constraints: ctrs.into(),
            unifier: HashMap::new(),
            equal_vars: vec![],
        }
    }
}

#[cfg(test)]
mod unify_tests {
    use super::{unify_equality_constraint, UnifyState};
    use crate::types::Type;

    #[test]
    fn unify_unit_unit() {
        let mut st = UnifyState::default();
        unify_equality_constraint(Type::Unit, Type::Unit, &mut st).unwrap();
        assert_eq!(st, UnifyState::default())
    }

    #[test]
    fn unify_bool_bool() {
        let mut st = UnifyState::default();
        unify_equality_constraint(Type::Bool, Type::Bool, &mut st).unwrap();
        assert_eq!(st, UnifyState::default())
    }

    #[test]
    fn unify_nat_nat() {
        let mut st = UnifyState::default();
        unify_equality_constraint(Type::Nat, Type::Nat, &mut st).unwrap();
        assert_eq!(st, UnifyState::default())
    }

    #[test]
    fn unify_fun_fun() {
        let mut st = UnifyState::default();
        unify_equality_constraint(
            Type::Fun(Box::new(Type::Nat), Box::new(Type::Nat)),
            Type::Fun(Box::new(Type::Nat), Box::new(Type::Nat)),
            &mut st,
        )
        .unwrap();
        let mut new_st = UnifyState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(st, new_st)
    }

    #[test]
    fn unify_prod_prod() {
        let mut st = UnifyState::default();
        unify_equality_constraint(
            Type::Prod(Box::new(Type::Bool), Box::new(Type::Nat)),
            Type::Prod(Box::new(Type::Bool), Box::new(Type::Nat)),
            &mut st,
        )
        .unwrap();
        let mut new_st = UnifyState::default();
        new_st.add_constraint(Type::Bool, Type::Bool);
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(st, new_st)
    }

    #[test]
    fn unify_sum_sum() {
        let mut st = UnifyState::default();
        unify_equality_constraint(
            Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
            Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
            &mut st,
        )
        .unwrap();
        let mut new_st = UnifyState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        new_st.add_constraint(Type::Bool, Type::Bool);
        assert_eq!(st, new_st)
    }

    #[test]
    fn unify_option_option() {
        let mut st = UnifyState::default();
        unify_equality_constraint(
            Type::Optional(Box::new(Type::Nat)),
            Type::Optional(Box::new(Type::Nat)),
            &mut st,
        )
        .unwrap();
        let mut new_st = UnifyState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(st, new_st)
    }

    #[test]
    fn unify_list_list() {
        let mut st = UnifyState::default();
        unify_equality_constraint(
            Type::List(Box::new(Type::Nat)),
            Type::List(Box::new(Type::Nat)),
            &mut st,
        )
        .unwrap();
        let mut new_st = UnifyState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(st, new_st)
    }

    #[test]
    fn unify_other() {
        let result = unify_equality_constraint(Type::Nat, Type::Bool, &mut UnifyState::default());
        assert!(result.is_err())
    }

    #[test]
    fn unify_var_ty() {
        let mut st = UnifyState::default();
        unify_equality_constraint(Type::Var("X0".to_owned()), Type::Bool, &mut st).unwrap();
        let mut new_st = UnifyState::default();
        new_st.unifier.insert("X0".to_owned(), Type::Bool);
        assert_eq!(st, new_st)
    }

    #[test]
    fn unify_var_var1() {
        let mut st = UnifyState::default();
        st.add_constraint(
            Type::List(Box::new("X0".to_owned().into())),
            Type::List(Box::new(Type::Nat)),
        );
        unify_equality_constraint("X1".to_owned().into(), "X0".to_owned().into(), &mut st).unwrap();
        let mut new_st = UnifyState::default();
        new_st.add_constraint(
            Type::List(Box::new("X1".to_owned().into())),
            Type::List(Box::new(Type::Nat)),
        );
        new_st.equal_vars.push(("X1".to_owned(), "X0".to_owned()));
        assert_eq!(st, new_st)
    }

    #[test]
    fn unift_var_var2() {
        let mut st = UnifyState::default();
        st.unifier.insert("X0".to_owned(), Type::Nat);
        unify_equality_constraint("X0".to_owned().into(), "X1".to_owned().into(), &mut st).unwrap();
        let mut new_st = UnifyState::default();
        new_st.unifier.insert("X0".to_owned(), Type::Nat);
        new_st.unifier.insert("X1".to_owned(), Type::Nat);
        assert_eq!(st, new_st)
    }
}
