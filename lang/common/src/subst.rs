use super::{terms::Term, types::Type, TypeVar, Var};

pub trait SubstTerm {
    type Term: Term;
    fn subst(self, v: &Var, t: Self::Term) -> Self::Term;
}

pub trait SubstType<Ty>
where
    Ty: Type,
{
    type Target;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target;
}
