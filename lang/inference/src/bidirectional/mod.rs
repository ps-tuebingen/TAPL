use crate::{types::Type, Var};
use std::collections::HashMap;

pub mod ascription;
pub mod bool;
pub mod errors;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod optional;
pub mod pair;
pub mod sum;
pub mod term;
pub mod unit;

use errors::Error;

type Environment = HashMap<Var, Type>;

pub trait Infer {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error>;
    fn infer_with(&self, v: Var, ty: Type, env: &Environment) -> Result<Type, Error> {
        let mut new_env = env.clone();
        new_env.insert(v, ty);
        self.infer(&mut new_env)
    }
    fn infer_local(&self, env: &Environment) -> Result<Type, Error> {
        let mut new_env = env.clone();
        self.infer(&mut new_env)
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error>;
    fn check_with(&self, v: Var, ty: Type, target: Type, env: &Environment) -> Result<(), Error> {
        let mut new_env = env.clone();
        new_env.insert(v, ty);
        self.check(target, &mut new_env)
    }
    fn check_local(&self, target: Type, env: &Environment) -> Result<(), Error> {
        let mut new_env = env.clone();
        self.check(target, &mut new_env)
    }
}

impl Infer for Var {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        env.get(self)
            .cloned()
            .ok_or(Error::FreeVariable { var: self.clone() })
    }

    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let ty_inferred = self.infer(env)?;
        if ty_inferred == target {
            Ok(())
        } else {
            Err(Error::BadTarget {
                ty: target,
                t: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod var_tests {
    use super::Infer;
    use crate::types::Type;
    use std::collections::HashMap;

    #[test]
    fn infer_var() {
        let result = "x"
            .to_owned()
            .infer(&mut HashMap::from([("x".to_owned(), Type::Bool)]))
            .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_var() {
        "x".to_owned()
            .check(
                Type::Bool,
                &mut HashMap::from([("x".to_owned(), Type::Bool)]),
            )
            .unwrap();
    }
}
