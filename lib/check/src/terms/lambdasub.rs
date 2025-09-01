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
        let sup_norm = self.sup_ty.clone().normalize(env.clone());
        let sup_kind = sup_norm.check_kind(env.clone())?;
        env.add_tyvar_super(self.var.clone(), sup_norm.clone());
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let term_res = self.body.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());

        let conc = TypingConclusion::new(
            env,
            self.clone(),
            ForallBounded::new(&self.var, self.sup_ty.clone(), term_ty).into(),
        );
        let deriv = TypingDerivation::lambdasub(conc, term_res);

        Ok(deriv.into())
    }
}
