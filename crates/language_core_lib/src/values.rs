use super::terms::LanguageTerm;
use crate::{
    errors::ErrorKind,
    values::{
        Cons, Exception, False, Fold, Lambda, LambdaSub, Left, Loc, Nil, Nothing, Num, Pack, Pair,
        Raise, Record, Right, Something, True, Tuple, TyLambda, Value, Variant,
    },
};

pub trait LanguageValue
where
    Self: Value + Into<<Self as LanguageValue>::Term>,
{
    type Term: LanguageTerm;
}
