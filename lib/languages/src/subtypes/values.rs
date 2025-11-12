use super::{Subtypes, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt};
use syntax::values::{
    Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, ValueGroup,
    Variant,
};

#[derive(FromVariants, LatexFmt, LangDisplay, Debug, PartialEq, Eq, Clone)]
#[Lang(Subtypes)]
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
