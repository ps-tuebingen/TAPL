use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::KindMismatch;
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Fun, Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck for Fun<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty>,
    Fun<Ty>: Into<Ty>,
    Top<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        let sup_fun = sup.clone().into_fun()?;
        let from_res = sup_fun.from.check_subtype(&(*self.from), env.clone())?;
        let to_res = self.to.check_subtype(&(*sup_fun.to), env.clone())?;
        Ok(SubtypeDerivation::fun(env, self.clone(), sup.clone(), from_res, to_res).into())
    }
}

impl<Ty> Kindcheck<Ty> for Fun<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, CheckError> {
        let from_kind = self.from.check_kind(env.clone())?;
        if from_kind != Kind::Star {
            return Err(KindMismatch::new(from_kind.to_string(), Kind::Star.to_string()).into());
        };

        let to_kind = self.to.check_kind(env)?;
        if to_kind != Kind::Star {
            return Err(KindMismatch::new(to_kind.to_string(), Kind::Star.to_string()).into());
        }
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Fun<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, env: Environment<Ty>) -> Ty {
        let from_norm = self.from.normalize(env.clone());
        let to_norm = self.to.normalize(env);
        Fun {
            from: Box::new(from_norm),
            to: Box::new(to_norm),
        }
        .into()
    }
}
