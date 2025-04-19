use super::Type;
use crate::{
    check::{to_subty_err, CheckEnvironment, Subtypecheck},
    errors::Error,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariable {
    v: TypeVar,
}

impl TypeVariable {
    pub fn new(v: &str) -> TypeVariable {
        TypeVariable { v: v.to_owned() }
    }
}

impl Type for TypeVariable {}

impl<Ty> SubstType<Ty> for TypeVariable
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

impl<Ty> Subtypecheck<Ty> for TypeVariable
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let ty_super = env.get_tyvar_super(&self.v).map_err(to_subty_err)?;
        ty_super.check_equal(&sup).map_err(to_subty_err)
    }

    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let ty_super = env.get_tyvar_super(&self.v).map_err(to_subty_err)?;
        sub.check_supertype(&ty_super, env)
    }
}

impl fmt::Display for TypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
