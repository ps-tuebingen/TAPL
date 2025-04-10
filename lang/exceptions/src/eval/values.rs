use crate::{
    syntax::{Error, Lambda, Raise, Term, Unit, Var},
    types::Type,
};
use common::errors::ErrorKind;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    Const(i64),
    Unit,
    True,
    False,
    Exception(Type),
    Raise {
        val: Box<Value>,
        cont_ty: Type,
        ex_ty: Type,
    },
}

impl Value {
    pub fn into_lambda(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    pub fn into_num(self) -> Result<i64, ErrorKind> {
        if let Value::Const(i) = self {
            Ok(i)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Number".to_owned(),
            })
        }
    }
}

impl From<Lambda> for Value {
    fn from(lam: Lambda) -> Value {
        Value::Lambda {
            var: lam.var,
            annot: lam.annot,
            body: *lam.body,
        }
    }
}

impl From<Unit> for Value {
    fn from(_: Unit) -> Value {
        Value::Unit
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
            Value::Unit => Unit.into(),
            Value::True => Term::True,
            Value::False => Term::False,
            Value::Const(i) => Term::Const(i),
            Value::Exception(ty) => Error { ty }.into(),
            Value::Raise {
                val,
                cont_ty,
                ex_ty,
            } => Raise {
                exception: Box::new((*val).into()),
                cont_ty,
                ex_ty,
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
