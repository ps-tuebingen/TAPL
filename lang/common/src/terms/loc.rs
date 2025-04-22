use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Reference,
    values::Loc as LocVal,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loc<T>
where
    T: LanguageTerm,
{
    loc: usize,
    phantom: PhantomData<T>,
}

impl<T> Loc<T>
where
    T: LanguageTerm,
{
    pub fn new(loc: usize) -> Loc<T> {
        Loc {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Loc<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Loc<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Loc<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &<T as LanguageTerm>::Type) -> Self::Target {
        self.into()
    }
}

impl<T> Typecheck for Loc<T>
where
    T: LanguageTerm,
    Reference<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let loc_ty = env.get_loc(&self.loc).map_err(to_check_err)?;
        loc_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Reference::new(loc_ty).into())
    }
}

impl<T> Eval for Loc<T>
where
    T: LanguageTerm,
    LocVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LocVal::new(self.loc).into())
    }
}

impl<T> fmt::Display for Loc<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
