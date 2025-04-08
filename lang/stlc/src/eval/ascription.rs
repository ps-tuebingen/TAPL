use super::{errors::Error, Value};
use crate::syntax::Ascribe;
use common::Eval;

impl<'a> Eval<'a> for Ascribe {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        self.term.eval(env)
    }
}

#[cfg(test)]
mod ascribe_tests {
    use super::{Ascribe, Eval};
    use crate::{eval::Value, syntax::True, types::Type};

    #[test]
    fn eval_ascribe() {
        let result = Ascribe {
            term: Box::new(True.into()),
            ty: Type::Bool,
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
