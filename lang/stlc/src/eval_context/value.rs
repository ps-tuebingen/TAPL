use crate::{
    syntax::{
        Cons, False, Lambda, Left, Nil, Nothing, Pair, Pred, Record, Right, Something, Succ, Term,
        True, Tup, Unit, Variant, Zero,
    },
    types::Type,
    Var,
};

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    Unit,
    True,
    False,
    Zero,
    Succ(Box<Value>),
    Pred(Box<Value>),
    Pair {
        fst: Box<Value>,
        snd: Box<Value>,
    },
    Tup(Vec<Value>),
    Record(HashMap<Var, Value>),
    Left {
        left_term: Box<Value>,
        right_ty: Type,
    },
    Right {
        right_term: Box<Value>,
        left_ty: Type,
    },
    Variant {
        label: Var,
        ty: Type,
        val: Box<Value>,
    },
    Nothing {
        inner_type: Type,
    },
    Something(Box<Value>),
    Nil {
        inner_type: Type,
    },
    Cons {
        inner_type: Type,
        fst: Box<Value>,
        rst: Box<Value>,
    },
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda { var, annot, body } => Lambda {
                var,
                annot,
                body: body.into(),
            }
            .into(),
            Value::Unit => Unit.into(),
            Value::True => True.into(),
            Value::False => False.into(),
            Value::Zero => Zero.into(),
            Value::Succ(val) => Succ {
                term: Box::new((*val).into()),
            }
            .into(),
            Value::Pred(val) => Pred {
                term: Box::new((*val).into()),
            }
            .into(),
            Value::Pair { fst: v1, snd: v2 } => Pair {
                fst: Box::new((*v1).into()),
                snd: Box::new((*v2).into()),
            }
            .into(),
            Value::Tup(vals) => Tup {
                terms: vals.into_iter().map(|val| val.into()).collect(),
            }
            .into(),
            Value::Record(records) => Record {
                records: records
                    .into_iter()
                    .map(|(label, val)| (label, val.into()))
                    .collect(),
            }
            .into(),
            Value::Left {
                left_term,
                right_ty,
            } => Left {
                left_term: Box::new((*left_term).into()),
                right_ty,
            }
            .into(),
            Value::Right {
                right_term,
                left_ty,
            } => Right {
                right_term: Box::new((*right_term).into()),
                left_ty,
            }
            .into(),
            Value::Variant { label, ty, val } => Variant {
                label,
                ty,
                term: Box::new((*val).into()),
            }
            .into(),
            Value::Nothing { inner_type } => Nothing { inner_type }.into(),
            Value::Something(val) => Something {
                term: Box::new((*val).into()),
            }
            .into(),
            Value::Nil { inner_type } => Nil { inner_type }.into(),
            Value::Cons {
                inner_type,
                fst,
                rst,
            } => Cons {
                inner_type,
                fst: Box::new((*fst).into()),
                rst: Box::new((*rst).into()),
            }
            .into(),
        }
    }
}

impl TryFrom<&Term> for Value {
    type Error = ();
    fn try_from(t: &Term) -> Result<Value, ()> {
        match t {
            Term::Lambda(lam) => Ok(Value::Lambda {
                var: lam.var.clone(),
                annot: lam.annot.clone(),
                body: *lam.body.clone(),
            }),
            Term::Unit(_) => Ok(Value::Unit),
            Term::True(_) => Ok(Value::True),
            Term::False(_) => Ok(Value::False),
            Term::Zero(_) => Ok(Value::Zero),
            Term::Pred(p) => {
                let inner = (&*p.term).try_into()?;
                if let Value::Succ(_) = inner {
                    Err(())
                } else {
                    Ok(Value::Pred(Box::new(inner)))
                }
            }
            Term::Succ(s) => {
                let inner = (&*s.term).try_into()?;
                if let Value::Pred(_) = inner {
                    Err(())
                } else {
                    Ok(Value::Succ(Box::new(inner)))
                }
            }
            Term::Pair(p) => {
                let p1 = (&*p.fst).try_into()?;
                let p2 = (&*p.snd).try_into()?;
                Ok(Value::Pair {
                    fst: Box::new(p1),
                    snd: Box::new(p2),
                })
            }
            Term::Tup(tup) => {
                let inner = tup
                    .terms
                    .iter()
                    .map(|t| t.try_into())
                    .collect::<Result<Vec<Value>, ()>>()?;
                Ok(Value::Tup(inner))
            }
            Term::Record(rec) => {
                let mut new_rec = HashMap::new();
                for (label, t) in rec.records.iter() {
                    let new_t = t.try_into()?;
                    new_rec.insert(label.clone(), new_t);
                }
                Ok(Value::Record(new_rec))
            }
            Term::Left(l) => {
                let inner = (&*l.left_term).try_into()?;
                Ok(Value::Left {
                    left_term: Box::new(inner),
                    right_ty: l.right_ty.clone(),
                })
            }
            Term::Right(r) => {
                let inner = (&*r.right_term).try_into()?;
                Ok(Value::Right {
                    right_term: Box::new(inner),
                    left_ty: r.left_ty.clone(),
                })
            }
            Term::Variant(var) => {
                let inner = (&*var.term).try_into()?;
                Ok(Value::Variant {
                    label: var.label.clone(),
                    ty: var.ty.clone(),
                    val: Box::new(inner),
                })
            }
            Term::Nothing(not) => Ok(Value::Nothing {
                inner_type: not.inner_type.clone(),
            }),
            Term::Something(s) => {
                let inner = (&*s.term).try_into()?;
                Ok(Value::Something(Box::new(inner)))
            }
            Term::Nil(nil) => Ok(Value::Nil {
                inner_type: nil.inner_type.clone(),
            }),
            Term::Cons(c) => {
                let fst = (&*c.fst).try_into()?;
                let rst = (&*c.rst).try_into()?;
                Ok(Value::Cons {
                    inner_type: c.inner_type.clone(),
                    fst: Box::new(fst),
                    rst: Box::new(rst),
                })
            }
            _ => Err(()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}

#[cfg(test)]
mod val_tests {
    use super::{Term, Value};
    use crate::{syntax::*, types::Type};
    use std::collections::HashMap;

    #[test]
    fn from_lambda() {
        let result: Value = (&Term::Lambda(Lambda {
            var: "x".to_owned(),
            annot: Type::Nat,
            body: Box::new("x".to_owned().into()),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Lambda {
            var: "x".to_owned(),
            annot: Type::Nat,
            body: "x".to_owned().into(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_unit() {
        let result: Value = (&Term::Unit(Unit)).try_into().unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_true() {
        let result: Value = (&Term::True(True)).try_into().unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_false() {
        let result: Value = (&Term::False(False)).try_into().unwrap();
        let expected = Value::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_zero() {
        let result: Value = (&Term::Zero(Zero)).try_into().unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn from_pred() {
        let result: Value = (&Term::Pred(Pred {
            term: Box::new(
                Pred {
                    term: Box::new(
                        Pred {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Pred(Box::new(Value::Pred(Box::new(Value::Pred(Box::new(
            Value::Zero,
        ))))));
        assert_eq!(result, expected)
    }

    #[test]
    fn from_pred_err() {
        let result: Result<Value, ()> = (&Term::Pred(Pred {
            term: Box::new(
                Succ {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
        }))
            .try_into();
        assert!(result.is_err())
    }

    #[test]
    fn from_succ() {
        let result: Value = (&Term::Succ(Succ {
            term: Box::new(
                Succ {
                    term: Box::new(
                        Succ {
                            term: Box::new(Zero.into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Succ(Box::new(Value::Succ(Box::new(Value::Succ(Box::new(
            Value::Zero,
        ))))));
        assert_eq!(result, expected)
    }

    #[test]
    fn from_succ_fail() {
        let result: Result<Value, ()> = (&Term::Succ(
            Succ {
                term: Box::new(
                    Pred {
                        term: Box::new(Zero.into()),
                    }
                    .into(),
                ),
            }
            .into(),
        ))
            .try_into();
        assert!(result.is_err())
    }

    #[test]
    fn from_pair() {
        let result: Value = (&Term::Pair(Pair {
            fst: Box::new(True.into()),
            snd: Box::new(Zero.into()),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Pair {
            fst: Box::new(Value::True),
            snd: Box::new(Value::Zero),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_tup() {
        let result: Value = (&Term::Tup(Tup {
            terms: vec![True.into(), False.into(), Zero.into()],
        }))
            .try_into()
            .unwrap();
        let expected = Value::Tup(vec![Value::True, Value::False, Value::Zero]);
        assert_eq!(result, expected)
    }

    #[test]
    fn from_rec() {
        let result: Value = (&Term::Record(Record {
            records: HashMap::from([
                ("label1".to_owned(), True.into()),
                ("label2".to_owned(), False.into()),
                ("label3".to_owned(), Zero.into()),
            ]),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Record(HashMap::from([
            ("label1".to_owned(), Value::True),
            ("label2".to_owned(), Value::False),
            ("label3".to_owned(), Value::Zero),
        ]));
        assert_eq!(result, expected)
    }

    #[test]
    fn from_left() {
        let result: Value = (&Term::Left(Left {
            left_term: Box::new(True.into()),
            right_ty: Type::Nat,
        }))
            .try_into()
            .unwrap();
        let expected = Value::Left {
            left_term: Box::new(Value::True),
            right_ty: Type::Nat,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_right() {
        let result: Value = (&Term::Right(Right {
            right_term: Box::new(False.into()),
            left_ty: Type::Bool,
        }))
            .try_into()
            .unwrap();
        let expected = Value::Right {
            right_term: Box::new(Value::False),
            left_ty: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_variant() {
        let result: Value = (&Term::Variant(Variant {
            label: "label".to_owned(),
            term: Box::new(True.into()),
            ty: Type::Variant(HashMap::from([
                ("label".to_owned(), Type::Bool),
                ("other".to_owned(), Type::Nat),
            ])),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Variant {
            label: "label".to_owned(),
            val: Box::new(Value::True),
            ty: Type::Variant(HashMap::from([
                ("label".to_owned(), Type::Bool),
                ("other".to_owned(), Type::Nat),
            ])),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_nothing() {
        let result: Value = (&Term::Nothing(Nothing {
            inner_type: Type::Nat,
        }))
            .try_into()
            .unwrap();
        let expected = Value::Nothing {
            inner_type: Type::Nat,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_something() {
        let result: Value = (&Term::Something(Something {
            term: Box::new(Zero.into()),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Something(Box::new(Value::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn from_nil() {
        let result: Value = (&Term::Nil(Nil {
            inner_type: Type::Bool,
        }))
            .try_into()
            .unwrap();
        let expected = Value::Nil {
            inner_type: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn from_cons() {
        let result: Value = (&Term::Cons(Cons {
            inner_type: Type::Bool,
            fst: Box::new(True.into()),
            rst: Box::new(
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
            ),
        }))
            .try_into()
            .unwrap();
        let expected = Value::Cons {
            inner_type: Type::Bool,
            fst: Box::new(Value::True),
            rst: Box::new(Value::Nil {
                inner_type: Type::Bool,
            }),
        };
        assert_eq!(result, expected)
    }
}
