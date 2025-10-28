use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    subst::SubstType,
    types::{OpLambda, Top, TypeGroup, TypeVariable},
};

impl<Lang> Subtypecheck for OpLambda<Lang>
where
    Lang: Language,
    TypeVariable<Lang>: Into<Lang::Type>,
    Top<Lang>: Into<Lang::Type>,
    OpLambda<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        mut env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        let sup_op = sup.clone().into_oplambda()?;
        sup_op.annot.check_equal(&self.annot)?;
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());

        let body_res = self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env.clone(),
        )?;
        Ok(SubtypeDerivation::op_lambda(env, self.clone(), sup.clone(), body_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_top(), DerivationRule::sub_oplam(false)])
    }
}

impl<Lang> Kindcheck for OpLambda<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let body_res = self.body.check_kind(env)?.into_kind()?;
        let body_kind = body_res.ret_kind();
        let ret_knd = Kind::Arrow(Rc::new(self.annot.clone()), Rc::new(body_kind));
        Ok(KindingDerivation::op_lam(self.clone(), ret_knd, body_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_op_lam(false)])
    }
}

impl<Lang> Normalize for OpLambda<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        let body_norm = self.body.clone().normalize(env);
        let self_norm = OpLambda {
            var: self.var.clone(),
            annot: self.annot.clone(),
            body: Rc::new(body_norm.ret_ty()),
        };
        NormalizingDerivation::cong(self, self_norm, vec![body_norm]).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::norm_cong(|sym| Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Lambda.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Typevariable),
                    separator: Box::new(SpecialChar::DoubleColon.into()),
                    snd: Box::new(Symbol::Kind),
                }),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(sym),
            }),
        })])
    }
}
