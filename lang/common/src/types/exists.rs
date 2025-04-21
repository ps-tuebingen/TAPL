use super::Type;
use crate::{
    check::{CheckEnvironment, Kindcheck},
    errors::Error,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exists<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    kind: Kind,
    pub ty: Box<Ty>,
}

impl<Ty> Exists<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1) -> Exists<Ty>
    where
        Ty1: Into<Ty>,
    {
        Exists {
            var: v.to_owned(),
            kind: knd,
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Exists<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Exists<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            Exists {
                var: self.var,
                kind: self.kind,
                ty: Box::new((*self.ty).subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> Kindcheck<Ty> for Exists<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.ty.check_kind(env)
    }
}

impl<Ty> fmt::Display for Exists<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exitsts {},{}}}", self.var, self.ty)
    }
}
