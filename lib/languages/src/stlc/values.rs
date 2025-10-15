use super::{Stlc, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{
    Cons, False, Lambda, Left, Nil, Nothing, Num, Pair, Record, Right, Something, True, Tuple,
    Unit, Value as ValueTrait, ValueGroup, Variant,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Stlc>),
    Unit(Unit<Stlc>),
    True(True<Stlc>),
    False(False<Stlc>),
    Num(Num<Stlc>),
    Pair(Pair<Stlc>),
    Tuple(Tuple<Stlc>),
    Record(Record<Stlc>),
    Left(Left<Stlc>),
    Right(Right<Stlc>),
    Variant(Variant<Stlc>),
    Nothing(Nothing<Stlc>),
    Something(Something<Stlc>),
    Nil(Nil<Stlc>),
    Cons(Cons<Stlc>),
}

impl ValueTrait for Value {
    type Lang = Stlc;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<Stlc>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<Stlc>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Stlc>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Stlc>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_pair(self) -> Result<Pair<Stlc>, ValueMismatch> {
        if let Value::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Pair".to_owned()))
        }
    }

    fn into_tuple(self) -> Result<Tuple<Stlc>, ValueMismatch> {
        if let Value::Tuple(tup) = self {
            Ok(tup)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Tuple".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Stlc>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_left(self) -> Result<Left<Stlc>, ValueMismatch> {
        if let Value::Left(lft) = self {
            Ok(lft)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Left".to_owned()))
        }
    }

    fn into_right(self) -> Result<Right<Stlc>, ValueMismatch> {
        if let Value::Right(right) = self {
            Ok(right)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Right".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Stlc>, ValueMismatch> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_nothing(self) -> Result<Nothing<Stlc>, ValueMismatch> {
        if let Value::Nothing(not) = self {
            Ok(not)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Nothing".to_owned()))
        }
    }

    fn into_something(self) -> Result<Something<Stlc>, ValueMismatch> {
        if let Value::Something(somet) = self {
            Ok(somet)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Something".to_owned()))
        }
    }

    fn into_nil(self) -> Result<Nil<Stlc>, ValueMismatch> {
        if let Value::Nil(nil) = self {
            Ok(nil)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Nil".to_owned()))
        }
    }

    fn into_cons(self) -> Result<Cons<Stlc>, ValueMismatch> {
        if let Value::Cons(cons) = self {
            Ok(cons)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Cons".to_owned()))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Stlc>::rule(),
            Unit::<Stlc>::rule(),
            True::<Stlc>::rule(),
            False::<Stlc>::rule(),
            Num::<Stlc>::rule(),
            Pair::<Stlc>::rule(),
            Tuple::<Stlc>::rule(),
            Record::<Stlc>::rule(),
            Left::<Stlc>::rule(),
            Right::<Stlc>::rule(),
            Variant::<Stlc>::rule(),
            Nothing::<Stlc>::rule(),
            Something::<Stlc>::rule(),
            Nil::<Stlc>::rule(),
            Cons::<Stlc>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Lambda(lam) => lam.into_term(),
            Value::Unit(u) => u.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Pair(pair) => pair.into_term(),
            Value::Tuple(tup) => tup.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::Left(lft) => lft.into_term(),
            Value::Right(right) => right.into_term(),
            Value::Variant(var) => var.into_term(),
            Value::Nothing(not) => not.into_term(),
            Value::Something(some) => some.into_term(),
            Value::Nil(nil) => nil.into_term(),
            Value::Cons(cons) => cons.into_term(),
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

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Unit(u) => u.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Pair(pair) => pair.to_latex(conf),
            Value::Tuple(tup) => tup.to_latex(conf),
            Value::Record(rec) => rec.to_latex(conf),
            Value::Left(lft) => lft.to_latex(conf),
            Value::Right(right) => right.to_latex(conf),
            Value::Variant(var) => var.to_latex(conf),
            Value::Nothing(not) => not.to_latex(conf),
            Value::Something(some) => some.to_latex(conf),
            Value::Nil(nil) => nil.to_latex(conf),
            Value::Cons(cons) => cons.to_latex(conf),
        }
    }
}

impl From<Lambda<Stlc>> for Value {
    fn from(lam: Lambda<Stlc>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<Stlc>> for Value {
    fn from(u: Unit<Stlc>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<Stlc>> for Value {
    fn from(tru: True<Stlc>) -> Value {
        Value::True(tru)
    }
}
impl From<False<Stlc>> for Value {
    fn from(fls: False<Stlc>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<Stlc>> for Value {
    fn from(num: Num<Stlc>) -> Value {
        Value::Num(num)
    }
}
impl From<Pair<Stlc>> for Value {
    fn from(pair: Pair<Stlc>) -> Value {
        Value::Pair(pair)
    }
}
impl From<Tuple<Stlc>> for Value {
    fn from(tup: Tuple<Stlc>) -> Value {
        Value::Tuple(tup)
    }
}
impl From<Record<Stlc>> for Value {
    fn from(rec: Record<Stlc>) -> Value {
        Value::Record(rec)
    }
}
impl From<Left<Stlc>> for Value {
    fn from(lft: Left<Stlc>) -> Value {
        Value::Left(lft)
    }
}
impl From<Right<Stlc>> for Value {
    fn from(right: Right<Stlc>) -> Value {
        Value::Right(right)
    }
}
impl From<Variant<Stlc>> for Value {
    fn from(var: Variant<Stlc>) -> Value {
        Value::Variant(var)
    }
}
impl From<Nothing<Stlc>> for Value {
    fn from(not: Nothing<Stlc>) -> Value {
        Value::Nothing(not)
    }
}
impl From<Something<Stlc>> for Value {
    fn from(some: Something<Stlc>) -> Value {
        Value::Something(some)
    }
}
impl From<Nil<Stlc>> for Value {
    fn from(nil: Nil<Stlc>) -> Value {
        Value::Nil(nil)
    }
}
impl From<Cons<Stlc>> for Value {
    fn from(cons: Cons<Stlc>) -> Value {
        Value::Cons(cons)
    }
}
