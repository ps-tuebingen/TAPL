use super::{to_err, types::Type};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Var,
};
use std::collections::HashMap;

pub mod ascribe;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tup;
pub mod unit;
pub mod variant;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

#[derive(Default, Clone)]
pub struct TypingEnv {
    pub used_vars: HashMap<Var, Type>,
}

#[cfg(test)]
mod var_tests {
    use super::TypingEnv;
    use crate::{syntax::Term, types::Type};
    use common::Typecheck;
    use std::collections::HashMap;

    #[test]
    fn check_var() {
        let result = Term::Var("x".to_owned())
            .check(&mut TypingEnv {
                used_vars: HashMap::from([("x".to_owned(), Type::Nat)]),
            })
            .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
