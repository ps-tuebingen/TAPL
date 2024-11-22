use crate::{
    terms::syntax::{
        Cons, False, Lambda, Left, Nil, Nothing, Pair, Record, Right, Something, Term, True, Tup,
        Unit, Variant,
    },
    types::Type,
    Var,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    Unit,
    True,
    False,
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
