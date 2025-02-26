use super::{errors::Error, Environment, Infer};
use crate::{syntax::Fix, types::Type};

impl Infer for Fix {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let inner_ty = self.term.infer(env)?;
        if let Type::Fun(ty1, ty2) = inner_ty {
            if *ty1 == *ty2 {
                Ok(*ty1)
            } else {
                Err(Error::TypeMismatch {
                    ty1: *ty1,
                    ty2: *ty2,
                })
            }
        } else {
            Err(Error::TypeMismatch {
                ty1: inner_ty,
                ty2: Type::Fun(
                    Box::new("X".to_owned().into()),
                    Box::new("Y".to_owned().into()),
                ),
            })
        }
    }

    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        self.term
            .check(Type::Fun(Box::new(target.clone()), Box::new(target)), env)
    }
}

#[cfg(test)]
mod fix_tests {
    use super::{Fix, Infer};
    use crate::{
        syntax::{Ascribe, Lambda},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn infer_fix() {
        let result = Fix {
            term: Box::new(
                Ascribe {
                    term: Box::new(
                        Lambda {
                            var: "x".to_owned(),
                            body: Box::new("x".to_owned().into()),
                        }
                        .into(),
                    ),
                    ty: Type::Fun(Box::new(Type::Nat), Box::new(Type::Nat)),
                }
                .into(),
            ),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fix() {
        Fix {
            term: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap();
    }
}
