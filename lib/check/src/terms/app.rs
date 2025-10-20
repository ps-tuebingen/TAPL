use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
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
        let mut premises = vec![];

        let fun_res = self.fun.check(env.clone())?;
        let fun_ty = fun_res.ret_ty();
        premises.push(fun_res);

        let fun_norm;
        if features.normalizing {
            let fun_norm_deriv = fun_ty.normalize(env.clone());
            fun_norm = fun_norm_deriv.ret_ty();
            premises.push(fun_norm_deriv);
        } else {
            fun_norm = fun_ty;
        };

        if features.kinded {
            fun_norm.check_kind(env.clone())?.into_star()?;
        }

        let fun: Fun<Lang> = fun_norm.into_fun()?;
        let arg_res = self.arg.check(env.clone())?;
        let arg_ty = arg_res.ret_ty();
        premises.push(arg_res);

        let arg_norm;
        if features.normalizing {
            let arg_norm_deriv = arg_ty.normalize(env.clone());
            arg_norm = arg_norm_deriv.ret_ty();
            premises.push(arg_norm_deriv);
        } else {
            arg_norm = arg_ty;
        }

        if features.kinded {
            arg_norm.check_kind(env.clone())?.into_star()?;
        }

        if features.subtyped {
            let arg_sub_deriv = arg_norm.check_subtype(&(*fun.from), env.clone())?;
            premises.push(arg_sub_deriv);
        }

        let deriv_conc =
            TypingConclusion::new(env.clone(), self.clone(), Rc::unwrap_or_clone(fun.to));
        let deriv = TypingDerivation::app(deriv_conc, premises);
        Ok(deriv.into())
    }
}
