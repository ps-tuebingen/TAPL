use super::terms::Term;
use common::{
    errors::ErrorKind,
    language::LanguageValue,
    values::{
        Cons, False, Lambda, Left, Nil, Nothing, Num, Pair, Record, Right, Something, True, Tuple,
        Unit, Value as ValueTrait, Variant,
    },
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term>),
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    Num(Num<Term>),
    Pair(Pair<Term>),
    Tuple(Tuple<Term>),
    Record(Record<Term>),
    Left(Left<Term>),
    Right(Right<Term>),
    Variant(Variant<Term>),
    Nothing(Nothing<Term>),
    Something(Something<Term>),
    Nil(Nil<Term>),
    Cons(Cons<Term>),
}

impl common::values::Value for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;

    fn into_lambda(self) -> Result<Lambda<Term>, ErrorKind> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
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

    fn into_pair(self) -> Result<Pair<Term>, ErrorKind> {
        if let Value::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Pair".to_owned(),
            })
        }
    }

    fn into_tuple(self) -> Result<Tuple<Term>, ErrorKind> {
        if let Value::Tuple(tup) = self {
            Ok(tup)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Tuple".to_owned(),
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

    fn into_left(self) -> Result<Left<Term>, ErrorKind> {
        if let Value::Left(lft) = self {
            Ok(lft)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Left".to_owned(),
            })
        }
    }

    fn into_right(self) -> Result<Right<Term>, ErrorKind> {
        if let Value::Right(right) = self {
            Ok(right)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Right".to_owned(),
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

    fn into_nothing(self) -> Result<Nothing<Term>, ErrorKind> {
        if let Value::Nothing(not) = self {
            Ok(not)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Nothing".to_owned(),
            })
        }
    }

    fn into_something(self) -> Result<Something<Term>, ErrorKind> {
        if let Value::Something(somet) = self {
            Ok(somet)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Something".to_owned(),
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
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Pair(pair) => pair.into_term().into(),
            Value::Tuple(tup) => tup.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::Left(lft) => lft.into_term().into(),
            Value::Right(right) => right.into_term().into(),
            Value::Variant(var) => var.into_term().into(),
            Value::Nothing(not) => not.into_term().into(),
            Value::Something(some) => some.into_term().into(),
            Value::Nil(nil) => nil.into_term().into(),
            Value::Cons(cons) => cons.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Pair(pair) => pair.fmt(f),
            Value::Tuple(tup) => tup.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Left(lft) => lft.fmt(f),
            Value::Right(right) => right.fmt(f),
            Value::Variant(var) => var.fmt(f),
            Value::Nothing(not) => not.fmt(f),
            Value::Something(some) => some.fmt(f),
            Value::Nil(nil) => nil.fmt(f),
            Value::Cons(cons) => cons.fmt(f),
        }
    }
}

impl From<Lambda<Term>> for Value {
    fn from(lam: Lambda<Term>) -> Value {
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
impl From<Pair<Term>> for Value {
    fn from(pair: Pair<Term>) -> Value {
        Value::Pair(pair)
    }
}
impl From<Tuple<Term>> for Value {
    fn from(tup: Tuple<Term>) -> Value {
        Value::Tuple(tup)
    }
}
impl From<Record<Term>> for Value {
    fn from(rec: Record<Term>) -> Value {
        Value::Record(rec)
    }
}
impl From<Left<Term>> for Value {
    fn from(lft: Left<Term>) -> Value {
        Value::Left(lft)
    }
}
impl From<Right<Term>> for Value {
    fn from(right: Right<Term>) -> Value {
        Value::Right(right)
    }
}
impl From<Variant<Term>> for Value {
    fn from(var: Variant<Term>) -> Value {
        Value::Variant(var)
    }
}
impl From<Nothing<Term>> for Value {
    fn from(not: Nothing<Term>) -> Value {
        Value::Nothing(not)
    }
}
impl From<Something<Term>> for Value {
    fn from(some: Something<Term>) -> Value {
        Value::Something(some)
    }
}
impl From<Nil<Term>> for Value {
    fn from(nil: Nil<Term>) -> Value {
        Value::Nil(nil)
    }
}
impl From<Cons<Term>> for Value {
    fn from(cons: Cons<Term>) -> Value {
        Value::Cons(cons)
    }
}
