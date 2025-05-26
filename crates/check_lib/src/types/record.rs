use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{Record, TypeGroup};

impl<Ty> Subtypecheck<Ty> for Record<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_norm = sup.clone().normalize(env);
        let sup_rec = sup_norm.into_record().map_err(to_subty_err)?;
        for (lb, ty) in sup_rec.records.iter() {
            let sub_ty = self
                .records
                .get(lb)
                .ok_or(to_subty_err(ErrorKind::UndefinedLabel(lb.clone())))?;
            sub_ty.check_subtype(ty, &mut env.clone())?;
        }
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Record<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        for (_, t) in self.records.iter() {
            t.check_kind(&mut env.clone())?
                .into_star()
                .map_err(to_kind_err)?;
        }
        Ok(Kind::Star)
    }
}
