use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use errors::NameMismatch;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{ForallBounded, Top, TypeGroup},
};

impl<Lang> Subtypecheck for ForallBounded<Lang>
where
    Lang: Language,
    ForallBounded<Lang>: Into<Lang::Type>,
    Top<Lang>: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang> + TypeGroup<Lang = Lang> + Subtypecheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }
        let other_forall = sup.clone().into_forall_bounded()?;
        let sup_norm = other_forall.sup_ty.normalize(env.clone());
        let self_norm = self.sup_ty.clone().normalize(env.clone());
        let sup_res = sup_norm.check_subtype(&self_norm, env.clone())?;
        if self.var != other_forall.var {
            return Err(NameMismatch::new(&other_forall.var, &self.var).into());
        }
        let ty_norm = self.ty.clone().normalize(env.clone());
        let inner_res = ty_norm.check_subtype(&(*other_forall.ty), env.clone())?;
        Ok(
            SubtypeDerivation::forall_bounded(env, self.clone(), sup.clone(), sup_res, inner_res)
                .into(),
        )
    }
}

impl<Lang> Kindcheck for ForallBounded<Lang>
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

impl<Lang> Normalize for ForallBounded<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        env.add_tyvar_super(self.var.clone(), *self.ty.clone());
        let ty_norm = self.ty.normalize(env);
        ForallBounded {
            var: self.var,
            sup_ty: self.sup_ty,
            ty: Box::new(ty_norm),
        }
        .into()
    }
}
