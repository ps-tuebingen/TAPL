use crate::{Kindcheck, Normalize, Subtypecheck};
use errors::KindMismatch;
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Fun, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Fun<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_fun = sup.clone().into_fun()?;
        sup_fun.from.check_subtype(&(*self.from), env.clone())?;
        self.to.check_subtype(&(*sup_fun.to), env)?;
        Ok(())
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
