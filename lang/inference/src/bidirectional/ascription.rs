use super::{to_infer_err, Environment, Infer};
use crate::{syntax::Ascribe, types::Type};
use common::errors::{Error, ErrorKind};

impl Infer for Ascribe {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        self.term.check(self.ty.clone(), env)?;
        Ok(self.ty.clone())
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if target == self.ty {
            self.term.check(self.ty.clone(), env)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: target.to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod ascribe_tests {
    use super::{Ascribe, Infer};
    use crate::{syntax::True, types::Type};
    use std::collections::HashMap;

    #[test]
    fn infer_ascribe() {
        let result = Ascribe {
            term: Box::new(True.into()),
            ty: Type::Bool,
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_ascribe() {
        Ascribe {
            term: Box::new(True.into()),
            ty: Type::Bool,
        }
        .check(Type::Bool, &mut HashMap::new())
        .unwrap()
    }
}
