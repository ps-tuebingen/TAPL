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
        let ty_norm = self.ty.clone().normalize(env.clone());
        let hd_res = self.head.check(env.clone())?;
        let hd_ty = hd_res.ret_ty().normalize(env.clone());
        hd_ty.check_equal(&ty_norm)?;
        hd_ty.check_kind(env.clone())?.into_star()?;

        let tl_res = self.tail.check(env.clone())?;
        let tl_ty = tl_res.ret_ty().normalize(env.clone());
        tl_ty.check_kind(env.clone())?.into_star()?;
        let list_ty: Lang::Type = List::new(ty_norm).into();
        tl_ty.check_equal(&list_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), list_ty.clone());
        let deriv = TypingDerivation::cons(conc, hd_res, tl_res);
        Ok(deriv.into())
    }
}
