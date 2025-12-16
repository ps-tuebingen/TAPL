use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::Left,
    types::{Sum, TypeGroup},
};

impl<Lang> Typecheck for Left<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
    Sum<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let left_res = self.left_term.check(env.clone())?;
        let left_ty = left_res.ret_ty();
        premises.push(left_res);

        let left_norm;
        let ty_norm;
        if features.normalizing() {
            let left_norm_deriv = left_ty.normalize(env.clone());
            let self_norm_deriv = self.ty.clone().normalize(env.clone());
            left_norm = left_norm_deriv.ret_ty();
            ty_norm = self_norm_deriv.ret_ty();
            premises.push(left_norm_deriv);
            premises.push(self_norm_deriv);
        } else {
            left_norm = left_ty;
            ty_norm = self.ty.clone();
        }

        let sum_ty = ty_norm.clone().into_sum().ok_or_else(|| {
            TypeMismatch::new(ty_norm.to_string(), "Sum Type".to_string(), self.span)
        })?;
        if features.kinded() {
            let left_res = left_norm.check_kind(env.clone())?.into_kind()?;
            let sum_res = sum_ty.check_kind(env.clone())?.into_kind()?;
            let left_knd = left_res.ret_kind();
            let sum_knd = sum_res.ret_kind();
            if left_knd != sum_knd {
                return Err(KindMismatch::new(
                    left_knd.to_string(),
                    sum_knd.to_string(),
                    self.span,
                )
                .into());
            }
            premises.push(left_res.into());
            premises.push(sum_res.into());
        }
        if *sum_ty.left != left_norm {
            return Err(TypeMismatch::new(
                sum_ty.left.to_string(),
                left_norm.to_string(),
                self.span,
            )
            .into());
        }

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::left(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![
                Keyword::Left.into(),
                Symbol::sqbrack(vec![
                    Symbol::sub(Symbol::Type, 1),
                    SpecialChar::Plus.into(),
                    Symbol::sub(Symbol::Type, 2),
                ]),
                Symbol::paren(Symbol::Term),
            ],
            vec![
                Symbol::sub(Symbol::Type, 1),
                SpecialChar::Plus.into(),
                Symbol::sub(Symbol::Type, 2),
            ],
            Symbol::sub(Symbol::Type, 1),
            "T-Left",
        )])
    }
}
