use super::{terms::Term, types::Type};
use common::errors::{ValueKind, ValueMismatch};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait, ValueGroup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    LambdaSub(LambdaSub<Term, Type>),
    Pack(Pack<Value, Type>),
    Num(Num<Term>),
    Record(Record<Value>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::Lambda(v) => v.knd(),
            Value::LambdaSub(v) => v.knd(),
            Value::Pack(v) => v.knd(),
            Value::Num(v) => v.knd(),
            Value::Record(v) => v.knd(),
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

    fn into_lambdasub(self) -> Result<LambdaSub<Term, Type>, ValueMismatch> {
        if let Value::LambdaSub(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::LambdaSub))
        }
    }

    fn into_pack(self) -> Result<Pack<Value, Type>, ValueMismatch> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Package))
        }
    }

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Number))
        }
    }

    fn into_record(self) -> Result<Record<Value>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Record))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Term, Type>::rule(),
            LambdaSub::<Term, Type>::rule(),
            Pack::<Value, Type>::rule(),
            Num::<Term>::rule(),
            Record::<Value>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::LambdaSub(lam) => lam.into_term().into(),
            Value::Pack(pack) => pack.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
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

impl From<LambdaSub<Term, Type>> for Value {
    fn from(lam: LambdaSub<Term, Type>) -> Value {
        Value::LambdaSub(lam)
    }
}

impl From<Pack<Value, Type>> for Value {
    fn from(pack: Pack<Value, Type>) -> Value {
        Value::Pack(pack)
    }
}

impl From<Record<Value>> for Value {
    fn from(rec: Record<Value>) -> Value {
        Value::Record(rec)
    }
}
