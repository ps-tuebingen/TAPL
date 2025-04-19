use super::{Environment, Infer};
use crate::{syntax::Let, types::Type};
use common::errors::Error;

impl Infer for Let {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let bound_ty = self.bound_term.infer_local(env)?;
        env.insert(self.var.clone(), bound_ty);
        self.in_term.infer(env)
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let bound_ty = self.bound_term.infer_local(env)?;
        env.insert(self.var.clone(), bound_ty);
        self.in_term.check(target, env)
    }
}

#[cfg(test)]
mod let_tests {
    use super::{Infer, Let};
    use crate::{syntax::Zero, types::Type};
    use std::collections::HashMap;

    #[test]
    fn infer_let() {
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(Zero.into()),
            in_term: Box::new("x".to_owned().into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_let() {
        Let {
            var: "x".to_owned(),
            bound_term: Box::new(Zero.into()),
            in_term: Box::new("x".to_owned().into()),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }
}
