use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{UndefinedLabel, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::Variant,
    types::{TypeGroup, Variant as VariantTy},
};

impl<Lang> Typecheck for Variant<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    VariantTy<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        let term_ty_norm;
        if features.normalizing {
            let term_ty_norm_deriv = term_ty.normalize(env.clone());
            term_ty_norm = term_ty_norm_deriv.ret_ty();
            premises.push(term_ty_norm_deriv);
        } else {
            term_ty_norm = term_ty;
        }

        let var_ty = ty_norm.clone().into_variant()?;
        let lb_ty = var_ty
            .variants
            .get(&self.label)
            .cloned()
            .ok_or_else(|| UndefinedLabel::new(&self.label))?;

        if features.kinded {
            let term_res = term_ty_norm.check_kind(env.clone())?.into_kind()?;
            let lb_res = lb_ty.check_kind(env.clone())?.into_kind()?;
            lb_res.ret_kind().check_equal(&term_res.ret_kind())?;
            premises.push(term_res.into());
            premises.push(lb_res.into());
        }
        lb_ty.check_equal(&term_ty_norm)?;

        let conc = TypingConclusion::new(env, self.clone(), ty_norm);
        premises.push(term_res);
        let deriv = TypingDerivation::variant(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        // <Label_k = Term> as <Label_i=Type_i,...>
        let term = vec![
            Symbol::angbrack(vec![
                Symbol::sub(Symbol::Label, "k"),
                SpecialChar::Equals.into(),
                Symbol::Term,
            ]),
            Keyword::As.into(),
            Symbol::angbrack(Symbol::many(Symbol::colon_sep(
                Symbol::sub(Symbol::Label, "i"),
                Symbol::sub(Symbol::Type, "i"),
            ))),
        ];
        HashSet::from([DerivationRule::check_cong(
            term,
            Symbol::Type,
            Symbol::sub(Symbol::Type, "k"),
            "T-Variant",
        )])
    }
}
