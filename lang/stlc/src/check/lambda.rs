use super::{to_check_err, TypingEnv};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        env.used_vars.insert(self.var.clone(), self.annot.clone());
        let ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot.clone()), Box::new(ty)))
    }
}

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let ty1 = self.fun.check(&mut env.clone())?;
        if let Type::Fun(ty11, ty12) = ty1 {
            let ty2 = self.arg.check(env)?;
            if ty2 == *ty11 {
                Ok(*ty12)
            } else {
                Err(to_check_err(ErrorKind::TypeMismatch {
                    found: ty11.to_string(),
                    expected: ty2.to_string(),
                }))
            }
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: ty1.to_string(),
                expected: "Function Type".to_owned(),
            }))
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
