use super::{BoundedQuantification, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait, ValueGroup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<BoundedQuantification>),
    LambdaSub(LambdaSub<BoundedQuantification>),
    Pack(Pack<BoundedQuantification>),
    Num(Num<BoundedQuantification>),
    Record(Record<BoundedQuantification>),
}

impl ValueTrait for Value {
    type Lang = BoundedQuantification;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<BoundedQuantification>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_lambdasub(self) -> Result<LambdaSub<BoundedQuantification>, ValueMismatch> {
        if let Value::LambdaSub(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "LambdaSub".to_owned()))
        }
    }

    fn into_pack(self) -> Result<Pack<BoundedQuantification>, ValueMismatch> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Package".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<BoundedQuantification>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<BoundedQuantification>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<BoundedQuantification>::rule(),
            LambdaSub::<BoundedQuantification>::rule(),
            Pack::<BoundedQuantification>::rule(),
            Num::<BoundedQuantification>::rule(),
            Record::<BoundedQuantification>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term(),
            Value::LambdaSub(lam) => lam.into_term(),
            Value::Pack(pack) => pack.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Record(rec) => rec.into_term(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::LambdaSub(lam) => lam.fmt(f),
            Value::Pack(pack) => pack.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Record(rec) => rec.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::LambdaSub(lam) => lam.to_latex(conf),
            Value::Pack(pack) => pack.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Record(rec) => rec.to_latex(conf),
        }
    }
}

impl From<Lambda<BoundedQuantification>> for Value {
    fn from(lam: Lambda<BoundedQuantification>) -> Value {
        Value::Lambda(lam)
    }
}

impl From<Num<BoundedQuantification>> for Value {
    fn from(num: Num<BoundedQuantification>) -> Value {
        Value::Num(num)
    }
}

impl From<LambdaSub<BoundedQuantification>> for Value {
    fn from(lam: LambdaSub<BoundedQuantification>) -> Value {
        Value::LambdaSub(lam)
    }
}

impl From<Pack<BoundedQuantification>> for Value {
    fn from(pack: Pack<BoundedQuantification>) -> Value {
        Value::Pack(pack)
    }
}

impl From<Record<BoundedQuantification>> for Value {
    fn from(rec: Record<BoundedQuantification>) -> Value {
        Value::Record(rec)
    }
}
