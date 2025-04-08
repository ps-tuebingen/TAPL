use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{Nothing, SomeCase, Something},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Nothing {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Error> {
        Ok(Type::Optional(Box::new(self.inner_type.clone())))
    }
}

impl<'a> Typecheck<'a> for Something {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let ty = self.term.check(env)?;
        Ok(Type::Optional(Box::new(ty)))
    }
}
impl<'a> Typecheck<'a> for SomeCase {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let bound_ty = self.bound_term.check(env)?;
        if let Type::Optional(ty) = bound_ty {
            let none_ty = self.none_rhs.check(&mut env.clone())?;
            env.used_vars.insert(self.some_var.clone(), *ty);
            let some_ty = self.some_rhs.check(env)?;
            if none_ty == some_ty {
                Ok(none_ty)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![none_ty, some_ty],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: bound_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod optional_tests {
    use super::{Nothing, SomeCase, Something};
    use crate::{syntax::Zero, types::Type};
    use common::Typecheck;

    #[test]
    fn check_nothing() {
        let result = Nothing {
            inner_type: Type::Bool,
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Optional(Box::new(Type::Bool));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_something() {
        let result = Something {
            term: Box::new(Zero.into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Optional(Box::new(Type::Nat));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_somecase() {
        let result = SomeCase {
            bound_term: Box::new(
                Nothing {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
            none_rhs: Box::new(Zero.into()),
            some_var: "x".to_owned(),
            some_rhs: Box::new("x".to_owned().into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
