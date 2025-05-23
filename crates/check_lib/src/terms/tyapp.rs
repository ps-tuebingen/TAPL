impl<T> Typecheck for TyApp<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let fun_ty = self
            .fun
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let arg_norm = self.arg.clone().normalize(&mut env.clone());
        let arg_kind = arg_norm.check_kind(&mut env.clone())?;
        if let Ok(forall) = fun_ty.clone().into_forall() {
            forall.kind.check_equal(&arg_kind).map_err(to_check_err)?;
            Ok(forall.ty.subst_type(&forall.var, &arg_norm))
        } else if let Ok(forall) = fun_ty.clone().into_forall_bounded() {
            let sup_knd = forall.sup_ty.check_kind(&mut env.clone())?;
            sup_knd.check_equal(&arg_kind).map_err(to_check_err)?;
            arg_norm.check_subtype(&forall.sup_ty, env)?;
            Ok(forall.ty.subst_type(&forall.var, &arg_norm))
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: fun_ty.to_string(),
                expected: "Universal Type".to_owned(),
            }))
        }
    }
}
