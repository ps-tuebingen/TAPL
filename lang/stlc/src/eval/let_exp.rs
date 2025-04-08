use super::{errors::Error, Value};
use crate::{syntax::Let, traits::subst::Subst};
use common::Eval;

impl Eval for Let {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: &mut Self::Env) -> Result<Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        self.in_term.subst(&self.var, bound_val.into()).eval(env)
    }
}

#[cfg(test)]
mod let_tests {
    use super::{Eval, Let, Value};
    use crate::syntax::{If, Succ, True, Zero};

    #[test]
    fn eval_let() {
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(True.into()),
            in_term: Box::new(
                If {
                    ifc: Box::new("x".to_owned().into()),
                    thenc: Box::new(Zero.into()),
                    elsec: Box::new(
                        Succ {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
