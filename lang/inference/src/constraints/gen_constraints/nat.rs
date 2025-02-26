use super::{GenConstraints, GenState};
use crate::{
    syntax::{IsZero, Pred, Succ, Zero},
    types::Type,
};

impl GenConstraints for Zero {
    fn gen_constraints(self, _: &mut GenState) -> Type {
        Type::Nat
    }
}

impl GenConstraints for Succ {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.term.gen_constraints(st);
        st.add_constraint(inner_ty, Type::Nat);
        Type::Nat
    }
}

impl GenConstraints for Pred {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.term.gen_constraints(st);
        st.add_constraint(inner_ty, Type::Nat);
        Type::Nat
    }
}

impl GenConstraints for IsZero {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.term.gen_constraints(st);
        st.add_constraint(inner_ty, Type::Nat);
        Type::Bool
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{GenConstraints, GenState, IsZero, Pred, Succ, Zero};
    use crate::types::Type;

    #[test]
    fn gen_zero() {
        let mut st = GenState::default();
        let result = Zero.gen_constraints(&mut st);
        let expected = Type::Nat;
        assert_eq!(result, expected);
        assert_eq!(st, GenState::default())
    }

    #[test]
    fn gen_succ() {
        let mut st = GenState::default();
        let result = Succ {
            term: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Nat;
        let mut new_st = GenState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_pred() {
        let mut st = GenState::default();
        let result = Pred {
            term: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Nat;
        let mut new_st = GenState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_iszero() {
        let mut st = GenState::default();
        let result = IsZero {
            term: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Bool;
        let mut new_st = GenState::default();
        new_st.add_constraint(Type::Nat, Type::Nat);
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
