use crate::{syntax::Term, types::Type};
use common::{
    errors::ErrorKind,
    values::{Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Num(Num),
    Unit(Unit),
    True(True),
    False(False),
    Exception(Exception<Type>),
    Raise(Raise<Value, Type>),
}

impl ValueTrait for Value {}

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
