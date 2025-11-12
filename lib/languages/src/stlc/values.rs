use super::{Stlc, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt};
use syntax::values::{
    Cons, False, Lambda, Left, Nil, Nothing, Num, Pair, Record, Right, Something, True, Tuple,
    Unit, Value as ValueTrait, ValueGroup, Variant,
};

#[derive(FromVariants, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq)]
#[Lang(Stlc)]
pub enum Value {
    Lambda(Lambda<Stlc>),
    Unit(Unit<Stlc>),
    True(True<Stlc>),
    False(False<Stlc>),
    Num(Num<Stlc>),
    Pair(Pair<Stlc>),
    Tuple(Tuple<Stlc>),
    Record(Record<Stlc>),
    Left(Left<Stlc>),
    Right(Right<Stlc>),
    Variant(Variant<Stlc>),
    Nothing(Nothing<Stlc>),
    Something(Something<Stlc>),
    Nil(Nil<Stlc>),
    Cons(Cons<Stlc>),
}

impl ValueTrait for Value {
    type Lang = Stlc;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<Stlc>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_true(self) -> Result<True<Stlc>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
        }
    }

    fn into_false(self) -> Result<False<Stlc>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
        }
    }

    fn into_num(self) -> Result<Num<Stlc>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
        }
    }

    fn into_pair(self) -> Result<Pair<Stlc>, ValueMismatch> {
        if let Value::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Pair".to_owned()))
        }
    }

    fn into_tuple(self) -> Result<Tuple<Stlc>, ValueMismatch> {
        if let Value::Tuple(tup) = self {
            Ok(tup)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Tuple".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Stlc>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_left(self) -> Result<Left<Stlc>, ValueMismatch> {
        if let Value::Left(lft) = self {
            Ok(lft)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Left".to_owned()))
        }
    }

    fn into_right(self) -> Result<Right<Stlc>, ValueMismatch> {
        if let Value::Right(right) = self {
            Ok(right)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Right".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Stlc>, ValueMismatch> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_nothing(self) -> Result<Nothing<Stlc>, ValueMismatch> {
        if let Value::Nothing(not) = self {
            Ok(not)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Nothing".to_owned()))
        }
    }

    fn into_something(self) -> Result<Something<Stlc>, ValueMismatch> {
        if let Value::Something(somet) = self {
            Ok(somet)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Something".to_owned()))
        }
    }

    fn into_nil(self) -> Result<Nil<Stlc>, ValueMismatch> {
        if let Value::Nil(nil) = self {
            Ok(nil)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Nil".to_owned()))
        }
    }

    fn into_cons(self) -> Result<Cons<Stlc>, ValueMismatch> {
        if let Value::Cons(cons) = self {
            Ok(cons)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Cons".to_owned()))
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![
            Lambda::<Stlc>::rule(),
            Unit::<Stlc>::rule(),
            True::<Stlc>::rule(),
            False::<Stlc>::rule(),
            Num::<Stlc>::rule(),
            Pair::<Stlc>::rule(),
            Tuple::<Stlc>::rule(),
            Record::<Stlc>::rule(),
            Left::<Stlc>::rule(),
            Right::<Stlc>::rule(),
            Variant::<Stlc>::rule(),
            Nothing::<Stlc>::rule(),
            Something::<Stlc>::rule(),
            Nil::<Stlc>::rule(),
            Cons::<Stlc>::rule(),
        ])
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Lambda(lam) => lam.into_term(),
            Value::Unit(u) => u.into_term(),
            Value::True(tru) => tru.into_term(),
            Value::False(fls) => fls.into_term(),
            Value::Num(num) => num.into_term(),
            Value::Pair(pair) => pair.into_term(),
            Value::Tuple(tup) => tup.into_term(),
            Value::Record(rec) => rec.into_term(),
            Value::Left(lft) => lft.into_term(),
            Value::Right(right) => right.into_term(),
            Value::Variant(var) => var.into_term(),
            Value::Nothing(not) => not.into_term(),
            Value::Something(some) => some.into_term(),
            Value::Nil(nil) => nil.into_term(),
            Value::Cons(cons) => cons.into_term(),
        }
    }
}
