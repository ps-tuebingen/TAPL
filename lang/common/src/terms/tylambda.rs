use super::Term;
use crate::{
    kinds::Kind,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyLambda<T>
where
    T: LanguageTerm,
{
    var: TypeVar,
    annot: Kind,
    term: Box<T>,
}

impl<T> TyLambda<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(v: &str, knd: Kind, t: T1) -> TyLambda<T>
    where
        T1: Into<T>,
    {
        TyLambda {
            var: v.into(),
            annot: knd,
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for TyLambda<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for TyLambda<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyLambda {
            var: self.var,
            annot: self.annot,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for TyLambda<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            TyLambda {
                var: self.var,
                annot: self.annot,
                term: Box::new(self.term.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> fmt::Display for TyLambda<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.term)
    }
}
