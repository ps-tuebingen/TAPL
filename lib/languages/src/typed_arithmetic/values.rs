use super::{TypedArithmetic, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::LangDisplay;
use std::fmt;
use syntax::values::{False, Num, True, Value as ValueTrait, ValueGroup};

#[derive(LangDisplay, Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<TypedArithmetic>),
    False(False<TypedArithmetic>),
    Num(Num<TypedArithmetic>),
}

impl ValueTrait for Value {
    type Lang = TypedArithmetic;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Result<True<TypedArithmetic>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }
    fn into_false(self) -> Result<False<TypedArithmetic>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }
    fn into_num(self) -> Result<Num<TypedArithmetic>, ValueMismatch> {
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
            True::<TypedArithmetic>::rule(),
            False::<TypedArithmetic>::rule(),
            Num::<TypedArithmetic>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
        }
    }
}

impl From<True<TypedArithmetic>> for Value {
    fn from(tru: True<TypedArithmetic>) -> Value {
        Value::True(tru)
    }
}
impl From<False<TypedArithmetic>> for Value {
    fn from(fls: False<TypedArithmetic>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<TypedArithmetic>> for Value {
    fn from(num: Num<TypedArithmetic>) -> Value {
        Value::Num(num)
    }
}
