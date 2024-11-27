use super::{errors::Error, Eval, Value};
use crate::syntax::Ascribe;

impl Eval for Ascribe {
    fn eval(self) -> Result<Value, Error> {
        self.term.eval()
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
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
