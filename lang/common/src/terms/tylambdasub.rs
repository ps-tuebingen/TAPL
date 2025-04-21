use super::Term;
use crate::{
    check::Typecheck,
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::ForallBounded,
    values::TyLambdaSub as TyLambdaSubVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyLambdaSub<T>
where
    T: LanguageTerm,
{
    var: TypeVar,
    sup: <T as LanguageTerm>::Type,
    term: Box<T>,
}

impl<T> TyLambdaSub<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty>(v: &str, sup: Ty, t: T1) -> TyLambdaSub<T>
    where
        T1: Into<T>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        TyLambdaSub {
            var: v.into(),
            sup: sup.into(),
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for TyLambdaSub<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for TyLambdaSub<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyLambdaSub {
            var: self.var,
            sup: self.sup,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for TyLambdaSub<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        let sup_subst = self.sup.subst_type(v, ty);
        if *v == self.var {
            TyLambdaSub {
                var: self.var,
                sup: sup_subst,
                term: self.term,
            }
            .into()
        } else {
            TyLambdaSub {
                var: self.var,
                sup: sup_subst,
                term: Box::new(self.term.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> Eval for TyLambdaSub<T>
where
    T: LanguageTerm,
    TyLambdaSubVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TyLambdaSubVal::new(&self.var, self.sup, *self.term).into())
    }
}

impl<T> Typecheck for TyLambdaSub<T>
where
    T: LanguageTerm,
    ForallBounded<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(env)?;
        Ok(ForallBounded::new(&self.var, self.sup.clone(), term_ty).into())
    }
}
impl<T> fmt::Display for TyLambdaSub<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup, self.term)
    }
}
