use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Let};

impl<Lang> Typecheck for Let<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty();
        premises.push(bound_res);

        let bound_norm;
        if features.normalizing {
            let bound_norm_deriv = bound_ty.normalize(env.clone());
            bound_norm = bound_norm_deriv.ret_ty();
            premises.push(bound_norm_deriv);
        } else {
            bound_norm = bound_ty;
        }

        if features.kinded {
            premises.push(bound_norm.check_kind(env.clone())?);
        }

        env.add_var(self.var.clone(), bound_norm);
        let in_res = self.in_term.check(env.clone())?;
        let in_ty = in_res.ret_ty();
        premises.push(in_res);

        let in_norm;
        if features.normalizing {
            let in_norm_deriv = in_ty.normalize(env.clone());
            in_norm = in_norm_deriv.ret_ty();
            premises.push(in_norm_deriv);
        } else {
            in_norm = in_ty;
        }

        if features.kinded {
            premises.push(in_norm.check_kind(env.clone())?);
        }

        let conc = TypingConclusion::new(env.clone(), self.clone(), in_norm);
        let deriv = TypingDerivation::lett(conc, premises);
        Ok(deriv.into())
    }
}
