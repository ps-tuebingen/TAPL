use super::{FOmegaSub, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait, ValueGroup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<FOmegaSub>),
    LambdaSub(LambdaSub<FOmegaSub>),
    Pack(Pack<FOmegaSub>),
    Record(Record<FOmegaSub>),
    Num(Num<FOmegaSub>),
}

impl ValueTrait for Value {
    type Lang = FOmegaSub;
    type Term = Term;
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<FOmegaSub>::rule(),
            LambdaSub::<FOmegaSub>::rule(),
            Pack::<FOmegaSub>::rule(),
            Record::<FOmegaSub>::rule(),
            Num::<FOmegaSub>::rule(),
        ])
    }
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<FOmegaSub>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_lambdasub(self) -> Result<LambdaSub<FOmegaSub>, ValueMismatch> {
        if let Value::LambdaSub(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "LambdaSub".to_owned()))
        }
    }

    fn into_pack(self) -> Result<Pack<FOmegaSub>, ValueMismatch> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Package".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<FOmegaSub>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<FOmegaSub>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term(),
            Value::LambdaSub(lam) => lam.into_term(),
            Value::Pack(pack) => pack.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::Num(num) => num.into_term(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::LambdaSub(lam) => lam.fmt(f),
            Value::Pack(pack) => pack.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Num(num) => num.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::LambdaSub(lam) => lam.to_latex(conf),
            Value::Pack(pack) => pack.to_latex(conf),
            Value::Record(rec) => rec.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
        }
    }
}

impl From<Pack<FOmegaSub>> for Value {
    fn from(pack: Pack<FOmegaSub>) -> Value {
        Value::Pack(pack)
    }
}
impl From<LambdaSub<FOmegaSub>> for Value {
    fn from(lam: LambdaSub<FOmegaSub>) -> Value {
        Value::LambdaSub(lam)
    }
}

impl From<Lambda<FOmegaSub>> for Value {
    fn from(lam: Lambda<FOmegaSub>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Num<FOmegaSub>> for Value {
    fn from(num: Num<FOmegaSub>) -> Value {
        Value::Num(num)
    }
}

impl From<Record<FOmegaSub>> for Value {
    fn from(rec: Record<FOmegaSub>) -> Value {
        Value::Record(rec)
    }
}
