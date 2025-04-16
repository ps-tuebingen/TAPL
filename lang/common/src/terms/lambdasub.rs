use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSub<T>
where
    T: LanguageTerm,
{
    var: Var,
    sup_ty: <T as LanguageTerm>::Type,
    body: Box<T>,
}

impl<T> LambdaSub<T>
where
    T: LanguageTerm,
{
    pub fn new<Typ, B>(v: &str, sup: Typ, bod: B) -> LambdaSub<T>
    where
        Typ: Into<<T as LanguageTerm>::Type>,
        B: Into<T>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: sup.into(),
            body: Box::new(bod.into()),
        }
    }
}

impl<T> Term for LambdaSub<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for LambdaSub<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: self.sup_ty,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for LambdaSub<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            LambdaSub {
                var: self.var,
                sup_ty: sup_subst,
                body: self.body,
            }
            .into()
        } else {
            LambdaSub {
                var: self.var,
                sup_ty: sup_subst,
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> fmt::Display for LambdaSub<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.body)
    }
}
