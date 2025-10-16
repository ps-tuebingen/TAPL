use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use errors::{NameMismatch, TypeMismatch};
use std::rc::Rc;
use syntax::{env::Environment, language::Language, terms::Unpack, types::TypeGroup};

impl<Lang> Typecheck for Unpack<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ret_ty().normalize(env.clone());
        if let Ok(bound_exists) = bound_ty.clone().into_exists() {
            if self.ty_name != bound_exists.var {
                return Err(NameMismatch::new(&bound_exists.var, &self.ty_name).into());
            }
            env.add_tyvar_kind(bound_exists.var, bound_exists.kind);
            env.add_var(self.term_name.clone(), Rc::unwrap_or_clone(bound_exists.ty));
            let in_res = self.in_term.check(env.clone())?;
            let in_ty = in_res.ret_ty().normalize(env.clone());
            let conc = TypingConclusion::new(env, self.clone(), in_ty);
            let deriv = TypingDerivation::unpack(conc, bound_res, in_res);
            Ok(deriv.into())
        } else if let Ok(bound_bound) = bound_ty.clone().into_exists_bounded() {
            if self.ty_name != bound_bound.var {
                return Err(NameMismatch::new(&bound_bound.var, &self.ty_name).into());
            }
            let sup_kind = bound_bound.sup_ty.check_kind(env.clone())?;
            env.add_tyvar_super(bound_bound.var, Rc::unwrap_or_clone(bound_bound.sup_ty));
            env.add_tyvar_kind(self.ty_name.clone(), sup_kind);
            env.add_var(self.term_name.clone(), Rc::unwrap_or_clone(bound_bound.ty));
            let inner_res = self.in_term.check(env.clone())?;
            let inner_ty = inner_res.ret_ty().normalize(env.clone());
            let conc = TypingConclusion::new(env.clone(), self.clone(), inner_ty);
            let deriv = TypingDerivation::unpack_bounded(conc, bound_res, inner_res);
            Ok(deriv.into())
        } else {
            Err(TypeMismatch::new(bound_ty.to_string(), "Existential Type".to_owned()).into())
        }
    }
}
