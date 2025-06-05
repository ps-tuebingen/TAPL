use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::{KindMismatch, TypeKind, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    subst::SubstType,
    terms::{Term, TyApp},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for TyApp<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + SubstType<Ty, Target = Ty> + Normalize<Ty> + Kindcheck<Ty> + Subtypecheck<Ty>,
    <T as Typecheck>::CheckError: From<TypeMismatch>
        + From<KindMismatch>
        + From<<Ty as Kindcheck<Ty>>::CheckError>
        + From<<Ty as Subtypecheck<Ty>>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let fun_res = self.fun.check(&mut env.clone())?;
        let fun_ty = fun_res.ty().normalize(&mut env.clone());
        let arg_norm = self.arg.clone().normalize(&mut env.clone());
        let arg_kind = arg_norm.check_kind(&mut env.clone())?;
        if let Ok(forall) = fun_ty.clone().into_forall() {
            forall.kind.check_equal(&arg_kind)?;
            let ty = forall.ty.subst_type(&forall.var, &arg_norm);
            let conc = Conclusion::new(env.clone(), self.clone(), ty);
            let deriv = Derivation::tyapp(conc, fun_res);
            Ok(deriv)
        } else if let Ok(forall) = fun_ty.clone().into_forall_bounded() {
            let sup_knd = forall.sup_ty.check_kind(&mut env.clone())?;
            sup_knd.check_equal(&arg_kind)?;
            arg_norm.check_subtype(&forall.sup_ty, env)?;
            let ty = forall.ty.subst_type(&forall.var, &arg_norm);
            let conc = Conclusion::new(env.clone(), self.clone(), ty);
            let deriv = Derivation::tyapp_bounded(conc, fun_res);
            Ok(deriv)
        } else {
            Err(TypeMismatch::new(fun_ty.knd(), TypeKind::Universal).into())
        }
    }
}
