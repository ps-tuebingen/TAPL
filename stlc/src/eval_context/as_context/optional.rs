use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::SomeCaseRhs, congruence, Value},
    syntax::{Nothing, SomeCase, Something},
};

impl AsContext for Nothing {
    fn to_context(self) -> Result<EvalContext, Error> {
        Ok(EvalContext::Value(Value::Nothing {
            inner_type: self.inner_type,
        }))
    }
}

impl AsContext for Something {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(val) => Ok(EvalContext::Value(Value::Something(Box::new(val)))),
            Err(_) => {
                let ctx = (*self.term).to_context()?;
                Ok(congruence::Something {
                    term: Box::new(ctx),
                }
                .into())
            }
        }
    }
}

impl AsContext for SomeCase {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.bound_term).try_into() {
            Ok(val) => Ok(SomeCaseRhs {
                bound_val: val,
                none_term: *self.none_rhs,
                some_var: self.some_var,
                some_term: *self.some_rhs,
            }
            .into()),
            Err(_) => {
                let ctx = (*self.bound_term).to_context()?;
                Ok(congruence::SomeCase {
                    bound_term: Box::new(ctx),
                    none_term: *self.none_rhs,
                    some_var: self.some_var,
                    some_term: *self.some_rhs,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod optional_tests {
    use super::{AsContext, EvalContext, Nothing, SomeCase, Something};
    use crate::{
        eval_context::{
            computation::{SomeCaseRhs, SuccPred},
            congruence, Value,
        },
        syntax::{False, Pred, Succ, True, Zero},
        types::Type,
    };

    #[test]
    fn ctx_nothing() {
        let result = Nothing {
            inner_type: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Nothing {
            inner_type: Type::Nat,
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_some_val() {
        let result = Something {
            term: Box::new(Zero.into()),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Something(Box::new(Value::Zero)));
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_some_cong() {
        let result = Something {
            term: Box::new(
                Succ {
                    term: Box::new(
                        Pred {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Something {
            term: Box::new(SuccPred { val: Value::Zero }.into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_case_val() {
        let result = SomeCase {
            bound_term: Box::new(
                Nothing {
                    inner_type: Type::Nat,
                }
                .into(),
            ),
            none_rhs: Box::new(True.into()),
            some_var: "x".to_owned(),
            some_rhs: Box::new(False.into()),
        }
        .to_context()
        .unwrap();
        let expected = SomeCaseRhs {
            bound_val: Value::Nothing {
                inner_type: Type::Nat,
            },
            none_term: True.into(),
            some_var: "x".to_owned(),
            some_term: False.into(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_case_cong() {
        let result = SomeCase {
            bound_term: Box::new(
                Something {
                    term: Box::new(
                        Succ {
                            term: Box::new(
                                Pred {
                                    term: Box::new(Zero.into()),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            none_rhs: Box::new(True.into()),
            some_var: "x".to_owned(),
            some_rhs: Box::new(False.into()),
        }
        .to_context()
        .unwrap();
        let expected = congruence::SomeCase {
            bound_term: Box::new(
                congruence::Something {
                    term: Box::new(SuccPred { val: Value::Zero }.into()),
                }
                .into(),
            ),
            none_term: True.into(),
            some_var: "x".to_owned(),
            some_term: False.into(),
        }
        .into();
        assert_eq!(result, expected)
    }
}
