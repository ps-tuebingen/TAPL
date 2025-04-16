use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unpack<T>
where
    T: LanguageTerm,
{
    ty_name: TypeVar,
    term_name: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Unpack<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2>(tyn: &str, tn: &str, bound: T1, int: T2) -> Unpack<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        Unpack {
            ty_name: tyn.to_owned(),
            term_name: tn.to_owned(),
            bound_term: Box::new(bound.into()),
            in_term: Box::new(int.into()),
        }
    }
}

impl<T> Term for Unpack<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Unpack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.term_name {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Unpack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;

    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        let bound_subst = self.bound_term.subst_type(v, ty);
        if *v == self.ty_name {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> fmt::Display for Unpack<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_name, self.term_name, self.bound_term, self.in_term
        )
    }
}
