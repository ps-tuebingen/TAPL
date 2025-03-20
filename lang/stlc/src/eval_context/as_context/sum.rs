use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::SumCaseRhs, congruence, Value},
    syntax::{Left, Right, SumCase},
};

impl AsContext for Left {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.left_term).try_into() {
            Ok(val) => Ok(EvalContext::Value(Value::Left {
                left_term: Box::new(val),
                ty: self.ty,
            })),
            Err(_) => {
                let ctx = (*self.left_term).to_context()?;
                Ok(congruence::Left {
                    left_term: Box::new(ctx),
                    ty: self.ty,
                }
                .into())
            }
        }
    }
}

impl AsContext for Right {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.right_term).try_into() {
            Ok(val) => Ok(EvalContext::Value(Value::Right {
                right_term: Box::new(val),
                ty: self.ty,
            })),
            Err(_) => {
                let ctx = (*self.right_term).to_context()?;
                Ok(congruence::Right {
                    right_term: Box::new(ctx),
                    ty: self.ty,
                }
                .into())
            }
        }
    }
}

impl AsContext for SumCase {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.bound_term).try_into() {
            Ok(val) => Ok(SumCaseRhs {
                bound_val: val,
                left_var: self.left_var,
                left_term: *self.left_term,
                right_var: self.right_var,
                right_term: *self.right_term,
            }
            .into()),
            Err(_) => {
                let ctx = (*self.bound_term).to_context()?;
                Ok(congruence::SumCase {
                    bound_term: Box::new(ctx),
                    left_var: self.left_var,
                    left_term: *self.left_term,
                    right_var: self.right_var,
                    right_term: *self.right_term,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod sum_tests {
    use super::{AsContext, EvalContext, Left, Right, SumCase};
    use crate::{
        eval_context::{
            computation::{IsZeroNum, SuccPred, SumCaseRhs},
            congruence, Value,
        },
        syntax::{False, IsZero, Pred, Succ, True, Zero},
        types::Type,
    };

    #[test]
    fn ctx_left_val() {
        let result = Left {
            left_term: Box::new(True.into()),
            ty: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Left {
            left_term: Box::new(Value::True),
            ty: Type::Nat,
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_left_cong() {
        let result = Left {
            left_term: Box::new(
                IsZero {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            ty: Type::Nat,
        }
        .to_context()
        .unwrap();
        let expected = congruence::Left {
            left_term: Box::new(IsZeroNum { num: Value::Zero }.into()),
            ty: Type::Nat,
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_right_val() {
        let result = Right {
            ty: Type::Nat,
            right_term: Box::new(True.into()),
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Right {
            right_term: Box::new(Value::True),
            ty: Type::Nat,
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_right_cong() {
        let result = Right {
            ty: Type::Bool,
            right_term: Box::new(
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
        let expected = congruence::Right {
            ty: Type::Bool,
            right_term: Box::new(SuccPred { val: Value::Zero }.into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_case_beta() {
        let result = SumCase {
            bound_term: Box::new(
                Left {
                    left_term: Box::new(True.into()),
                    ty: Type::Nat,
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new(True.into()),
            right_var: "y".to_owned(),
            right_term: Box::new(False.into()),
        }
        .to_context()
        .unwrap();
        let expected = SumCaseRhs {
            bound_val: Value::Left {
                left_term: Box::new(Value::True),
                ty: Type::Nat,
            },
            left_var: "x".to_owned(),
            left_term: True.into(),
            right_var: "y".to_owned(),
            right_term: False.into(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_case_cong() {
        let result = SumCase {
            bound_term: Box::new(
                Left {
                    ty: Type::Nat,
                    left_term: Box::new(
                        IsZero {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new(True.into()),
            right_var: "y".to_owned(),
            right_term: Box::new(False.into()),
        }
        .to_context()
        .unwrap();
        let expected = congruence::SumCase {
            bound_term: Box::new(
                congruence::Left {
                    left_term: Box::new(IsZeroNum { num: Value::Zero }.into()),
                    ty: Type::Nat,
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: True.into(),
            right_var: "y".to_owned(),
            right_term: False.into(),
        }
        .into();
        assert_eq!(result, expected)
    }
}
