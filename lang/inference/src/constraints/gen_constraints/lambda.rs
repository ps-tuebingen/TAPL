use super::{GenConstraints, GenState};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};

impl GenConstraints for Lambda {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let fresh_var = st.fresh_var();
        st.env.insert(self.var, fresh_var.clone().into());
        let body_ty = self.body.gen_constraints(st);
        Type::Fun(Box::new(fresh_var.into()), Box::new(body_ty))
    }
}

impl GenConstraints for App {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        let arg_ty = self.arg.gen_constraints(st);
        let fresh_var = st.fresh_var();
        let fun_ty = self.fun.gen_constraints(st);
        st.add_constraint(
            fun_ty,
            Type::Fun(Box::new(arg_ty), Box::new(fresh_var.clone().into())),
        );
        fresh_var.into()
    }
}

#[cfg(test)]
mod lam_tests {
    use super::{App, GenConstraints, GenState, Lambda};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn gen_lam() {
        let mut st = GenState::default();
        let result = Lambda {
            var: "x".to_owned(),
            body: Box::new("x".to_owned().into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Fun(
            Box::new(Type::Var("X0".to_owned())),
            Box::new(Type::Var("X0".to_owned())),
        );
        let mut new_st = GenState::default();
        new_st.used_vars.insert("X0".to_owned());
        new_st
            .env
            .insert("x".to_owned(), Type::Var("X0".to_owned()));
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }

    #[test]
    fn gen_app() {
        let mut st = GenState::default();
        let result = App {
            fun: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            arg: Box::new(Zero.into()),
        }
        .gen_constraints(&mut st);
        let expected = Type::Var("X0".to_owned());
        let mut new_st = GenState::default();
        new_st
            .env
            .insert("x".to_owned(), Type::Var("X1".to_owned()));
        new_st.used_vars.insert("X0".to_owned());
        new_st.used_vars.insert("X1".to_owned());
        new_st.add_constraint(
            Type::Fun(
                Box::new("X1".to_owned().into()),
                Box::new("X1".to_owned().into()),
            ),
            Type::Fun(
                Box::new(Type::Nat),
                Box::new(Type::Var("X0".to_owned())),
            ),
        );
        assert_eq!(result, expected);
        assert_eq!(st, new_st)
    }
}
