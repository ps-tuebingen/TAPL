use super::Type;
use crate::{
    check::{to_kind_err, to_subty_err, Kindcheck, Subtypecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun<Ty>
where
    Ty: Type,
{
    pub from: Box<Ty>,
    pub to: Box<Ty>,
}

impl<Ty> Fun<Ty>
where
    Ty: Type,
{
    pub fn new<T: Into<Ty>, U: Into<Ty>>(from: T, to: U) -> Fun<Ty> {
        Fun {
            from: Box::new(from.into()),
            to: Box::new(to.into()),
        }
    }
}

impl<Ty> Type for Fun<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Fun<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Fun {
            from: Box::new(self.from.subst_type(v, ty)),
            to: Box::new(self.to.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for Fun<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        let sup_fun = sup.clone().into_fun().map_err(to_subty_err)?;
        self.from
            .check_supertype(&(*sup_fun.from), &mut env.clone())?;
        sup_fun.to.check_supertype(&(*self.to), env)?;
        Ok(())
    }

    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sub.clone().into_bot() {
            return Ok(());
        }

        let sub_fun = sub.clone().into_fun().map_err(to_subty_err)?;
        self.from
            .check_subtype(&(*sub_fun.from), &mut env.clone())?;
        sub_fun.to.check_subtype(&(*self.to), env)?;
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Fun<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let from_kind = self.from.check_kind(&mut env.clone())?;
        if from_kind != Kind::Star {
            return Err(to_kind_err(ErrorKind::KindMismatch {
                found: from_kind.to_string(),
                expected: "*".to_owned(),
            }));
        };
        let to_kind = self.to.check_kind(env)?;
        if to_kind != Kind::Star {
            return Err(to_kind_err(ErrorKind::KindMismatch {
                found: to_kind.to_string(),
                expected: "*".to_owned(),
            }));
        }
        Ok(Kind::Star)
    }
}

impl<Ty> fmt::Display for Fun<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
