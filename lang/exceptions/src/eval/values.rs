use crate::{syntax::Term, types::Type};
use common::values::{Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Num(Num),
    Unit(Unit),
    True(True),
    False(False),
    Exception(Exception<Type>),
    Raise(Raise<Value, Type, Term>),
}

impl ValueTrait<Term> for Value {
    type Term = Term;
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into(),
            Value::Unit(u) => u.into(),
            Value::True(tru) => tru.into(),
            Value::False(fls) => fls.into(),
            Value::Num(num) => num.into(),
            Value::Exception(exc) => exc.into(),
            Value::Raise(raise) => raise.into(),
        }
    }
}
