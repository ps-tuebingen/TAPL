use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Ascribe, types::TypeGroup};

impl<Lang> Typecheck for Ascribe<Lang>
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

        let ty_norm;
        let asc_norm;
        if features.normalizing {
            let ty_norm_deriv = term_ty.normalize(env.clone());
            let asc_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            asc_norm = asc_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
            premises.push(asc_norm_deriv);
        } else {
            ty_norm = term_ty;
            asc_norm = self.ty.clone();
        }

        if features.kinded {
            let t_kind = ty_norm.check_kind(env.clone())?;
            let ty_kind = self.ty.check_kind(env.clone())?;
            t_kind.check_equal(&ty_kind)?;
        }

        asc_norm.check_equal(&ty_norm)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::ascribe(conc, premises);
        Ok(deriv.into())
    }
}
