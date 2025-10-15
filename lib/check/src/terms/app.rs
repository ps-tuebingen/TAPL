use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::App,
    types::{Fun, TypeGroup},
};

impl<Lang> Typecheck for App<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang>
        + Normalize<Lang = Lang>
        + Kindcheck<Lang = Lang>
        + Subtypecheck<Lang = Lang>,
    Self: Into<Lang::Term>,
    Fun<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();

        let fun_res = self.fun.check(env.clone())?;
        let fun_ty = if features.normalizing {
            fun_res.ret_ty().normalize(env.clone())
        } else {
            fun_res.ret_ty()
        };

        if features.kinded {
            fun_ty.check_kind(env.clone())?.into_star()?;
        }

        let fun: Fun<Lang> = fun_ty.into_fun()?;
        let arg_res = self.arg.check(env.clone())?;
        let arg_ty = if features.normalizing {
            arg_res.ret_ty().normalize(env.clone())
        } else {
            arg_res.ret_ty()
        };

        if features.kinded {
            arg_ty.check_kind(env.clone())?.into_star()?;
        }
        if features.subtyped {
            arg_ty.check_subtype(&(*fun.from), env.clone())?;
        }

        let deriv_conc = TypingConclusion::new(env.clone(), self.clone(), *fun.to);
        let deriv = TypingDerivation::app(deriv_conc, fun_res, arg_res);
        Ok(deriv.into())
    }
}
