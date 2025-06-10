use crate::{
    errors::{ValueKind, ValueMismatch},
    values::{
        Cons, Exception, False, Fold, Lambda, LambdaSub, Left, Loc, Nil, Nothing, Num, Pack, Pair,
        Raise, Record, Right, Something, True, Tuple, TyLambda, Value, Variant,
    },
};
use syntax::{terms::Term, types::Type};

pub trait ValueGroup: Value {
    type Term: Term + From<Self>;
    type Type: Type;

    fn into_lambda(self) -> Result<Lambda<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Lambda))
    }

    fn into_tylambda(self) -> Result<TyLambda<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::TyLambda))
    }

    fn into_lambdasub(
        self,
    ) -> Result<LambdaSub<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::LambdaSub))
    }

    fn into_pair(self) -> Result<Pair<Self>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Pair))
    }

    fn into_record(self) -> Result<Record<Self>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Record))
    }

    fn into_tuple(self) -> Result<Tuple<Self>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Tuple))
    }

    fn into_left(self) -> Result<Left<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Left))
    }

    fn into_right(self) -> Result<Right<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Right))
    }

    fn into_variant(self) -> Result<Variant<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Variant))
    }

    fn into_nothing(
        self,
    ) -> Result<Nothing<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Nothing))
    }

    fn into_something(self) -> Result<Something<Self>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Something))
    }

    fn into_nil(self) -> Result<Nil<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Nil))
    }

    fn into_cons(self) -> Result<Cons<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Cons))
    }

    fn into_loc(self) -> Result<Loc<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Location))
    }

    fn into_raise(self) -> Result<Raise<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Raise))
    }

    fn into_exception(
        self,
    ) -> Result<Exception<<Self as ValueGroup>::Term, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Exception))
    }

    fn into_fold(self) -> Result<Fold<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Fold))
    }

    fn into_pack(self) -> Result<Pack<Self, Self::Type>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Package))
    }

    fn into_true(self) -> Result<True<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::True))
    }

    fn into_false(self) -> Result<False<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::False))
    }

    fn into_num(self) -> Result<Num<<Self as ValueGroup>::Term>, ValueMismatch> {
        Err(ValueMismatch::new(&self, ValueKind::Number))
    }
}
