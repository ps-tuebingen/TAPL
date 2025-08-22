use crate::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
    values::ValueGroup,
};
use std::fmt;

pub trait Language: fmt::Display + fmt::Debug + Clone {
    type Term: Term
        + SubstTerm<Lang = Self, Target = Self::Term>
        + SubstType<Lang = Self, Target = Self::Term>;

    type Type: TypeGroup + SubstType<Lang = Self, Target = Self::Type>;

    type Value: ValueGroup<Lang = Self> + Into<Self::Term>;

    fn describe(&self) -> &str;
}
