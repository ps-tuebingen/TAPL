use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::{UndefinedLabel, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::{HashMap, HashSet};
use syntax::{
    env::Environment,
    language::Language,
    types::{Record, Top, TypeGroup},
};

impl<Lang> Subtypecheck for Record<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Record<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + Normalize<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        let mut premises = vec![];

        let sup_norm;
        if features.normalizing {
            let sup_norm_deriv = sup.clone().normalize(env.clone());
            sup_norm = sup_norm_deriv.ret_ty();
            premises.push(sup_norm_deriv);
        } else {
            sup_norm = sup.clone();
        }

        let sup_rec = sup_norm.into_record()?;
        for (lb, ty) in sup_rec.records.iter() {
            let sub_ty = self.records.get(lb).ok_or(UndefinedLabel::new(lb))?;
            premises.push(sub_ty.check_subtype(ty, env.clone())?);
        }
        Ok(SubtypeDerivation::record(env, self.clone(), sup.clone(), premises).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_rec()])
    }
}

impl<Lang> Kindcheck for Record<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let mut rec_res = vec![];
        for (_, t) in self.records.iter() {
            let ty_res = t.check_kind(env.clone())?;
            rec_res.push(ty_res);
        }
        Ok(KindingDerivation::record(self.clone(), rec_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_rec()])
    }
}

impl<Lang> Normalize for Record<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        let mut premises = vec![];
        let mut recs_norm = HashMap::new();
        for (lb, ty) in self.records.iter() {
            let ty_norm = ty.clone().normalize(env.clone());
            recs_norm.insert(lb.clone(), ty_norm.ret_ty());
            premises.push(ty_norm)
        }
        let self_norm = Record { records: recs_norm };
        NormalizingDerivation::cong(self, self_norm, premises).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::norm_cong(|sym| Symbol::Delim {
            delim_open: SpecialChar::BrackO,
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: Box::new(SpecialChar::Colon.into()),
                snd: Box::new(sym),
            }))),
            delim_close: SpecialChar::BrackC,
        })])
    }
}
