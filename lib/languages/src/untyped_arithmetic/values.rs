use super::{UntypedArithmetic, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{False, Num, True, Value as ValueTrait, ValueGroup};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<UntypedArithmetic>),
    False(False<UntypedArithmetic>),
    Num(Num<UntypedArithmetic>),
}

impl ValueTrait for Value {
    type Lang = UntypedArithmetic;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Result<True<UntypedArithmetic>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<UntypedArithmetic>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<UntypedArithmetic>, ValueMismatch> {
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
            True::<UntypedArithmetic>::rule(),
            False::<UntypedArithmetic>::rule(),
            Num::<UntypedArithmetic>::rule(),
        ])
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
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

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
        }
    }
}

impl From<True<UntypedArithmetic>> for Value {
    fn from(tru: True<UntypedArithmetic>) -> Value {
        Value::True(tru)
    }
}

impl From<False<UntypedArithmetic>> for Value {
    fn from(fls: False<UntypedArithmetic>) -> Value {
        Value::False(fls)
    }
}

impl From<Num<UntypedArithmetic>> for Value {
    fn from(num: Num<UntypedArithmetic>) -> Value {
        Value::Num(num)
    }
}
