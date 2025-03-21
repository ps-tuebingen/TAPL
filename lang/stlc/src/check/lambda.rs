use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};

impl Check for Lambda {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        env.used_vars.insert(self.var.clone(), self.annot.clone());
        let ty = self.body.check(env)?;
        Ok(Type::Fun(Box::new(self.annot.clone()), Box::new(ty)))
    }
}

impl Check for App {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let ty1 = self.fun.check_local(env)?;
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
    use super::{App, Check, Lambda};
    use crate::{syntax::Zero, types::Type};

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
