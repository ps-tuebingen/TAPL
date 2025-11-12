use super::{Exceptions, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::LangDisplay;
use std::fmt;
use syntax::values::{
    Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(LangDisplay, Debug, Clone, PartialEq, Eq)]
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

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Unit(u) => u.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Exception(exc) => exc.to_latex(conf),
            Value::Raise(raise) => raise.to_latex(conf),
        }
    }
}

impl From<Lambda<Exceptions>> for Value {
    fn from(lam: Lambda<Exceptions>) -> Value {
        Value::Lambda(lam)
    }
}

impl From<Num<Exceptions>> for Value {
    fn from(num: Num<Exceptions>) -> Value {
        Value::Num(num)
    }
}

impl From<Unit<Exceptions>> for Value {
    fn from(unit: Unit<Exceptions>) -> Value {
        Value::Unit(unit)
    }
}

impl From<True<Exceptions>> for Value {
    fn from(tru: True<Exceptions>) -> Value {
        Value::True(tru)
    }
}

impl From<False<Exceptions>> for Value {
    fn from(fls: False<Exceptions>) -> Value {
        Value::False(fls)
    }
}

impl From<Exception<Exceptions>> for Value {
    fn from(exc: Exception<Exceptions>) -> Value {
        Value::Exception(exc)
    }
}

impl From<Raise<Exceptions>> for Value {
    fn from(raise: Raise<Exceptions>) -> Value {
        Value::Raise(raise)
    }
}
