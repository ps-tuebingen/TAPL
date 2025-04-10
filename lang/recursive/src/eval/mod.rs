use crate::{
    errors::ErrorKind,
    terms::{False, Fold, Lambda, Pair, Record, Term, True, Var, Variant},
    types::Type,
    Label,
};
use std::{collections::HashMap, fmt};

pub mod bool;
pub mod fix;
pub mod fold;
pub mod lambda;
pub mod let_exp;
pub mod nat;
pub mod pair;
pub mod record;
pub mod terms;
pub mod variant;

#[derive(Debug, Clone)]
pub enum Value {
    True,
    False,
    Unit,
    Const(i64),
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    Fold {
        ty: Type,
        val: Box<Value>,
    },
    Pair {
        fst: Box<Value>,
        snd: Box<Value>,
    },
    Record(HashMap<Label, Value>),
    Variant {
        label: Label,
        val: Box<Value>,
        annot: Type,
    },
}

impl Value {
    pub fn into_lambda(self) -> Result<Lambda, ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok(Lambda {
                var,
                annot,
                body: Box::new(body),
            })
        } else {
            Err(ErrorKind::value(self, "Lambda Abstraction"))
        }
    }

    pub fn into_fold(self) -> Result<(Type, Value), ErrorKind> {
        if let Value::Fold { ty, val } = self {
            Ok((ty, *val))
        } else {
            Err(ErrorKind::value(self, "Fold Term"))
        }
    }

    pub fn into_const(self) -> Result<i64, ErrorKind> {
        if let Value::Const(i) = self {
            Ok(i)
        } else {
            Err(ErrorKind::value(self, "Numerical Value"))
        }
    }

    pub fn into_pair(self) -> Result<(Value, Value), ErrorKind> {
        if let Value::Pair { fst, snd } = self {
            Ok((*fst, *snd))
        } else {
            Err(ErrorKind::value(self, "Pair"))
        }
    }

    pub fn into_record(self) -> Result<HashMap<Label, Value>, ErrorKind> {
        if let Value::Record(recs) = self {
            Ok(recs)
        } else {
            Err(ErrorKind::value(self, "Record"))
        }
    }

    pub fn into_variant(self) -> Result<(Label, Value, Type), ErrorKind> {
        if let Value::Variant { label, val, annot } = self {
            Ok((label, *val, annot))
        } else {
            Err(ErrorKind::value(self, "Variant"))
        }
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Unit => Term::Unit,
            Value::True => True.into(),
            Value::False => False.into(),
            Value::Const(i) => i.into(),
            Value::Lambda { var, annot, body } => Lambda {
                var,
                annot,
                body: Box::new(body),
            }
            .into(),
            Value::Fold { ty, val } => Fold {
                ty,
                term: Box::new((*val).into()),
            }
            .into(),
            Value::Pair { fst, snd } => Pair {
                fst: Box::new((*fst).into()),
                snd: Box::new((*snd).into()),
            }
            .into(),
            Value::Record(recs) => Record {
                records: recs.into_iter().map(|(lb, val)| (lb, val.into())).collect(),
            }
            .into(),
            Value::Variant { label, val, annot } => Variant {
                label,
                term: Box::new((*val).into()),
                annot,
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
