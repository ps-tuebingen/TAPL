use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SomeCase<T>
where
    T: LanguageTerm,
{
    bound_term: Box<T>,
    none_term: Box<T>,
    some_var: Var,
    some_term: Box<T>,
}

impl<T> Term for SomeCase<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for SomeCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.some_var {
            SomeCase {
                bound_term: Box::new(self.bound_term.subst(v, t)),
                none_term: Box::new(self.none_term.subst(v, t)),
                some_var: self.some_var,
                some_term: self.some_term,
            }
            .into()
        } else {
            SomeCase {
                bound_term: Box::new(self.bound_term.subst(v, t)),
                none_term: Box::new(self.none_term.subst(v, t)),
                some_var: self.some_var,
                some_term: Box::new(self.some_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for SomeCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        SomeCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            none_term: Box::new(self.none_term.subst_type(v, ty)),
            some_var: self.some_var,
            some_term: Box::new(self.some_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for SomeCase<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ nothing => {} | something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}
