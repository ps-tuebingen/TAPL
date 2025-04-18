use crate::syntax::{
    terms::{Lambda, Pack, Pred, Record, Succ, Term, TyLambda, Var},
    types::{Type, TypeVar},
};
use common::{errors::ErrorKind, Label};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum Value {
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    TyLambda {
        var: TypeVar,
        sup_ty: Type,
        body: Term,
    },
    Pack {
        inner_ty: Type,
        val: Box<Value>,
        outer_ty: Type,
    },
    Record {
        records: HashMap<Label, Value>,
    },
    Zero,
    Succ {
        term: Box<Value>,
    },
    Pred {
        term: Box<Value>,
    },
}

impl Value {
    pub fn as_lambda(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    pub fn as_tylambda(self) -> Result<(TypeVar, Type, Term), ErrorKind> {
        if let Value::TyLambda { var, sup_ty, body } = self {
            Ok((var, sup_ty, body))
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Type Abstraction".to_owned(),
            })
        }
    }

    pub fn as_pack(self) -> Result<(Type, Value, Type), ErrorKind> {
        if let Value::Pack {
            inner_ty,
            val,
            outer_ty,
        } = self
        {
            Ok((inner_ty, *val, outer_ty))
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Package Value".to_owned(),
            })
        }
    }

    pub fn as_rec(self) -> Result<HashMap<Label, Value>, ErrorKind> {
        if let Value::Record { records } = self {
            Ok(records)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Record Value".to_owned(),
            })
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda { var, annot, body } => Lambda {
                var,
                annot,
                body: Box::new(body),
            }
            .into(),
            Value::TyLambda { var, sup_ty, body } => TyLambda {
                var,
                sup_ty,
                body: Box::new(body),
            }
            .into(),
            Value::Pack {
                inner_ty,
                val,
                outer_ty,
            } => Pack {
                inner_ty,
                term: Box::new((*val).into()),
                outer_ty,
            }
            .into(),
            Value::Record { records } => Record {
                records: records
                    .into_iter()
                    .map(|(label, val)| (label, val.into()))
                    .collect(),
            }
            .into(),
            Value::Zero => Term::Zero,
            Value::Succ { term } => Succ {
                term: Box::new((*term).into()),
            }
            .into(),
            Value::Pred { term } => Pred {
                term: Box::new((*term).into()),
            }
            .into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
