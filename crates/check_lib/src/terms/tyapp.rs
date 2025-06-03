use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::{KindMismatch, TypeKind, TypeMismatch};
use syntax::{
    env::Environment,
    subst::SubstType,
    terms::{Term, TyApp},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for TyApp<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + SubstType<Ty, Target = Ty> + Normalize<Ty> + Kindcheck<Ty> + Subtypecheck<Ty>,
    <T as Typecheck>::CheckError: From<TypeMismatch>
        + From<KindMismatch>
        + From<<Ty as Kindcheck<Ty>>::CheckError>
        + From<<Ty as Subtypecheck<Ty>>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let fun_ty = self
            .fun
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let arg_norm = self.arg.clone().normalize(&mut env.clone());
        let arg_kind = arg_norm.check_kind(&mut env.clone())?;
        if let Ok(forall) = fun_ty.clone().into_forall() {
            forall.kind.check_equal(&arg_kind)?;
            Ok(forall.ty.subst_type(&forall.var, &arg_norm))
        } else if let Ok(forall) = fun_ty.clone().into_forall_bounded() {
            let sup_knd = forall.sup_ty.check_kind(&mut env.clone())?;
            sup_knd.check_equal(&arg_kind)?;
            arg_norm.check_subtype(&forall.sup_ty, env)?;
            Ok(forall.ty.subst_type(&forall.var, &arg_norm))
        } else {
            Err(TypeMismatch::new(fun_ty.knd(), TypeKind::Universal).into())
        }
    }
}
