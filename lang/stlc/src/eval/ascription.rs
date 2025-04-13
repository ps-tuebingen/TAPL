use super::Value;
use crate::syntax::Ascribe;
use common::{errors::Error, Eval};

impl Eval<'_> for Ascribe {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
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
