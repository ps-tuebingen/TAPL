use super::{Exceptions, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt};
use syntax::values::{
    Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(LatexFmt, FromVariants, LangDisplay, Debug, Clone, PartialEq, Eq)]
#[Lang(Exceptions)]
pub enum Value {
    Lambda(Lambda<Exceptions>),
    Num(Num<Exceptions>),
    Unit(Unit<Exceptions>),
    True(True<Exceptions>),
    False(False<Exceptions>),
    Exception(Exception<Exceptions>),
    Raise(Raise<Exceptions>),
}

impl ValueTrait for Value {
    type Lang = Exceptions;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Result<True<Exceptions>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Exceptions>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_exception(self) -> Result<Exception<Exceptions>, ValueMismatch> {
        if let Value::Exception(ex) = self {
            Ok(ex)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Exception".to_owned()))
        }
    }

    fn into_lambda(self) -> Result<Lambda<Exceptions>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_raise(self) -> Result<Raise<Exceptions>, ValueMismatch> {
        if let Value::Raise(raise) = self {
            Ok(raise)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Raise".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Exceptions>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Exceptions>::rule(),
            Unit::<Exceptions>::rule(),
            True::<Exceptions>::rule(),
            False::<Exceptions>::rule(),
            Num::<Exceptions>::rule(),
            Exception::<Exceptions>::rule(),
            Raise::<Exceptions>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term(),
            Value::Unit(u) => u.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Exception(exc) => exc.into_term(),
            Value::Raise(raise) => raise.into_term(),
        }
    }
}
