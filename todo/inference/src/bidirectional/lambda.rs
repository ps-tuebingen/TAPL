use super::{to_infer_err, Environment, Infer};
use crate::{
    syntax::{App, Lambda},
    types::Type,
};
use common::errors::{Error, ErrorKind};

impl Infer for Lambda {
    fn infer(&self, _: &mut Environment) -> Result<Type, Error> {
        Err(to_infer_err(ErrorKind::Infer {
            term: self.to_string(),
            reason: format!("Unknown Type for variable {}", self.var),
        }))
    }

    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        if let Type::Fun(ty1, ty2) = target {
            env.insert(self.var.clone(), *ty1);
            self.body.check(*ty2, env)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: target.to_string(),
            }))
        }
    }
}

impl Infer for App {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        let fun_ty = self.fun.infer_local(env)?;
        if let Type::Fun(ty1, ty2) = fun_ty {
            self.arg.check(*ty1, env)?;
            Ok(*ty2)
        } else {
            Err(to_infer_err(ErrorKind::TypeMismatch {
                found: fun_ty.to_string(),
                expected: "Function Type".to_owned(),
            }))
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        let arg_ty = self.arg.infer_local(env)?;
        self.fun
            .check(Type::Fun(Box::new(arg_ty), Box::new(target)), env)
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::{App, Infer, Lambda};
    use crate::{
        syntax::{Ascribe, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn infer_lambda() {
        let result = Lambda {
            var: "x".to_owned(),
            body: Box::new("x".to_owned().into()),
        }
        .infer(&mut HashMap::new());
        assert!(result.is_err())
    }

    #[test]
    fn check_lambda() {
        Lambda {
            var: "x".to_owned(),
            body: Box::new("x".to_owned().into()),
        }
        .check(
            Type::Fun(Box::new(Type::Bool), Box::new(Type::Bool)),
            &mut HashMap::new(),
        )
        .unwrap()
    }

    #[test]
    fn infer_app() {
        let result = App {
            fun: Box::new(
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
            arg: Box::new(Zero.into()),
        }
        .infer(&mut HashMap::new())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_app() {
        App {
            fun: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            arg: Box::new(Zero.into()),
        }
        .check(Type::Nat, &mut HashMap::new())
        .unwrap()
    }
}
