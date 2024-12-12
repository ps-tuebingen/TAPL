use super::{types::Type, Var};
use std::collections::HashMap;

pub mod ascribe;
pub mod bool;
pub mod errors;
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

use errors::Error;

#[derive(Default)]
pub struct TypingEnv {
    pub used_vars: HashMap<Var, Type>,
}

pub trait Check {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error>;
    fn check_local(&self, env: &TypingEnv) -> Result<Type, Error> {
        let mut new_env = TypingEnv {
            used_vars: env.used_vars.clone(),
        };
        self.check(&mut new_env)
    }
}

impl Check for Var {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        env.used_vars
            .get(self)
            .cloned()
            .ok_or(Error::UnboundVariable { var: self.clone() })
    }
}

#[cfg(test)]
mod var_tests {
    use super::{Check, TypingEnv};
    use crate::types::Type;
    use std::collections::HashMap;

    #[test]
    fn check_var() {
        let result = "x"
            .to_owned()
            .check(&mut TypingEnv {
                used_vars: HashMap::from([("x".to_owned(), Type::Nat)]),
            })
            .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
