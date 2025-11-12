use super::{FOmegaSub, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, LangDisplay, LatexFmt};
use syntax::values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait, ValueGroup};

#[derive(GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq)]
#[Lang(FOmegaSub)]
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
