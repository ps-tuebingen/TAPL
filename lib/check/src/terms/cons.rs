use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    terms::Cons,
    types::{List, TypeGroup},
};

impl<Lang> Typecheck for Cons<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    List<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        let hd_res = self.head.check(env.clone())?;
        let hd_ty = hd_res.ret_ty();
        premises.push(hd_res);

        let hd_norm;
        if features.normalizing {
            let hd_norm_deriv = hd_ty.normalize(env.clone());
            hd_norm = hd_norm_deriv.ret_ty();
            premises.push(hd_norm_deriv);
        } else {
            hd_norm = hd_ty;
        }

        hd_norm.check_equal(&ty_norm)?;

        if features.kinded {
            hd_norm.check_kind(env.clone())?.into_star()?;
        }

        let tl_res = self.tail.check(env.clone())?;
        let tl_ty = tl_res.ret_ty();
        premises.push(tl_res);

        let tl_norm;
        if features.normalizing {
            let tl_norm_deriv = tl_ty.normalize(env.clone());
            tl_norm = tl_norm_deriv.ret_ty();
            premises.push(tl_norm_deriv);
        } else {
            tl_norm = tl_ty;
        }

        if features.kinded {
            tl_norm.check_kind(env.clone())?.into_star()?;
        }

        let list_ty: Lang::Type = List::new(ty_norm).into();
        tl_norm.check_equal(&list_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), list_ty.clone());
        let deriv = TypingDerivation::cons(conc, premises);
        Ok(deriv.into())
    }
}
