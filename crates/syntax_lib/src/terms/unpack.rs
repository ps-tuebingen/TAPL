use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};

use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unpack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub ty_name: TypeVar,
    pub term_name: Var,
    pub bound_term: Box<T>,
    pub in_term: Box<T>,
    phantom: PhantomData<Ty>,
}

impl<T, Ty> Unpack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, T2>(tyn: &str, tn: &str, bound: T1, int: T2) -> Unpack<T, Ty>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        Unpack {
            ty_name: tyn.to_owned(),
            term_name: tn.to_owned(),
            bound_term: Box::new(bound.into()),
            in_term: Box::new(int.into()),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Term for Unpack<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Unpack<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
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
                phantom: PhantomData,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: Box::new(self.in_term.subst(v, t)),
                phantom: PhantomData,
            }
            .into()
        }
    }
}

impl<T, Ty> SubstType<Ty> for Unpack<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        let bound_subst = self.bound_term.subst_type(v, ty);
        if *v == self.ty_name {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
                phantom: PhantomData,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst_type(v, ty)),
                phantom: PhantomData,
            }
            .into()
        }
    }
}

impl<T, Ty> fmt::Display for Unpack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_name, self.term_name, self.bound_term, self.in_term
        )
    }
}
