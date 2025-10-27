use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::If, types::TypeGroup};

impl<Lang> Typecheck for If<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let if_res = self.if_cond.check(env.clone())?;
        let if_ty = if_res.ret_ty();
        premises.push(if_res);

        let if_norm;
        if features.normalizing {
            let if_norm_deriv = if_ty.normalize(env.clone());
            if_norm = if_norm_deriv.ret_ty();
            premises.push(if_norm_deriv);
        } else {
            if_norm = if_ty;
        }

        if features.kinded {
            let if_res = if_norm.check_kind(env.clone())?.into_kind()?;
            if_res.ret_kind().into_star()?;
            premises.push(if_res.into());
        }

        if_norm.into_bool()?;

        let then_res = self.then_term.check(env.clone())?;
        let then_ty = then_res.ret_ty();
        premises.push(then_res);

        let then_norm;
        if features.normalizing {
            let then_norm_deriv = then_ty.normalize(env.clone());
            then_norm = then_norm_deriv.ret_ty();
            premises.push(then_norm_deriv);
        } else {
            then_norm = then_ty;
        }

        let else_res = self.else_term.check(env.clone())?;
        let else_ty = else_res.ret_ty();
        premises.push(else_res);

        let else_norm;
        if features.normalizing {
            let else_norm_deriv = else_ty.normalize(env.clone());
            else_norm = else_norm_deriv.ret_ty();
            premises.push(else_norm_deriv);
        } else {
            else_norm = else_ty;
        }

        if features.kinded {
            let then_res = then_norm.check_kind(env.clone())?.into_kind()?;
            let else_res = else_norm.check_kind(env.clone())?.into_kind()?;
            then_res.ret_kind().check_equal(&else_res.ret_kind())?;
            premises.push(then_res.into());
            premises.push(else_res.into());
        }

        then_norm.check_equal(&else_norm)?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), then_norm);
        let deriv = TypingDerivation::ift(conc, premises);
        Ok(deriv.into())
    }
}
