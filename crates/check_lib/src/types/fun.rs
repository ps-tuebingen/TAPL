use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{Fun, TypeGroup};

impl<Ty> Subtypecheck<Ty> for Fun<Ty>
where
    Ty: TypeGroup,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_fun = sup.clone().into_fun().map_err(to_subty_err)?;
        sup_fun
            .from
            .check_subtype(&(*self.from), &mut env.clone())?;
        self.to.check_subtype(&(*sup_fun.to), env)?;
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Fun<Ty>
where
    Ty: TypeGroup,
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

impl<Ty> Normalize<Ty> for Fun<Ty>
where
    Ty: TypeGroup,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let from_norm = self.from.normalize(env);
        let to_norm = self.to.normalize(env);
        Fun {
            from: Box::new(from_norm),
            to: Box::new(to_norm),
        }
        .into()
    }
}
