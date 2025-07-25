use super::{terms::Term, types::Type};
use common::errors::{ValueKind, ValueMismatch};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{False, Lambda, Num, True, TyLambda, Unit, Value as ValueTrait, ValueGroup};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    Num(Num<Term>),
    Lambda(Lambda<Term, Type>),
    TyLambda(TyLambda<Term>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::Unit(v) => v.knd(),
            Value::True(v) => v.knd(),
            Value::False(v) => v.knd(),
            Value::Num(v) => v.knd(),
            Value::Lambda(v) => v.knd(),
            Value::TyLambda(v) => v.knd(),
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

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Number))
        }
    }

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Lambda))
        }
    }

    fn into_tylambda(self) -> Result<TyLambda<Term>, ValueMismatch> {
        if let Value::TyLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::TyLambda))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Term, Type>::rule(),
            TyLambda::<Term>::rule(),
            Unit::<Term>::rule(),
            True::<Term>::rule(),
            False::<Term>::rule(),
            Num::<Term>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::TyLambda(tylam) => tylam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::TyLambda(tylam) => tylam.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::TyLambda(tylam) => tylam.to_latex(conf),
            Value::Unit(u) => u.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
        }
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
impl From<Lambda<Term, Type>> for Value {
    fn from(lam: Lambda<Term, Type>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<TyLambda<Term>> for Value {
    fn from(tylam: TyLambda<Term>) -> Value {
        Value::TyLambda(tylam)
    }
}
