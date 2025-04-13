use super::{TypeVar, Var};

pub trait SubstTerm {
    type Term;
    type Type;
    fn subst(self, v: &Var, t: Self::Term) -> Self::Term;
}

pub trait SubstType {
    type Type;
    fn subst_type(self, v: &TypeVar, ty: Self::Type) -> Self;
}
