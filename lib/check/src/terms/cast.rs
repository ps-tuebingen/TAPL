use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Cast, types::TypeGroup};

impl<Lang> Typecheck for Cast<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let term_norm;
        let self_norm;
        if features.normalizing {
            let term_norm_deriv = term_ty.normalize(env.clone());
            let self_norm_deriv = self.ty.clone().normalize(env.clone());
            term_norm = term_norm_deriv.ret_ty();
            self_norm = self_norm_deriv.ret_ty();
            premises.push(term_norm_deriv);
        } else {
            term_norm = term_ty;
            self_norm = self.ty.clone();
        }

        if features.kinded {
            let term_kind = term_norm.check_kind(env.clone())?;
            let ty_kind = self_norm.check_kind(env.clone())?;
            term_kind.check_equal(&ty_kind)?;
        }

        let conc = TypingConclusion::new(env, self.clone(), self_norm.clone());
        let deriv = TypingDerivation::cast(conc, premises);
        Ok(deriv.into())
    }
}
