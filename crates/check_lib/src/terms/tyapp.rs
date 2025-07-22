use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::{TypeMismatch, check_error::CheckError};
use syntax::{
    env::Environment,
    subst::SubstType,
    terms::{Term, TyApp},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for TyApp<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: TypeGroup + SubstType<Ty, Target = Ty> + Normalize<Ty> + Kindcheck<Ty> + Subtypecheck<Ty>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let fun_res = self.fun.check(env.clone())?;
        let fun_ty = fun_res.ty().normalize(env.clone());
        let arg_norm = self.arg.clone().normalize(env.clone());
        let arg_kind = arg_norm.check_kind(env.clone())?;
        if let Ok(forall) = fun_ty.clone().into_forall() {
            forall.kind.check_equal(&arg_kind)?;
            let ty = forall.ty.subst_type(&forall.var, &arg_norm);
            let conc = Conclusion::new(env.clone(), self.clone(), ty);
            let deriv = TypingDerivation::tyapp(conc, fun_res);
            Ok(deriv)
        } else if let Ok(forall) = fun_ty.clone().into_forall_bounded() {
            let sup_knd = forall.sup_ty.check_kind(env.clone())?;
            sup_knd.check_equal(&arg_kind)?;
            arg_norm.check_subtype(&forall.sup_ty, env.clone())?;
            let ty = forall.ty.subst_type(&forall.var, &arg_norm);
            let conc = Conclusion::new(env, self.clone(), ty);
            let deriv = TypingDerivation::tyapp_bounded(conc, fun_res);
            Ok(deriv)
        } else {
            Err(TypeMismatch::new(fun_ty.to_string(), "Universal Type".to_owned()).into())
        }
    }
}
