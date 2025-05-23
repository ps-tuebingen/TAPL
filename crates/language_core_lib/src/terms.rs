use super::{LanguageType, LanguageValue};
use crate::{
    check::{Kindcheck, Subtypecheck, Typecheck},
    eval::{Eval, Normalize},
    parse::Parse,
    subst::{SubstTerm, SubstType},
    terms::Term,
};

pub trait LanguageTerm
where
    Self: Term
        + Parse
        + SubstTerm<Self, Target = Self>
        + SubstType<<Self as LanguageTerm>::Type, Target = Self>
        + Eval<Value = <Self as LanguageTerm>::Value>
        + Typecheck<Type = <Self as LanguageTerm>::Type>,
{
    type Type: LanguageType
        + Subtypecheck<<Self as LanguageTerm>::Type, Env = <Self as Typecheck>::Env>
        + Kindcheck<<Self as LanguageTerm>::Type, Env = <Self as Typecheck>::Env>
        + Normalize<<Self as LanguageTerm>::Type, Env = <Self as Typecheck>::Env>;
    type Value: LanguageValue<Term = Self>;
}
