use crate::{
    errors::ErrorKind,
    syntax::{
        kinds::Kind,
        terms::{False, Lambda, Pack, Record, Term, True, TyLambda, Var},
        types::{Type, TypeVar},
        Label,
    },
};
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
        annot: Kind,
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
    True,
    False,
    Unit,
    Const(i64),
}

impl Value {
    pub fn as_lambda(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::BadValue {
                found: self,
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    pub fn as_tylambda(self) -> Result<(TypeVar, Kind, Term), ErrorKind> {
        if let Value::TyLambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::BadValue {
                found: self,
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
            Err(ErrorKind::BadValue {
                found: self,
                expected: "Package".to_owned(),
            })
        }
    }

    pub fn as_rec(self) -> Result<HashMap<Label, Value>, ErrorKind> {
        if let Value::Record { records } = self {
            Ok(records)
        } else {
            Err(ErrorKind::BadValue {
                found: self,
                expected: "Record".to_owned(),
            })
        }
    }

    pub fn as_const(self) -> Result<i64, ErrorKind> {
        if let Value::Const(i) = self {
            Ok(i)
        } else {
            Err(ErrorKind::BadValue {
                found: self,
                expected: "Constant".to_owned(),
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
            Value::TyLambda { var, annot, body } => TyLambda {
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
            Value::Record { records } => Record {
                records: records
                    .into_iter()
                    .map(|(label, val)| (label, val.into()))
                    .collect(),
            }
            .into(),
            Value::True => True.into(),
            Value::False => False.into(),
            Value::Unit => Term::Unit,
            Value::Const(i) => i.into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
