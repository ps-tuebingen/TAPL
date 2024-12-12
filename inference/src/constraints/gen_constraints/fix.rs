use super::{GenConstraints, GenState};
use crate::{syntax::Fix, types::Type};

impl GenConstraints for Fix {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let inner_ty = self.term.gen_constraints(st);
        let fresh_var = st.fresh_var();
        st.add_constraint(
            inner_ty,
            Type::Fun(
                Box::new(fresh_var.clone().into()),
                Box::new(fresh_var.clone().into()),
            ),
        );
        fresh_var.into()
    }
}

#[cfg(test)]
mod fix_tests {
    use super::{Fix, GenConstraints, GenState};
    use crate::{syntax::True, types::Type};

    #[test]
    fn gen_fix() {
        let mut st = GenState::default();
        let result = Fix {
            term: Box::new(True.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Var("X0".to_owned());
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st.add_constraint(
            Type::Bool,
            Type::Fun(
                Box::new(Type::Var("X0".to_owned())),
                Box::new(Type::Var("X0".to_owned())),
            ),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
