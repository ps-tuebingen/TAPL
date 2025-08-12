use crate::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
    values::ValueGroup,
};

pub trait Language {
    type Term: Term
        + SubstTerm<Self::Term, Target = Self::Term>
        + SubstType<Self::Type, Target = Self::Term>;

    type Type: TypeGroup + SubstType<Self::Type, Target = Self::Type>;

    type Value: ValueGroup<Term = Self::Term, Type = Self::Type>;

    fn describe(&self) -> &str;
}
