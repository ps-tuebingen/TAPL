use super::Type;
use crate::{
    check::{to_kind_err, to_subty_err, CheckEnvironment, Kindcheck, Subtypecheck},
    errors::Error,
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariable<Ty>
where
    Ty: Type,
{
    pub v: TypeVar,
    phantom: PhantomData<Ty>,
}

impl<Ty> TypeVariable<Ty>
where
    Ty: Type,
{
    pub fn new(v: &str) -> TypeVariable<Ty> {
        TypeVariable {
            v: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<Ty> Type for TypeVariable<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for TypeVariable<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.v {
            ty.clone()
        } else {
            self.into()
        }
    }
}

impl<Ty> Subtypecheck<Ty> for TypeVariable<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let ty_super = env.get_tyvar_super(&self.v).map_err(to_subty_err)?;
        let sup_norm = sup.clone().normalize(env);

        if sup_norm.clone().into_top().is_ok() {
            return Ok(());
        }

        if let Ok(v) = sup_norm.clone().into_variable() {
            if v.v == self.v {
                return Ok(());
            }
        }
        ty_super.check_equal(&sup_norm).map_err(to_subty_err)
    }
}

impl<Ty> Kindcheck<Ty> for TypeVariable<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        env.get_tyvar_kind(&self.v).map_err(to_kind_err)
    }
}

impl<Ty> Normalize<Ty> for TypeVariable<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.get_tyvar_super(&self.v).unwrap_or(self.into())
    }
}

impl<Ty> fmt::Display for TypeVariable<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
