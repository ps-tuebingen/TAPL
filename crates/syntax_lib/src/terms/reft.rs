use super::Term;
use common::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, EvalEnvironment, Normalize},
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Reference,
    values::Loc as LocVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ref<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
}

impl<T> Ref<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1) -> Ref<T>
    where
        T1: Into<T>,
    {
        Ref {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Ref<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Ref<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Ref {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Ref<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Ref {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Ref<T>
where
    T: LanguageTerm,
    Reference<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Reference::new(term_ty).into())
    }
}

impl<T> Eval for Ref<T>
where
    T: LanguageTerm,
    LocVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let fresh_loc = env.fresh_location();
        env.save_location(fresh_loc, term_val);
        Ok(LocVal::new(fresh_loc).into())
    }
}

impl<T> fmt::Display for Ref<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ref({})", self.term)
    }
}
