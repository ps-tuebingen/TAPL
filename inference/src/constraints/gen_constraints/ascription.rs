use super::{GenConstraints, GenState};
use crate::{syntax::Ascribe, types::Type};

impl GenConstraints for Ascribe {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let term_ty = self.term.gen_constraints(st);
        st.add_constraint(self.ty, term_ty.clone());
        term_ty
    }
}

#[cfg(test)]
mod ascription_tests {
    use super::{Ascribe, GenConstraints, GenState};
    use crate::{syntax::True, types::Type};

    #[test]
    fn gen_ascription() {
        let mut st = GenState::default();
        let result = Ascribe {
            ty: Type::Bool,
            term: Box::new(True.into()),
        }
        .gen_constraints(&mut st);
        let mut new_st = GenState::default();
        new_st.add_constraint(Type::Bool, Type::Bool);
        let expected = Type::Bool;
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
