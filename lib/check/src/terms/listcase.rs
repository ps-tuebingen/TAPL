use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::ListCase, types::TypeGroup};

impl<Lang> Typecheck for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;
        let bound_list = bound_ty.clone().into_list()?;

        let nil_res = self.nil_rhs.check(env.clone())?;
        let nil_ty = nil_res.ret_ty().normalize(env.clone());
        let nil_kind = nil_ty.check_kind(env.clone())?;

        env.add_var(self.cons_fst.clone(), Rc::unwrap_or_clone(bound_list.ty));
        env.add_var(self.cons_rst.clone(), bound_ty);
        let cons_res = self.cons_rhs.check(env.clone())?;
        let cons_ty = nil_res.ret_ty().normalize(env.clone());
        let cons_kind = cons_ty.check_kind(env.clone())?;

        nil_kind.check_equal(&cons_kind)?;
        nil_ty.check_equal(&cons_ty)?;

        let conc = TypingConclusion::new(env.clone(), self.clone(), cons_ty);
        let deriv = TypingDerivation::listcase(conc, nil_res, cons_res);

        Ok(deriv.into())
    }
}
