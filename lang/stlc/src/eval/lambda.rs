use super::{to_eval_err, Value};
use crate::{
    syntax::{App, Lambda},
    traits::subst::Subst,
};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for Lambda {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::Lambda {
            var: self.var.clone(),
            annot: self.annot.clone(),
            body: *self.body.clone(),
        })
    }
}

impl Eval<'_> for App {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val1 = self.fun.eval(env)?;
        match val1 {
            Value::Lambda {
                var,
                annot: _,
                body,
            } => {
                let body_subst = body.subst(&var, *self.arg);
                body_subst.eval(env)
            }
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val1.to_string(),
                expected: "Function".to_owned(),
            })),
        }
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::{App, Eval, Lambda, Value};
    use crate::{syntax::True, types::Type};

    #[test]
    fn eval_lam() {
        let result = Lambda {
            var: "x".to_owned(),
            annot: Type::Bool,
            body: Box::new("x".to_owned().into()),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Lambda {
            var: "x".to_owned(),
            annot: Type::Bool,
            body: "x".to_owned().into(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_app() {
        let result = App {
            fun: Box::new(
                Lambda {
                    var: "x".to_owned(),
                    annot: Type::Bool,
                    body: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            arg: Box::new(True.into()),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
