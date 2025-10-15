use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use errors::UndefinedLabel;
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
        let ty_norm = self.ty.clone().normalize(env.clone());
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;

        let var_ty = ty_norm.clone().into_variant()?;
        let lb_ty = var_ty
            .variants
            .get(&self.label)
            .cloned()
            .ok_or(UndefinedLabel::new(&self.label))?;
        let lb_knd = lb_ty.check_kind(env.clone())?;

        lb_knd.check_equal(&term_knd)?;
        lb_ty.check_equal(&term_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), ty_norm);
        let deriv = TypingDerivation::variant(conc, term_res);
        Ok(deriv.into())
    }
}
