use super::{terms::Term, types::Type};
use common::errors::{ValueKind, ValueMismatch};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{
    Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, ValueGroup,
    Variant,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Unit(Unit<Term>),
    Record(Record<Value>),
    Variant(Variant<Value, Type>),
    Nil(Nil<Term, Type>),
    Cons(Cons<Value, Type>),
    Loc(Loc<Term>),
    Num(Num<Term>),
    True(True<Term>),
    False(False<Term>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::Lambda(v) => v.knd(),
            Value::Unit(v) => v.knd(),
            Value::Record(v) => v.knd(),
            Value::Variant(v) => v.knd(),
            Value::Nil(v) => v.knd(),
            Value::Cons(v) => v.knd(),
            Value::Loc(v) => v.knd(),
            Value::Num(v) => v.knd(),
            Value::True(v) => v.knd(),
            Value::False(v) => v.knd(),
        }
    }
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Type;

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Lambda))
        }
    }

    fn into_record(self) -> Result<Record<Value>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Record))
        }
    }

    fn into_variant(self) -> Result<Variant<Value, Type>, ValueMismatch> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Variant))
        }
    }

    fn into_nil(self) -> Result<Nil<Term, Type>, ValueMismatch> {
        if let Value::Nil(nil) = self {
            Ok(nil)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Nil))
        }
    }

    fn into_cons(self) -> Result<Cons<Value, Type>, ValueMismatch> {
        if let Value::Cons(cons) = self {
            Ok(cons)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Cons))
        }
    }

    fn into_loc(self) -> Result<Loc<Term>, ValueMismatch> {
        if let Value::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Location))
        }
    }

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Number))
        }
    }

    fn into_true(self) -> Result<True<Term>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::True))
        }
    }

    fn into_false(self) -> Result<False<Term>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::False))
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

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Term, Type>::rule(),
            Unit::<Term>::rule(),
            Record::<Value>::rule(),
            Variant::<Value, Type>::rule(),
            Nil::<Term, Type>::rule(),
            Cons::<Value, Type>::rule(),
            Loc::<Term>::rule(),
            Num::<Term>::rule(),
            True::<Term>::rule(),
            False::<Term>::rule(),
        ])
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

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Unit(u) => u.to_latex(conf),
            Value::Record(rec) => rec.to_latex(conf),
            Value::Variant(var) => var.to_latex(conf),
            Value::Nil(nil) => nil.to_latex(conf),
            Value::Cons(cons) => cons.to_latex(conf),
            Value::Loc(loc) => loc.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
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

impl From<Record<Value>> for Value {
    fn from(rec: Record<Value>) -> Value {
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
