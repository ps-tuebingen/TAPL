use super::{terms::Term, types::Type};
use common::errors::ErrorKind;
use eval::values::{
    Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, ValueGroup,
    Variant,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Unit(Unit<Term>),
    Record(Record<Value>),
    Variant(Variant<Value, Type>),
    Nil(Nil<Term, Type>),
    Cons(Cons<Type, Value>),
    Loc(Loc<Term>),
    Num(Num<Term>),
    True(True<Term>),
    False(False<Term>),
}

impl eval::values::Value for Value {
    type Term = Term;
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Type;

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ErrorKind> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    fn into_record(self) -> Result<Record<Term>, ErrorKind> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Record".to_owned(),
            })
        }
    }

    fn into_variant(self) -> Result<Variant<Term>, ErrorKind> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Variant".to_owned(),
            })
        }
    }

    fn into_nil(self) -> Result<Nil<Term>, ErrorKind> {
        if let Value::Nil(nil) = self {
            Ok(nil)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Nil".to_owned(),
            })
        }
    }

    fn into_cons(self) -> Result<Cons<Term>, ErrorKind> {
        if let Value::Cons(cons) = self {
            Ok(cons)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Cons".to_owned(),
            })
        }
    }

    fn into_loc(self) -> Result<Loc<Term>, ErrorKind> {
        if let Value::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Location".to_owned(),
            })
        }
    }

    fn into_num(self) -> Result<Num<Term>, ErrorKind> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Number".to_owned(),
            })
        }
    }

    fn into_true(self) -> Result<True<Term>, ErrorKind> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "True".to_owned(),
            })
        }
    }

    fn into_false(self) -> Result<False<Term>, ErrorKind> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "False".to_owned(),
            })
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::Variant(var) => var.into_term().into(),
            Value::Nil(nil) => nil.into_term().into(),
            Value::Cons(cons) => cons.into_term().into(),
            Value::Loc(loc) => loc.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Variant(var) => var.fmt(f),
            Value::Nil(nil) => nil.fmt(f),
            Value::Cons(cons) => cons.fmt(f),
            Value::Loc(loc) => loc.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
        }
    }
}

impl From<Loc<Term>> for Value {
    fn from(loc: Loc<Term>) -> Value {
        Value::Loc(loc)
    }
}
impl From<Lambda<Term, Type>> for Value {
    fn from(lam: Lambda<Term, Type>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<Term>> for Value {
    fn from(u: Unit<Term>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<Term>> for Value {
    fn from(tru: True<Term>) -> Value {
        Value::True(tru)
    }
}
impl From<False<Term>> for Value {
    fn from(fls: False<Term>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<Term>> for Value {
    fn from(num: Num<Term>) -> Value {
        Value::Num(num)
    }
}

impl From<Record<Term>> for Value {
    fn from(rec: Record<Term>) -> Value {
        Value::Record(rec)
    }
}

impl From<Variant<Value, Type>> for Value {
    fn from(var: Variant<Value, Type>) -> Value {
        Value::Variant(var)
    }
}

impl From<Nil<Term, Type>> for Value {
    fn from(nil: Nil<Term, Type>) -> Value {
        Value::Nil(nil)
    }
}
impl From<Cons<Value, Type>> for Value {
    fn from(cons: Cons<Value, Type>) -> Value {
        Value::Cons(cons)
    }
}
