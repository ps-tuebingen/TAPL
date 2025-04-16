use super::{LanguageType, LanguageValue};
use crate::{
    check::Typecheck,
    eval::Eval,
    subst::{SubstTerm, SubstType},
    terms::Term,
};

pub trait LanguageTerm
where
    Self: Term
        + SubstTerm<Self, Target = Self>
        + SubstType<<Self as LanguageTerm>::Type, Target = Self>
        + Eval<Value = <Self as LanguageTerm>::Value>
        + Typecheck<Type = <Self as LanguageTerm>::Type>,
{
    type Type: LanguageType;
    type Value: LanguageValue<Term = Self>;
}
