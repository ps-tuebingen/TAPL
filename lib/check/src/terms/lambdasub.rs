use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::LambdaSub,
    types::{ForallBounded, Type},
};

impl<Lang> Typecheck for LambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: Type + Kindcheck<Lang = Lang> + Normalize<Lang = Lang>,
    ForallBounded<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let sup_norm = if features.normalizing {
            let sup_norm_deriv = self.sup_ty.clone().normalize(env.clone());
            sup_norm_deriv.ret_ty()
        } else {
            self.sup_ty.clone()
        };

        if features.kinded {
            let sup_res = sup_norm.check_kind(env.clone())?.into_kind()?;
            env.add_tyvar_kind(self.var.clone(), sup_res.ret_kind());
            premises.push(sup_res.into());
        }

        env.add_tyvar_super(self.var.clone(), sup_norm.clone());
        let term_res = self.body.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let term_norm;
        if features.normalizing {
            let term_norm_deriv = term_ty.normalize(env.clone());
            term_norm = term_norm_deriv.ret_ty();
            premises.push(term_norm_deriv);
        } else {
            term_norm = term_ty;
        }

        let conc = TypingConclusion::new(
            env,
            self.clone(),
            ForallBounded::new(&self.var, self.sup_ty.clone(), term_norm).into(),
        );
        let deriv = TypingDerivation::lambdasub(conc, premises);
        Ok(deriv.into())
    }
}
