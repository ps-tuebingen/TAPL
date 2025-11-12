use super::{LambdaOmega, terms::Term};
use errors::ValueMismatch;
use macros::{FromVariants, GrammarDescribe, LangDisplay, LatexFmt};
use syntax::values::{False, Lambda, Num, True, TyLambda, Unit, Value as ValueTrait, ValueGroup};

#[derive(GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Clone, Debug, PartialEq, Eq)]
#[Lang(LambdaOmega)]
pub enum Value {
    Unit(Unit<LambdaOmega>),
    True(True<LambdaOmega>),
    False(False<LambdaOmega>),
    Num(Num<LambdaOmega>),
    Lambda(Lambda<LambdaOmega>),
    TyLambda(TyLambda<LambdaOmega>),
}

impl ValueTrait for Value {
    type Lang = LambdaOmega;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Result<True<LambdaOmega>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<LambdaOmega>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<LambdaOmega>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_lambda(self) -> Result<Lambda<LambdaOmega>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_tylambda(self) -> Result<TyLambda<LambdaOmega>, ValueMismatch> {
        if let Value::TyLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "TyLambda".to_owned()))
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term(),
            Value::TyLambda(tylam) => tylam.into_term(),
            Value::Unit(u) => u.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
        }
    }
}
