use crate::{
    language::Language,
    values::{
        Cons, Exception, False, Fold, Lambda, LambdaSub, Left, Loc, Nil, Nothing, Num, Pack, Pair,
        Raise, Record, Right, Something, True, Tuple, TyLambda, Value, Variant,
    },
};
use errors::ValueMismatch;

/// Triat for values in a language
/// needed to have `into_x` functions
/// each one returns an error by default, each language overwrites for values present in the
/// language
pub trait ValueGroup
where
    Self: Value + Into<<Self::Lang as Language>::Term>,
{
    /// Turn `Self` into [`Lambda`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_lambda(self) -> Result<Lambda<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
    }
    /// Turn `Self` into [`TyLambda`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_tylambda(self) -> Result<TyLambda<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "TyLambda".to_owned()))
    }
    /// Turn `Self` into [`LambdaSub`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_lambdasub(self) -> Result<LambdaSub<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "LambdaSub".to_owned()))
    }
    /// Turn `Self` into [`Pair`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_pair(self) -> Result<Pair<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Pair".to_owned()))
    }

    /// Turn `Self` into [`Record`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_record(self) -> Result<Record<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Record".to_owned()))
    }
    /// Turn `Self` into [`Tuple`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_tuple(self) -> Result<Tuple<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Tuple".to_owned()))
    }
    /// Turn `Self` into [`Left`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_left(self) -> Result<Left<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Left".to_owned()))
    }

    /// Turn `Self` into [`Right`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_right(self) -> Result<Right<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Right".to_owned()))
    }
    /// Turn `Self` into [`Variant`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_variant(self) -> Result<Variant<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Variant".to_owned()))
    }
    /// Turn `Self` into [`Nothing`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_nothing(self) -> Result<Nothing<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Nothing".to_owned()))
    }
    /// Turn `Self` into [`Something`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_something(self) -> Result<Something<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Something".to_owned()))
    }
    /// Turn `Self` into [`Nil`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_nil(self) -> Result<Nil<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Nil".to_owned()))
    }
    /// Turn `Self` into [`Cons`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_cons(self) -> Result<Cons<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Cons".to_owned()))
    }
    /// Turn `Self` into [`Loc`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_loc(self) -> Result<Loc<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Location".to_owned()))
    }
    /// Turn `Self` into [`Raise`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_raise(self) -> Result<Raise<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Raise".to_owned()))
    }
    /// Turn `Self` into [`Exception`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_exception(self) -> Result<Exception<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Exception".to_owned()))
    }
    /// Turn `Self` into [`Fold`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_fold(self) -> Result<Fold<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Fold".to_owned()))
    }
    /// Turn `Self` into [`Pack`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_pack(self) -> Result<Pack<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Package".to_owned()))
    }

    /// Turn `Self` into [`True`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_true(self) -> Result<True<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "True".to_owned()))
    }

    /// Turn `Self` into [`False`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_false(self) -> Result<False<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "False".to_owned()))
    }

    /// Turn `Self` into [`Num`]
    /// # Errors
    /// Returns an error if `Self` is a different value
    fn into_num(self) -> Result<Num<Self::Lang>, ValueMismatch> {
        Err(ValueMismatch::new(self.to_string(), "Number".to_owned()))
    }
}
