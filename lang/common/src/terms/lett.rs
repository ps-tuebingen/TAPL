use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<T>
where
    T: LanguageTerm,
{
    var: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Let<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Let<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            Let {
                var: self.var,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Let<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Let {
            var: self.var,
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            in_term: Box::new(self.in_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Let<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
