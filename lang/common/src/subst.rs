use super::{terms::Term, types::Type, TypeVar, Var};

pub trait SubstTerm<T>
where
    T: Term,
{
    type Target;
    fn subst(self, v: &Var, t: &T) -> Self::Target;
}

pub trait SubstType<Ty>
where
    Ty: Type,
{
    type Target;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target;
}
