use super::{to_eval_err, Value};
use crate::{
    syntax::{Nothing, SomeCase, Something},
    traits::subst::Subst,
};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for Something {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        Ok(Value::Something(Box::new(val)))
    }
}

impl Eval<'_> for Nothing {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::Nothing {
            inner_type: self.inner_type,
        })
    }
}

impl Eval<'_> for SomeCase {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let bound_res = self.bound_term.eval(env)?;
        match bound_res {
            Value::Nothing { .. } => self.none_rhs.eval(env),
            Value::Something(val) => self.some_rhs.subst(&self.some_var, (*val).into()).eval(env),
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: bound_res.to_string(),
                expected: "Option".to_owned(),
            })),
        }
    }
}

#[cfg(test)]
mod optional_tests {
    use super::{Eval, Nothing, Something, Value};
    use crate::{syntax::Zero, types::Type};

    #[test]
    fn eval_nothing() {
        let result = Nothing {
            inner_type: Type::Bool,
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Nothing {
            inner_type: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_something() {
        let result = Something {
            term: Box::new(Zero.into()),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Something(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }
}
