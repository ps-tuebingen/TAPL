use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::{check_error::CheckError, NameMismatch};
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{ExistsBounded, Top, TypeGroup},
};

impl<Lang> Subtypecheck for ExistsBounded<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    ExistsBounded<Lang>: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang> + TypeGroup<Lang = Lang> + Subtypecheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        mut env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        let sup_norm = sup.clone().normalize(env.clone());
        let self_norm = self.sup_ty.clone().normalize(env.clone());
        let other_exists = sup_norm.into_exists_bounded()?;
        other_exists.sup_ty.check_equal(&self_norm)?;
        if self.var != other_exists.var {
            return Err(NameMismatch::new(&other_exists.var, &self.var).into());
        }
        let old_env = env.clone();
        env.add_tyvar_super(other_exists.var, *self.sup_ty.clone());
        let inner_res = self
            .ty
            .clone()
            .normalize(env.clone())
            .check_subtype(&(*other_exists.ty), env)?;
        Ok(SubtypeDerivation::exists_bounded(old_env, self.clone(), sup.clone(), inner_res).into())
    }
}

impl<Lang> Kindcheck for ExistsBounded<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        let sup_kind = self.sup_ty.check_kind(env.clone())?;
        env.add_tyvar_kind(self.var.clone(), sup_kind);
        self.ty.check_kind(env)
    }
}

impl<Lang> Normalize for ExistsBounded<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        env.add_tyvar_super(self.var.clone(), *self.sup_ty.clone());
        self.into()
    }
}
