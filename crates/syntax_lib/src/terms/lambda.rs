use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Fun,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lambda<T>
where
    T: Term,
{
    pub var: Var,
    pub annot: <T as Term>::Type,
    pub body: Box<T>,
}

impl<T> Lambda<T>
where
    T: Term,
{
    pub fn new<A, B>(v: &str, a: A, b: B) -> Lambda<T>
    where
        A: Into<<T as Term>::Type>,
        B: Into<T>,
    {
        Lambda {
            var: v.to_owned(),
            annot: a.into(),
            body: Box::new(b.into()),
        }
    }
}

impl<T> Term for Lambda<T> where T: Term {}

impl<T> SubstTerm<T> for Lambda<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as Term>::Type> for Lambda<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Lambda {
            var: self.var,
            annot: self.annot.subst_type(v, ty),
            body: Box::new(self.body.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Lambda<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
