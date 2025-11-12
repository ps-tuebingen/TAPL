use super::{FOmega, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{LangDisplay, LatexFmt};
use syntax::values::{
    False, Lambda, Num, Pack, Record, True, TyLambda, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<FOmega>),
    TyLambda(TyLambda<FOmega>),
    Pack(Pack<FOmega>),
    Record(Record<FOmega>),
    True(True<FOmega>),
    False(False<FOmega>),
    Unit(Unit<FOmega>),
    Num(Num<FOmega>),
}

impl ValueTrait for Value {
    type Lang = FOmega;
    type Term = Term;
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<FOmega>::rule(),
            TyLambda::<FOmega>::rule(),
            Pack::<FOmega>::rule(),
            Record::<FOmega>::rule(),
            True::<FOmega>::rule(),
            False::<FOmega>::rule(),
            Unit::<FOmega>::rule(),
            Num::<FOmega>::rule(),
        ])
    }
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<FOmega>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_tylambda(self) -> Result<TyLambda<FOmega>, ValueMismatch> {
        if let Value::TyLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "TyLambda".to_owned()))
        }
    }

    fn into_pack(self) -> Result<Pack<FOmega>, ValueMismatch> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Package".to_owned()))
        }
    }
    fn into_record(self) -> Result<Record<FOmega>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<FOmega>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }
    fn into_false(self) -> Result<False<FOmega>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
    fn into_num(self) -> Result<Num<FOmega>, ValueMismatch> {
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
            Value::TyLambda(tylam) => tylam.into_term(),
            Value::Pack(pack) => pack.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Unit(u) => u.into_term(),
            Value::Num(num) => num.into_term(),
        }
    }
}

impl From<Pack<FOmega>> for Value {
    fn from(pack: Pack<FOmega>) -> Value {
        Value::Pack(pack)
    }
}
impl From<TyLambda<FOmega>> for Value {
    fn from(lam: TyLambda<FOmega>) -> Value {
        Value::TyLambda(lam)
    }
}
impl From<Lambda<FOmega>> for Value {
    fn from(lam: Lambda<FOmega>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<FOmega>> for Value {
    fn from(u: Unit<FOmega>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<FOmega>> for Value {
    fn from(tru: True<FOmega>) -> Value {
        Value::True(tru)
    }
}
impl From<False<FOmega>> for Value {
    fn from(fls: False<FOmega>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<FOmega>> for Value {
    fn from(num: Num<FOmega>) -> Value {
        Value::Num(num)
    }
}

impl From<Record<FOmega>> for Value {
    fn from(rec: Record<FOmega>) -> Value {
        Value::Record(rec)
    }
}
