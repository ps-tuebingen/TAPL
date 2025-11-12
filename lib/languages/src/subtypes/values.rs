use super::{Subtypes, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{LangDisplay, LatexFmt};
use syntax::values::{
    Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, ValueGroup,
    Variant,
};

#[derive(LatexFmt, LangDisplay, Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Lambda(Lambda<Subtypes>),
    Unit(Unit<Subtypes>),
    Record(Record<Subtypes>),
    Variant(Variant<Subtypes>),
    Nil(Nil<Subtypes>),
    Cons(Cons<Subtypes>),
    Loc(Loc<Subtypes>),
    Num(Num<Subtypes>),
    True(True<Subtypes>),
    False(False<Subtypes>),
}

impl ValueTrait for Value {
    type Lang = Subtypes;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<Subtypes>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Subtypes>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Subtypes>, ValueMismatch> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_nil(self) -> Result<Nil<Subtypes>, ValueMismatch> {
        if let Value::Nil(nil) = self {
            Ok(nil)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Nil".to_owned()))
        }
    }

    fn into_cons(self) -> Result<Cons<Subtypes>, ValueMismatch> {
        if let Value::Cons(cons) = self {
            Ok(cons)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Cons".to_owned()))
        }
    }

    fn into_loc(self) -> Result<Loc<Subtypes>, ValueMismatch> {
        if let Value::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Location".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Subtypes>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<Subtypes>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Subtypes>, ValueMismatch> {
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
            Value::Unit(u) => u.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::Variant(var) => var.into_term(),
            Value::Nil(nil) => nil.into_term(),
            Value::Cons(cons) => cons.into_term(),
            Value::Loc(loc) => loc.into_term(),
            Value::Num(num) => num.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Subtypes>::rule(),
            Unit::<Subtypes>::rule(),
            Record::<Subtypes>::rule(),
            Variant::<Subtypes>::rule(),
            Nil::<Subtypes>::rule(),
            Cons::<Subtypes>::rule(),
            Loc::<Subtypes>::rule(),
            Num::<Subtypes>::rule(),
            True::<Subtypes>::rule(),
            False::<Subtypes>::rule(),
        ])
    }
}

impl From<Loc<Subtypes>> for Value {
    fn from(loc: Loc<Subtypes>) -> Value {
        Value::Loc(loc)
    }
}
impl From<Lambda<Subtypes>> for Value {
    fn from(lam: Lambda<Subtypes>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Unit<Subtypes>> for Value {
    fn from(u: Unit<Subtypes>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<Subtypes>> for Value {
    fn from(tru: True<Subtypes>) -> Value {
        Value::True(tru)
    }
}
impl From<False<Subtypes>> for Value {
    fn from(fls: False<Subtypes>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<Subtypes>> for Value {
    fn from(num: Num<Subtypes>) -> Value {
        Value::Num(num)
    }
}

impl From<Record<Subtypes>> for Value {
    fn from(rec: Record<Subtypes>) -> Value {
        Value::Record(rec)
    }
}

impl From<Variant<Subtypes>> for Value {
    fn from(var: Variant<Subtypes>) -> Value {
        Value::Variant(var)
    }
}

impl From<Nil<Subtypes>> for Value {
    fn from(nil: Nil<Subtypes>) -> Value {
        Value::Nil(nil)
    }
}
impl From<Cons<Subtypes>> for Value {
    fn from(cons: Cons<Subtypes>) -> Value {
        Value::Cons(cons)
    }
}
