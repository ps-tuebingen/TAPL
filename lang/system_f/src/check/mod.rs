use crate::{
    syntax::Var,
    to_err,
    types::{TyVar, Type},
};
use common::errors::{Error, ErrorKind, ErrorLocation};
use std::collections::HashMap;

pub mod lambda;
pub mod terms;
pub mod tylambda;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

#[derive(Default, Clone)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: Vec<TyVar>,
}

#[cfg(test)]
mod check_tests {
    use crate::{
        examples::{double, id, quadruple, self_app},
        types::Type,
    };
    use common::Typecheck;

    #[test]
    fn check_id() {
        let result = id().check(&mut Default::default()).unwrap();
        let expected = Type::forall("X", Type::fun("X".into(), "X".into()));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_double() {
        let result = double().check(&mut Default::default()).unwrap();
        let expected = Type::forall(
            "X",
            Type::fun(
                Type::fun("X".into(), "X".into()),
                Type::fun("X".into(), "X".into()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_selfapp() {
        let result = self_app().check(&mut Default::default()).unwrap();
        let expected = Type::fun(
            Type::forall("X", Type::fun("X".into(), "X".into())),
            Type::forall("X", Type::fun("X".into(), "X".into())),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_quadruple() {
        let result = quadruple().check(&mut Default::default()).unwrap();
        let expected = Type::forall(
            "X",
            Type::fun(
                Type::fun("X".into(), "X".into()),
                Type::fun("X".into(), "X".into()),
            ),
        );
        assert_eq!(result, expected)
    }
}
