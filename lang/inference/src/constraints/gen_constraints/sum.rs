use super::{GenConstraints, GenState};
use crate::{
    syntax::{Left, Right, SumCase},
    types::Type,
};

impl GenConstraints for Left {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.left_term.gen_constraints(st);
        let fresh_var = st.fresh_var();
        Type::Sum(Box::new(inner_ty), Box::new(fresh_var.into()))
    }
}

impl GenConstraints for Right {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.right_term.gen_constraints(st);
        let fresh_var = st.fresh_var();
        Type::Sum(Box::new(fresh_var.into()), Box::new(inner_ty))
    }
}

impl GenConstraints for SumCase {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let bound_ty = self.bound_term.gen_constraints(st);
        let left_var = st.fresh_var();
        let right_var = st.fresh_var();
        st.add_constraint(
            bound_ty,
            Type::Sum(
                Box::new(left_var.clone().into()),
                Box::new(right_var.clone().into()),
            ),
        );

        let left_ty = self.left_term.gen_with(self.left_var, left_var.into(), st);
        let right_ty = self
            .right_term
            .gen_with(self.right_var, right_var.into(), st);
        st.add_constraint(left_ty.clone(), right_ty);
        left_ty
    }
}

#[cfg(test)]
mod sum_tests {
    use super::{GenConstraints, GenState, Left, Right, SumCase};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn gen_left() {
        let mut st = GenState::default();
        let result = Left {
            left_term: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Sum(Box::new(Type::Nat), Box::new("X0".to_owned().into()));
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_right() {
        let mut st = GenState::default();
        let result = Right {
            right_term: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Sum(Box::new("X0".to_owned().into()), Box::new(Type::Nat));
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_case() {
        let mut st = GenState::default();
        let result = SumCase {
            bound_term: Box::new(
                Left {
                    left_term: Box::new(Zero.into()),
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new("x".to_owned().into()),
            right_var: "y".to_owned(),
            right_term: Box::new("y".to_owned().into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Var("X1".to_owned());
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.used_vars.insert("X2".to_owned());
        new_st.add_constraint(
            Type::Sum(Box::new(Type::Nat), Box::new("X0".to_owned().into())),
            Type::Sum(
                Box::new("X1".to_owned().into()),
                Box::new("X2".to_owned().into()),
            ),
        );
        new_st.add_constraint("X1".to_owned().into(), "X2".to_owned().into());
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
