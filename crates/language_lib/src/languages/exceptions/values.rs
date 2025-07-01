use super::{terms::Term, types::Type};
use common::errors::{ValueKind, ValueMismatch};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{
    Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Num(Num<Term>),
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    Exception(Exception<Term, Type>),
    Raise(Raise<Value, Type>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::Lambda(v) => v.knd(),
            Value::Num(v) => v.knd(),
            Value::Unit(v) => v.knd(),
            Value::True(v) => v.knd(),
            Value::False(v) => v.knd(),
            Value::Exception(v) => v.knd(),
            Value::Raise(v) => v.knd(),
        }
    }
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Type;

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

    fn into_exception(self) -> Result<Exception<Term, Type>, ValueMismatch> {
        if let Value::Exception(ex) = self {
            Ok(ex)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Exception))
        }
    }

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Lambda))
        }
    }

    fn into_raise(self) -> Result<Raise<Value, Type>, ValueMismatch> {
        if let Value::Raise(raise) = self {
            Ok(raise)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Raise))
        }
    }

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Number))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Term, Type>::rule(),
            Unit::<Term>::rule(),
            True::<Term>::rule(),
            False::<Term>::rule(),
            Num::<Term>::rule(),
            Exception::<Term, Type>::rule(),
            Raise::<Value, Type>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Exception(exc) => exc.into_term().into(),
            Value::Raise(raise) => raise.into_term().into(),
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
            Value::Exception(exc) => exc.fmt(f),
            Value::Raise(raise) => raise.fmt(f),
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
            Value::Exception(exc) => exc.to_latex(conf),
            Value::Raise(raise) => raise.to_latex(conf),
        }
    }
}

impl From<Lambda<Term, Type>> for Value {
    fn from(lam: Lambda<Term, Type>) -> Value {
        Value::Lambda(lam)
    }
}

impl From<Num<Term>> for Value {
    fn from(num: Num<Term>) -> Value {
        Value::Num(num)
    }
}

impl From<Unit<Term>> for Value {
    fn from(unit: Unit<Term>) -> Value {
        Value::Unit(unit)
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

impl From<Exception<Term, Type>> for Value {
    fn from(exc: Exception<Term, Type>) -> Value {
        Value::Exception(exc)
    }
}

impl From<Raise<Value, Type>> for Value {
    fn from(raise: Raise<Value, Type>) -> Value {
        Value::Raise(raise)
    }
}
