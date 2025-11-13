use super::{Recursive, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{
    False, Fold, Lambda, Num, Pair, Record, True, Unit, Value as ValueTrait, ValueGroup, Variant,
};

#[derive(
    IntoTerm, GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq,
)]
#[Lang(Recursive)]
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
        if let Self::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Recursive>, ValueMismatch> {
        if let Self::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Recursive>, ValueMismatch> {
        if let Self::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_lambda(self) -> Result<Lambda<Recursive>, ValueMismatch> {
        if let Self::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_fold(self) -> Result<Fold<Recursive>, ValueMismatch> {
        if let Self::Fold(fld) = self {
            Ok(fld)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Fold".to_owned()))
        }
    }

    fn into_pair(self) -> Result<Pair<Recursive>, ValueMismatch> {
        if let Self::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Pair".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Recursive>, ValueMismatch> {
        if let Self::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Recursive>, ValueMismatch> {
        if let Self::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }
}
