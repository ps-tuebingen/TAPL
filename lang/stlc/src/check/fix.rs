use super::{errors::Error, TypingEnv};
use crate::{syntax::Fix, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
    use super::Fix;
    use crate::{syntax::Lambda, types::Type};
    use common::Typecheck;

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
