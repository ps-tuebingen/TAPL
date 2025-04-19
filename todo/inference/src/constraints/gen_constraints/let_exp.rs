use super::{GenConstraints, GenState};
use crate::{syntax::Let, types::Type};

impl GenConstraints for Let {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let bound_ty = self.bound_term.gen_constraints(st);
        st.env.insert(self.var, bound_ty);
        self.in_term.gen_constraints(st)
    }
}

#[cfg(test)]
mod let_tests {
    use super::{GenConstraints, GenState, Let};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn gen_let() {
        let mut st = GenState::default();
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(Zero.into()),
            in_term: Box::new("x".to_owned().into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Nat;
        let mut new_st = GenState::default();
        new_st.env.insert("x".to_owned(), Type::Nat);
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
