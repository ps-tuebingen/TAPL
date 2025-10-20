use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, NormalizingDerivation, SubtypeDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
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
        let features = Lang::features();
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }
        let mut premises = vec![];

        let sup_norm;
        let self_sup_norm;
        if features.normalizing {
            let sup_norm_deriv = sup.clone().normalize(env.clone());
            let self_sup_norm_deriv = self.sup.clone().normalize(env.clone());
            sup_norm = sup_norm_deriv.ret_ty();
            self_sup_norm = self_sup_norm_deriv.ret_ty();
            premises.push(sup_norm_deriv);
            premises.push(self_sup_norm_deriv);
        } else {
            sup_norm = sup.clone();
            self_sup_norm = Rc::unwrap_or_clone(self.sup.clone());
        }

        let sup_op = sup_norm.into_oplambdasub()?;
        sup_op.sup.check_equal(&self_sup_norm)?;
        env.add_tyvar_super(self.var.clone(), self_sup_norm);

        let body_res = self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env.clone(),
        )?;
        premises.push(body_res);
        Ok(SubtypeDerivation::op_lambda_sub(env, self.clone(), sup.clone(), premises).into())
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
        Ok(Kind::Arrow(Rc::new(sup_kind), Rc::new(body_kind)))
    }
}

impl<Lang> Normalize for OpLambdaSub<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        let body_norm = self.body.clone().normalize(env);
        let self_norm = OpLambdaSub {
            var: self.var.clone(),
            sup: self.sup.clone(),
            body: Rc::new(body_norm.ret_ty()),
        };
        NormalizingDerivation::cong(self, self_norm, vec![body_norm]).into()
    }
}
