use crate::{TypeVar, Var, language::Language};
use std::rc::Rc;

pub trait SubstTerm {
    type Lang: Language;
    type Target;
    fn subst(self, v: &Var, t: &<Self::Lang as Language>::Term) -> Self::Target;
}

pub trait SubstType {
    type Target;
    type Lang: Language;
    fn subst_type(self, v: &TypeVar, ty: &<Self::Lang as Language>::Type) -> Self::Target;

    fn subst_sim(self, subst: &[(&TypeVar, &<Self::Lang as Language>::Type)]) -> Self::Target
    where
        Self: Into<Self::Target>,
        Self::Target: SubstType<Target = Self::Target, Lang = Self::Lang>,
    {
        let mut target = self.into();
        for (v, ty) in subst {
            target = target.subst_type(v, ty);
        }
        target
    }
}

impl<T> SubstTerm for Rc<T>
where
    T: SubstTerm + Clone,
{
    type Lang = T::Lang;
    type Target = Rc<T::Target>;
    fn subst(self, v: &Var, t: &<Self::Lang as Language>::Term) -> Self::Target {
        Rc::new(Self::unwrap_or_clone(self).subst(v, t))
    }
}

impl<T> SubstType for Rc<T>
where
    T: SubstType + Clone,
{
    type Target = Rc<T::Target>;
    type Lang = T::Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Self::Lang as Language>::Type) -> Self::Target {
        Rc::new(Self::unwrap_or_clone(self).subst_type(v, ty))
    }
}
