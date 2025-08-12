use crate::{
    terms::Term,
    types::Type,
    values::{
        Cons, Exception, False, Fold, Lambda, LambdaSub, Left, Loc, Nil, Nothing, Num, Pack, Pair,
        Raise, Record, Right, Something, True, Tuple, TyLambda, Value, Variant,
    },
};
use errors::ValueMismatch;

pub trait ValueGroup: Value {
    type Term: Term + From<Self>;
    type Type: Type;

    fn into_lambda(self) -> Result<Lambda<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
    }

    fn into_tylambda(self) -> Result<TyLambda<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "TyLambda".to_owned()))
    }

    fn into_lambdasub(
        self,
    ) -> Result<LambdaSub<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "LambdaSub".to_owned()))
    }

    fn into_pair(self) -> Result<Pair<Self>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Pair".to_owned()))
    }

    fn into_record(self) -> Result<Record<Self>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
    }

    fn into_tuple(self) -> Result<Tuple<Self>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Tuple".to_owned()))
    }

    fn into_left(self) -> Result<Left<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Left".to_owned()))
    }

    fn into_right(self) -> Result<Right<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Right".to_owned()))
    }

    fn into_variant(self) -> Result<Variant<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
    }

    fn into_nothing(
        self,
    ) -> Result<Nothing<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Nothing".to_owned()))
    }

    fn into_something(self) -> Result<Something<Self>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Something".to_owned()))
    }

    fn into_nil(self) -> Result<Nil<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Nil".to_owned()))
    }

    fn into_cons(self) -> Result<Cons<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Cons".to_owned()))
    }

    fn into_loc(self) -> Result<Loc<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Location".to_owned()))
    }

    fn into_raise(self) -> Result<Raise<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Raise".to_owned()))
    }

    fn into_exception(
        self,
    ) -> Result<Exception<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Exception".to_owned()))
    }

    fn into_fold(self) -> Result<Fold<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Fold".to_owned()))
    }

    fn into_pack(self) -> Result<Pack<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Package".to_owned()))
    }

    fn into_true(self) -> Result<True<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
    }

    fn into_false(self) -> Result<False<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
    }

    fn into_num(self) -> Result<Num<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
    }
}
