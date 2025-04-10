use crate::{
    syntax::{Const, Label, Lambda, LambdaSub, Pack, Record, Term, Var},
    types::Type,
};
use common::errors::ErrorKind;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum Value {
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    LambdaSub {
        var: Var,
        sup_ty: Type,
        body: Term,
    },
    Pack {
        inner_ty: Type,
        val: Box<Value>,
        outer_ty: Type,
    },
    Const(i64),
    Record(HashMap<Label, Value>),
}

impl Value {
    pub fn as_lam(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    pub fn as_lamsub(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::LambdaSub { var, sup_ty, body } = self {
            Ok((var, sup_ty, body))
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction (subtype)".to_owned(),
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
                expected: "Existential Pack".to_owned(),
            })
        }
    }

    pub fn as_const(self) -> Result<i64, ErrorKind> {
        if let Value::Const(i) = self {
            Ok(i)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Number".to_owned(),
            })
        }
    }

    pub fn as_record(self) -> Result<HashMap<Label, Value>, ErrorKind> {
        if let Value::Record(recs) = self {
            Ok(recs)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Record".to_owned(),
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
            Value::LambdaSub { var, sup_ty, body } => LambdaSub {
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
            Value::Const(i) => Const { i }.into(),
            Value::Record(recs) => Record {
                records: recs.into_iter().map(|(lb, v)| (lb, v.into())).collect(),
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
