use super::{Recursive, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{
    False, Fold, Lambda, Num, Pair, Record, True, Unit, Value as ValueTrait, ValueGroup, Variant,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<Recursive>),
    False(False<Recursive>),
    Unit(Unit<Recursive>),
    Num(Num<Recursive>),
    Lambda(Lambda<Recursive>),
    Fold(Fold<Recursive>),
    Pair(Pair<Recursive>),
    Record(Record<Recursive>),
    Variant(Variant<Recursive>),
}

impl ValueTrait for Value {
    type Lang = Recursive;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Result<True<Recursive>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Recursive>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Recursive>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_lambda(self) -> Result<Lambda<Recursive>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_fold(self) -> Result<Fold<Recursive>, ValueMismatch> {
        if let Value::Fold(fld) = self {
            Ok(fld)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Fold".to_owned()))
        }
    }

    fn into_pair(self) -> Result<Pair<Recursive>, ValueMismatch> {
        if let Value::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Pair".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Recursive>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Recursive>, ValueMismatch> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Unit(u) => u.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Lambda(lam) => lam.into_term(),
            Value::Fold(fld) => fld.into_term(),
            Value::Pair(pr) => pr.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::Variant(var) => var.into_term(),
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Unit::<Recursive>::rule(),
            True::<Recursive>::rule(),
            False::<Recursive>::rule(),
            Num::<Recursive>::rule(),
            Lambda::<Recursive>::rule(),
            Fold::<Recursive>::rule(),
            Pair::<Recursive>::rule(),
            Record::<Recursive>::rule(),
            Variant::<Recursive>::rule(),
        ])
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit(u) => u.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Lambda(lam) => lam.fmt(f),
            Value::Fold(fld) => fld.fmt(f),
            Value::Pair(pr) => pr.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Variant(var) => var.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Unit(u) => u.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Fold(fld) => fld.to_latex(conf),
            Value::Pair(pr) => pr.to_latex(conf),
            Value::Record(rec) => rec.to_latex(conf),
            Value::Variant(var) => var.to_latex(conf),
        }
    }
}

impl From<Fold<Recursive>> for Value {
    fn from(fld: Fold<Recursive>) -> Value {
        Value::Fold(fld)
    }
}

impl From<Lambda<Recursive>> for Value {
    fn from(lam: Lambda<Recursive>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<Recursive>> for Value {
    fn from(u: Unit<Recursive>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<Recursive>> for Value {
    fn from(tru: True<Recursive>) -> Value {
        Value::True(tru)
    }
}
impl From<False<Recursive>> for Value {
    fn from(fls: False<Recursive>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<Recursive>> for Value {
    fn from(num: Num<Recursive>) -> Value {
        Value::Num(num)
    }
}
impl From<Pair<Recursive>> for Value {
    fn from(pair: Pair<Recursive>) -> Value {
        Value::Pair(pair)
    }
}

impl From<Record<Recursive>> for Value {
    fn from(rec: Record<Recursive>) -> Value {
        Value::Record(rec)
    }
}

impl From<Variant<Recursive>> for Value {
    fn from(var: Variant<Recursive>) -> Value {
        Value::Variant(var)
    }
}
