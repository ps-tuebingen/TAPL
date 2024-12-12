use super::{errors::Error, Eval, Value};
use crate::{
    syntax::{Nothing, SomeCase, Something},
    traits::subst::Subst,
};

impl Eval for Something {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Something(Box::new(val)))
    }
}

impl Eval for Nothing {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Nothing {
            inner_type: self.inner_type,
        })
    }
}

impl Eval for SomeCase {
    fn eval(self) -> Result<Value, Error> {
        let bound_res = self.bound_term.eval()?;
        match bound_res {
            Value::Nothing { .. } => self.none_rhs.eval(),
            Value::Something(val) => self.some_rhs.subst(&self.some_var, (*val).into()).eval(),
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
        .eval()
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
        .eval()
        .unwrap();
        let expected = Value::Something(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }
}
