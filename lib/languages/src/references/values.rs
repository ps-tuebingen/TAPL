use super::{References, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{False, Lambda, Loc, Num, True, Unit, Value as ValueTrait, ValueGroup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<References>),
    Unit(Unit<References>),
    Num(Num<References>),
    Loc(Loc<References>),
    True(True<References>),
    False(False<References>),
}

impl ValueTrait for Value {
    type Lang = References;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<References>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<References>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_loc(self) -> Result<Loc<References>, ValueMismatch> {
        if let Value::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Location".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<References>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<References>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Unit(u) => u.into_term(),
            Value::Loc(loc) => loc.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<References>::rule(),
            Num::<References>::rule(),
            Unit::<References>::rule(),
            Loc::<References>::rule(),
            True::<References>::rule(),
            False::<References>::rule(),
        ])
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::Loc(loc) => loc.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Unit(u) => u.to_latex(conf),
            Value::Loc(loc) => loc.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
        }
    }
}

impl From<Lambda<References>> for Value {
    fn from(lam: Lambda<References>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<References>> for Value {
    fn from(u: Unit<References>) -> Value {
        Value::Unit(u)
    }
}
impl From<Num<References>> for Value {
    fn from(n: Num<References>) -> Value {
        Value::Num(n)
    }
}
impl From<Loc<References>> for Value {
    fn from(loc: Loc<References>) -> Value {
        Value::Loc(loc)
    }
}

impl From<True<References>> for Value {
    fn from(tru: True<References>) -> Value {
        Value::True(tru)
    }
}

impl From<False<References>> for Value {
    fn from(fls: False<References>) -> Value {
        Value::False(fls)
    }
}
