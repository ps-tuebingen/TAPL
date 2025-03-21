use super::{errors::Error, Check, TypingEnv};
use crate::{syntax::Fix, types::Type};

impl Check for Fix {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty = self.term.check(env)?;
        if let Type::Fun(ty1, ty2) = ty {
            if *ty1 == *ty2 {
                Ok(*ty1)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![*ty1, *ty2],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty,
                term: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod fix_tests {
    use super::{Check, Fix};
    use crate::{syntax::Lambda, types::Type};

    #[test]
    fn check_fix() {
        let result = Fix {
            term: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    annot: Type::Nat,
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
