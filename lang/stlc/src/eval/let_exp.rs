use super::{errors::Error, Value};
use crate::{syntax::Let, traits::subst::Subst};
use common::Eval;

impl Eval<'_> for Let {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
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
        .eval(Default::default())
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
