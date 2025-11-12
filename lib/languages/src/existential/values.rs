use super::{Existential, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{LangDisplay, LatexFmt};
use syntax::values::{
    False, Lambda, Num, Pack, Record, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Unit(Unit<Existential>),
    Lambda(Lambda<Existential>),
    Pack(Pack<Existential>),
    Num(Num<Existential>),
    Record(Record<Existential>),
    True(True<Existential>),
    False(False<Existential>),
}

impl ValueTrait for Value {
    type Lang = Existential;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<Existential>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_pack(self) -> Result<Pack<Existential>, ValueMismatch> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Package".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Existential>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Existential>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<Existential>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Existential>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Unit::<Existential>::rule(),
            Lambda::<Existential>::rule(),
            Pack::<Existential>::rule(),
            Num::<Existential>::rule(),
            Record::<Existential>::rule(),
            True::<Existential>::rule(),
            False::<Existential>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Unit(u) => u.into_term(),
            Value::Lambda(lam) => lam.into_term(),
            Value::Pack(pack) => pack.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
        }
    }
}

impl From<Lambda<Existential>> for Value {
    fn from(lam: Lambda<Existential>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<Existential>> for Value {
    fn from(u: Unit<Existential>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<Existential>> for Value {
    fn from(tru: True<Existential>) -> Value {
        Value::True(tru)
    }
}
impl From<False<Existential>> for Value {
    fn from(fls: False<Existential>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<Existential>> for Value {
    fn from(num: Num<Existential>) -> Value {
        Value::Num(num)
    }
}
impl From<Record<Existential>> for Value {
    fn from(rec: Record<Existential>) -> Value {
        Value::Record(rec)
    }
}

impl From<Pack<Existential>> for Value {
    fn from(pack: Pack<Existential>) -> Value {
        Value::Pack(pack)
    }
}
