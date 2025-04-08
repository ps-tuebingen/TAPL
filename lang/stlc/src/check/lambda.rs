use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        env.used_vars.insert(self.var.clone(), self.annot.clone());
        let ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot.clone()), Box::new(ty)))
    }
}

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let ty1 = self.fun.check(&mut env.clone())?;
        if let Type::Fun(ty11, ty12) = ty1 {
            let ty2 = self.arg.check(env)?;
            if ty2 == *ty11 {
                Ok(*ty12)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![*ty11, ty2],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: ty1,
                term: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::{App, Lambda};
    use crate::{syntax::Zero, types::Type};
    use common::Typecheck;

    #[test]
    fn check_lam() {
        let result = Lambda {
            var: "x".to_owned(),
            annot: Type::Nat,
            body: Box::new("x".to_owned().into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Fun(Box::new(Type::Nat), Box::new(Type::Nat));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_app() {
        let result = App {
            fun: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    annot: Type::Nat,
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            arg: Box::new(Zero.into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
