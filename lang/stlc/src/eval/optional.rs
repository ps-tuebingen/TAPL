use super::{errors::Error, Value};
use crate::{
    syntax::{Nothing, SomeCase, Something},
    traits::subst::Subst,
};
use common::Eval;

impl Eval for Something {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: &mut Self::Env) -> Result<Value, Error> {
        let val = self.term.eval(env)?;
        Ok(Value::Something(Box::new(val)))
    }
}

impl Eval for Nothing {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _: &mut Self::Env) -> Result<Value, Error> {
        Ok(Value::Nothing {
            inner_type: self.inner_type,
        })
    }
}

impl Eval for SomeCase {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: &mut Self::Env) -> Result<Value, Error> {
        let bound_res = self.bound_term.eval(env)?;
        match bound_res {
            Value::Nothing { .. } => self.none_rhs.eval(env),
            Value::Something(val) => self.some_rhs.subst(&self.some_var, (*val).into()).eval(env),
            _ => Err(Error::BadValue { val: bound_res }),
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
        .eval(&mut Default::default())
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
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Something(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }
}
