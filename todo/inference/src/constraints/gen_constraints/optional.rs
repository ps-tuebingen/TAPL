use super::{GenConstraints, GenState};
use crate::{
    syntax::{Nothing, SomeCase, Something},
    types::Type,
};

impl GenConstraints for Nothing {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let fresh_var = st.fresh_var();
        Type::Optional(Box::new(fresh_var.into()))
    }
}

impl GenConstraints for Something {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.term.gen_constraints(st);
        Type::Optional(Box::new(inner_ty))
    }
}

impl GenConstraints for SomeCase {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let bound_ty = self.bound_term.gen_constraints(st);
        let fresh_var = st.fresh_var();
        st.add_constraint(bound_ty, Type::Optional(Box::new(fresh_var.clone().into())));
        let none_ty = self.none_rhs.gen_constraints(st);
        let some_ty = self
            .some_rhs
            .gen_with(self.some_var, fresh_var.clone().into(), st);
        st.add_constraint(none_ty.clone(), some_ty);
        none_ty
    }
}

#[cfg(test)]
mod option_tests {
    use super::{GenConstraints, GenState, Nothing, SomeCase, Something};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn gen_nothing() {
        let mut st = GenState::default();
        let result = Nothing.gen_constraints(&mut st);
        let expected = Type::Optional(Box::new("X0".to_owned().into()));
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_something() {
        let mut st = GenState::default();
        let result = Something {
            term: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Optional(Box::new(Type::Nat));
        assert_eq!(result, expected);
        assert_eq!(st, GenState::default())
    }

    #[test]
    fn gen_case() {
        let mut st = GenState::default();
        let result = SomeCase {
            bound_term: Box::new(
                Something {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            none_rhs: Box::new(Zero.into()),
            some_var: "x".to_owned(),
            some_rhs: Box::new("x".to_owned().into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Nat;
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.add_constraint(
            Type::Optional(Box::new(Type::Nat)),
            Type::Optional(Box::new("X0".to_owned().into())),
        );
        new_st.add_constraint(Type::Nat, "X0".to_owned().into());
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
