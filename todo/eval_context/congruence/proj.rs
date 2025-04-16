use super::{CongruenceRule, Error, Eval, EvalContext, Value};
use crate::eval_context::computation::ProjBeta;

#[derive(Debug, PartialEq, Eq)]
pub struct Proj {
    pub tup: Box<EvalContext>,
    pub ind: usize,
}

impl Eval for Proj {
    fn eval(self) -> Result<Value, Error> {
        let tup_val = self.tup.eval()?;
        let ctx: EvalContext = ProjBeta {
            tup: tup_val,
            ind: self.ind,
        }
        .into();
        ctx.eval()
    }
}

impl From<Proj> for CongruenceRule {
    fn from(proj: Proj) -> CongruenceRule {
        CongruenceRule::Proj(proj)
    }
}

impl From<Proj> for EvalContext {
    fn from(proj: Proj) -> EvalContext {
        EvalContext::Congruence(proj.into())
    }
}

#[cfg(test)]
mod proj_tests {
    use super::{Eval, Proj};
    use crate::{
        eval_context::{computation::TailList, congruence::Tup, Value},
        syntax::Zero,
        types::Type,
    };

    #[test]
    fn eval_proj() {
        let result = Proj {
            tup: Box::new(
                Tup {
                    values: vec![Value::True],
                    next: Box::new(
                        TailList {
                            list: Value::Cons {
                                fst: Box::new(Value::Zero),
                                rst: Box::new(Value::Nil {
                                    inner_type: Type::Nat,
                                }),
                                inner_type: Type::Nat,
                            },
                        }
                        .into(),
                    ),
                    rest: vec![Zero.into()],
                }
                .into(),
            ),
            ind: 0,
        }
        .eval()
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
