use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::ProjBeta, congruence, Value},
    syntax::{Proj, Tup},
};

impl AsContext for Tup {
    fn to_context(self) -> Result<EvalContext, Error> {
        let mut vals = vec![];
        let mut non_vals = vec![];
        let mut collecting_values = true;
        for t in self.terms.into_iter() {
            if collecting_values {
                match (&t).try_into() {
                    Ok(val) => vals.push(val),
                    Err(_) => {
                        collecting_values = false;
                        non_vals.push(t);
                    }
                }
            } else {
                non_vals.push(t);
            }
        }
        if non_vals.is_empty() {
            Ok(EvalContext::Value(Value::Tup(vals)))
        } else {
            let next = non_vals.remove(0).to_context()?;
            Ok(congruence::Tup {
                values: vals,
                next: Box::new(next),
                rest: non_vals,
            }
            .into())
        }
    }
}

impl AsContext for Proj {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.tup).try_into() {
            Ok(val) => Ok(ProjBeta {
                tup: val,
                ind: self.ind,
            }
            .into()),
            Err(_) => {
                let tup_ctx = (*self.tup).to_context()?;
                Ok(congruence::Proj {
                    tup: Box::new(tup_ctx),
                    ind: self.ind,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod tup_tests {
    use super::{AsContext, EvalContext, Proj, Tup};
    use crate::{
        eval_context::{
            computation::{IsZeroNum, ProjBeta},
            congruence, Value,
        },
        syntax::{IsZero, Nil, True, Zero},
        types::Type,
    };

    #[test]
    fn ctx_tup_val() {
        let result = Tup {
            terms: vec![
                Zero.into(),
                True.into(),
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ],
        }
        .to_context()
        .unwrap();
        let expected = EvalContext::Value(Value::Tup(vec![
            Value::Zero,
            Value::True,
            Value::Nil {
                inner_type: Type::Bool,
            },
        ]));
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_tup_cong() {
        let result = Tup {
            terms: vec![
                Zero.into(),
                IsZero {
                    term: Box::new(Zero.into()),
                }
                .into(),
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ],
        }
        .to_context()
        .unwrap();
        let expected = congruence::Tup {
            values: vec![Value::Zero],
            next: Box::new(IsZeroNum { num: Value::Zero }.into()),
            rest: vec![Nil {
                inner_type: Type::Bool,
            }
            .into()],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj_beta() {
        let result = Proj {
            tup: Box::new(
                Tup {
                    terms: vec![
                        Zero.into(),
                        True.into(),
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                    ],
                }
                .into(),
            ),
            ind: 1,
        }
        .to_context()
        .unwrap();
        let expected = ProjBeta {
            tup: Value::Tup(vec![
                Value::Zero,
                Value::True,
                Value::Nil {
                    inner_type: Type::Bool,
                },
            ]),
            ind: 1,
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_proj_cong() {
        let result = Proj {
            tup: Box::new(
                Tup {
                    terms: vec![
                        Zero.into(),
                        IsZero {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                    ],
                }
                .into(),
            ),
            ind: 1,
        }
        .to_context()
        .unwrap();
        let expected = congruence::Proj {
            tup: Box::new(
                congruence::Tup {
                    values: vec![Value::Zero],
                    next: Box::new(IsZeroNum { num: Value::Zero }.into()),
                    rest: vec![Value::Nil {
                        inner_type: Type::Bool,
                    }
                    .into()],
                }
                .into(),
            ),
            ind: 1,
        }
        .into();
        assert_eq!(result, expected)
    }
}
