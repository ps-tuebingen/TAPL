use crate::{
    errors::ErrorKind,
    syntax::{Lambda, LambdaSub, Pack, Term, Var},
    types::Type,
};
use std::fmt;

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
}

impl Value {
    pub fn as_lam(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(ErrorKind::BadValue {
                found: self,
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    pub fn as_lamsub(self) -> Result<(Var, Type, Term), ErrorKind> {
        if let Value::LambdaSub { var, sup_ty, body } = self {
            Ok((var, sup_ty, body))
        } else {
            Err(ErrorKind::BadValue {
                found: self,
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
            Err(ErrorKind::BadValue {
                found: self,
                expected: "Existential Pack".to_owned(),
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
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
