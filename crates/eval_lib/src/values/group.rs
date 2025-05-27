use crate::values::{
    Cons, Exception, False, Fold, Lambda, LambdaSub, Left, Loc, Nil, Nothing, Num, Pack, Pair,
    Raise, Record, Right, Something, True, Tuple, TyLambda, Value, Variant,
};
use common::errors::ErrorKind;
use syntax::types::Type;

pub trait ValueGroup: Value {
    type Type: Type;

    fn into_lambda(self) -> Result<Lambda<<Self as Value>::Term, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_tylambda(self) -> Result<TyLambda<<Self as Value>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Type Abstraction".to_owned(),
        })
    }

    fn into_lambdasub(self) -> Result<LambdaSub<<Self as Value>::Term, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Type Abstraction".to_owned(),
        })
    }

    fn into_pair(self) -> Result<Pair<Self>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Pair".to_owned(),
        })
    }

    fn into_record(self) -> Result<Record<Self>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Record".to_owned(),
        })
    }

    fn into_tuple(self) -> Result<Tuple<Self>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Tuple".to_owned(),
        })
    }

    fn into_left(self) -> Result<Left<Self, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Left".to_owned(),
        })
    }

    fn into_right(self) -> Result<Right<Self, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Right".to_owned(),
        })
    }

    fn into_variant(self) -> Result<Variant<Self, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Variant".to_owned(),
        })
    }

    fn into_nothing(self) -> Result<Nothing<<Self as Value>::Term, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Nothing".to_owned(),
        })
    }

    fn into_something(self) -> Result<Something<Self>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Something".to_owned(),
        })
    }

    fn into_nil(self) -> Result<Nil<<Self as Value>::Term, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Nil".to_owned(),
        })
    }

    fn into_cons(self) -> Result<Cons<Self::Type, Self>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Cons".to_owned(),
        })
    }

    fn into_loc(self) -> Result<Loc<<Self as Value>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Location".to_owned(),
        })
    }

    fn into_raise(self) -> Result<Raise<Self, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }

    fn into_exception(self) -> Result<Exception<<Self as Value>::Term, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Exception".to_owned(),
        })
    }

    fn into_fold(self) -> Result<Fold<Self, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Fold".to_owned(),
        })
    }

    fn into_pack(self) -> Result<Pack<Self, Self::Type>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Package".to_owned(),
        })
    }

    fn into_true(self) -> Result<True<<Self as Value>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "True".to_owned(),
        })
    }

    fn into_false(self) -> Result<False<<Self as Value>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "False".to_owned(),
        })
    }

    fn into_num(self) -> Result<Num<<Self as Value>::Term>, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Number".to_owned(),
        })
    }
}
