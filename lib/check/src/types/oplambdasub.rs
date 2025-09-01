use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    subst::SubstType,
    types::{OpLambdaSub, Top, TypeGroup, TypeVariable},
};
impl<Lang> Subtypecheck for OpLambdaSub<Lang>
where
    Lang: Language,
    TypeVariable<Lang>: Into<Lang::Type>,
    Top<Lang>: Into<Lang::Type>,
    OpLambdaSub<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + Normalize<Lang = Lang> + TypeGroup<Lang = Lang>,
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
        let self_sup_norm = self.sup.clone().normalize(env.clone());
        let sup_op = sup_norm.into_oplambdasub()?;
        sup_op.sup.check_equal(&self_sup_norm)?;
        env.add_tyvar_super(self.var.clone(), self_sup_norm);

        let body_res = self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env.clone(),
        )?;
        Ok(SubtypeDerivation::op_lambda_sub(env, self.clone(), sup.clone(), body_res).into())
    }
}

impl<Lang> Kindcheck for OpLambdaSub<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        let sup_kind = self.sup.check_kind(env.clone())?;
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let body_kind = self.body.check_kind(env)?;
        Ok(Kind::Arrow(Box::new(sup_kind), Box::new(body_kind)))
    }
}

impl<Lang> Normalize for OpLambdaSub<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        let body_norm = self.body.normalize(env);
        OpLambdaSub {
            var: self.var,
            sup: self.sup,
            body: Box::new(body_norm),
        }
        .into()
    }
}
