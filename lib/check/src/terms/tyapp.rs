use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{TypeMismatch, check_error::CheckError};
use std::rc::Rc;
use syntax::{
    env::Environment, language::Language, subst::SubstType, terms::TyApp, types::TypeGroup,
};

impl<Lang> Typecheck for TyApp<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang>
        + Normalize<Lang = Lang>
        + Kindcheck<Lang = Lang>
        + Subtypecheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let fun_res = self.fun.check(env.clone())?;
        let fun_ty = fun_res.ret_ty();
        premises.push(fun_res);

        let fun_norm;
        let arg_norm;
        if features.normalizing {
            let fun_norm_deriv = fun_ty.normalize(env.clone());
            let arg_norm_deriv = self.arg.clone().normalize(env.clone());
            fun_norm = fun_norm_deriv.ret_ty();
            arg_norm = arg_norm_deriv.ret_ty();
            premises.push(fun_norm_deriv);
            premises.push(arg_norm_deriv);
        } else {
            fun_norm = fun_ty;
            arg_norm = self.arg.clone();
        }

        if let Ok(forall) = fun_norm.clone().into_forall() {
            if features.kinded {
                let arg_kind = arg_norm.check_kind(env.clone())?;
                forall.kind.check_equal(&arg_kind)?;
            }

            let ty = forall.ty.subst_type(&forall.var, &arg_norm);
            let conc = TypingConclusion::new(env.clone(), self.clone(), Rc::unwrap_or_clone(ty));
            let deriv = TypingDerivation::tyapp(conc, premises);
            Ok(deriv.into())
        } else if let Ok(forall) = fun_norm.clone().into_forall_bounded() {
            if features.kinded {
                let arg_kind = arg_norm.check_kind(env.clone())?;
                let sup_knd = forall.sup_ty.check_kind(env.clone())?;
                sup_knd.check_equal(&arg_kind)?;
            }
            arg_norm.check_subtype(&forall.sup_ty, env.clone())?;
            let ty = forall.ty.subst_type(&forall.var, &arg_norm);
            let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(ty));
            let deriv = TypingDerivation::tyapp_bounded(conc, premises);
            Ok(deriv.into())
        } else {
            Err(TypeMismatch::new(fun_norm.to_string(), "Universal Type".to_owned()).into())
        }
    }
}
