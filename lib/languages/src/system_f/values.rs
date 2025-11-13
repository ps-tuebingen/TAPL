use super::{SystemF, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{Lambda, TyLambda, Value as ValueTrait, ValueGroup};

#[derive(
    IntoTerm, GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Debug, PartialEq, Eq, Clone,
)]
#[Lang(SystemF)]
pub enum Value {
    Lambda(Lambda<SystemF>),
    TyLambda(TyLambda<SystemF>),
}

impl ValueTrait for Value {
    type Lang = SystemF;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<SystemF>, ValueMismatch> {
        if let Self::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_tylambda(self) -> Result<TyLambda<SystemF>, ValueMismatch> {
        if let Self::TyLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "TyLambda".to_owned()))
        }
    }
}
