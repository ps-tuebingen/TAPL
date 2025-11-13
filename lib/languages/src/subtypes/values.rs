use super::{Subtypes, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{
    Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, ValueGroup,
    Variant,
};

#[derive(
    IntoTerm, GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, PartialEq, Eq, Clone,
)]
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
        if let Self::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Subtypes>, ValueMismatch> {
        if let Self::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Subtypes>, ValueMismatch> {
        if let Self::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_nil(self) -> Result<Nil<Subtypes>, ValueMismatch> {
        if let Self::Nil(nil) = self {
            Ok(nil)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Nil".to_owned()))
        }
    }

    fn into_cons(self) -> Result<Cons<Subtypes>, ValueMismatch> {
        if let Self::Cons(cons) = self {
            Ok(cons)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Cons".to_owned()))
        }
    }

    fn into_loc(self) -> Result<Loc<Subtypes>, ValueMismatch> {
        if let Self::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Location".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Subtypes>, ValueMismatch> {
        if let Self::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<Subtypes>, ValueMismatch> {
        if let Self::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Subtypes>, ValueMismatch> {
        if let Self::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
}
