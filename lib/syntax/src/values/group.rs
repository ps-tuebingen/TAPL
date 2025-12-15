use crate::{
    language::Language,
    values::{
        Cons, Exception, False, Fold, Lambda, LambdaSub, Left, Loc, Nil, Nothing, Num, Pack, Pair,
        Raise, Record, Right, Something, True, Tuple, TyLambda, Value, Variant,
    },
};

/// Trait for values in a language
/// needed to have `into_x` functions
/// each one returns an error by default, each language overwrites for values present in the
/// language
pub trait ValueGroup
where
    Self: Value + Into<<Self::Lang as Language>::Term>,
{
    /// Turn `Self` into [`Lambda`]

    fn into_lambda(self) -> Option<Lambda<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`TyLambda`]

    fn into_tylambda(self) -> Option<TyLambda<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`LambdaSub`]

    fn into_lambdasub(self) -> Option<LambdaSub<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Pair`]

    fn into_pair(self) -> Option<Pair<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`Record`]

    fn into_record(self) -> Option<Record<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Tuple`]

    fn into_tuple(self) -> Option<Tuple<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Left`]

    fn into_left(self) -> Option<Left<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`Right`]

    fn into_right(self) -> Option<Right<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Variant`]

    fn into_variant(self) -> Option<Variant<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Nothing`]

    fn into_nothing(self) -> Option<Nothing<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Something`]

    fn into_something(self) -> Option<Something<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Nil`]

    fn into_nil(self) -> Option<Nil<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Cons`]

    fn into_cons(self) -> Option<Cons<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Loc`]

    fn into_loc(self) -> Option<Loc<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Raise`]

    fn into_raise(self) -> Option<Raise<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Exception`]

    fn into_exception(self) -> Option<Exception<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Fold`]

    fn into_fold(self) -> Option<Fold<Self::Lang>> {
        None
    }
    /// Turn `Self` into [`Pack`]

    fn into_pack(self) -> Option<Pack<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`True`]
    fn into_true(self) -> Option<True<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`False`]
    fn into_false(self) -> Option<False<Self::Lang>> {
        None
    }

    /// Turn `Self` into [`Num`]
    fn into_num(self) -> Option<Num<Self::Lang>> {
        None
    }
}
