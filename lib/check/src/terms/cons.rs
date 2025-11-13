use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::Cons,
    types::{List, TypeGroup},
};

impl<Lang> Typecheck for Cons<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    List<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let annot_norm;
        if features.normalizing {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            annot_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            annot_norm = self.ty.clone();
        }

        let hd_res = self.head.check(env.clone())?;
        let hd_ty = hd_res.ret_ty();
        premises.push(hd_res);

        let hd_norm;
        if features.normalizing {
            let hd_norm_deriv = hd_ty.normalize(env.clone());
            hd_norm = hd_norm_deriv.ret_ty();
            premises.push(hd_norm_deriv);
        } else {
            hd_norm = hd_ty;
        }

        hd_norm.check_equal(&annot_norm)?;

        if features.kinded {
            let hd_res = hd_norm.check_kind(env.clone())?.into_kind()?;
            hd_res.ret_kind().into_star()?;
            premises.push(hd_res.into());
        }

        let tl_res = self.tail.check(env.clone())?;
        let tl_ty = tl_res.ret_ty();
        premises.push(tl_res);

        let tail_ty_norm;
        if features.normalizing {
            let tl_norm_deriv = tl_ty.normalize(env.clone());
            tail_ty_norm = tl_norm_deriv.ret_ty();
            premises.push(tl_norm_deriv);
        } else {
            tail_ty_norm = tl_ty;
        }

        if features.kinded {
            let tl_res = tail_ty_norm.check_kind(env.clone())?.into_kind()?;
            tl_res.ret_kind().into_star()?;
            premises.push(tl_res.into());
        }

        let list_ty: Lang::Type = List::new(annot_norm).into();
        tail_ty_norm.check_equal(&list_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), list_ty);
        let deriv = TypingDerivation::cons(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cons()])
    }
}
