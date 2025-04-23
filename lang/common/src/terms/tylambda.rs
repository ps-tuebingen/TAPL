use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    kinds::Kind,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Forall,
    values::TyLambda as TyLambdaVal,
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

impl<T> Eval for TyLambda<T>
where
    T: LanguageTerm,
    TyLambdaVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TyLambdaVal::new(&self.var, self.annot, *self.term).into())
    }
}

impl<T> Typecheck for TyLambda<T>
where
    T: LanguageTerm,
    Forall<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let term_ty = self.term.check(env)?;
        let term_knd = term_ty.check_kind(env)?;
        self.annot.check_equal(&term_knd).map_err(to_check_err)?;
        Ok(Forall::new(&self.var, self.annot.clone(), term_ty).into())
    }
}
impl<T> fmt::Display for TyLambda<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.({})", self.var, self.annot, self.term)
    }
}
