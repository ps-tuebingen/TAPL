use crate::{
    errors::ErrorKind,
    terms::{False, Lambda, Pack, Pred, Record, Succ, Term, True, Var, Zero},
    types::Type,
    Label,
};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub enum Value {
    Unit,
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    Pack {
        inner_ty: Type,
        val: Box<Value>,
        outer_ty: Type,
    },
    Zero,
    Succ(Box<Value>),
    Pred(Box<Value>),
    Record(HashMap<Label, Value>),
    True,
    False,
}

impl Value {
    pub fn as_lambda(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::value(&self, "Lambda Abstraction"))
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
            Err(ErrorKind::value(&self, "Pack Value"))
        }
    }

    pub fn as_rec(self) -> Result<HashMap<Label, Value>, ErrorKind> {
        if let Value::Record(recs) = self {
            Ok(recs)
        } else {
            Err(ErrorKind::value(&self, "Record Value"))
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Unit => Term::Unit,
            Value::Lambda { var, annot, body } => Lambda {
                var,
                annot,
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
            Value::Zero => Zero.into(),
            Value::Succ(val) => Succ(Box::new((*val).into())).into(),
            Value::Pred(val) => Pred(Box::new((*val).into())).into(),
            Value::Record(recs) => Record {
                records: recs
                    .into_iter()
                    .map(|(label, val)| (label, val.into()))
                    .collect(),
            }
            .into(),
            Value::True => True.into(),
            Value::False => False.into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
