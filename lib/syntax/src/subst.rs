use crate::{TypeVar, Var, language::Language};

pub trait SubstTerm {
    type Lang: Language;
    type Target;
    fn subst(self, v: &Var, t: &<Self::Lang as Language>::Term) -> Self::Target;
}

pub trait SubstType {
    type Target;
    type Lang: Language;
    fn subst_type(self, v: &TypeVar, ty: &<Self::Lang as Language>::Type) -> Self::Target;
}
