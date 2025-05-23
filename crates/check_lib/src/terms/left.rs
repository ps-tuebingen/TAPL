impl<T> Typecheck for Left<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let left_ty = self
            .left_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let left_knd = left_ty.check_kind(&mut env.clone())?;
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let sum_ty = ty_norm.into_sum().map_err(to_check_err)?;
        let sum_kind = sum_ty.check_kind(env)?;
        left_knd.check_equal(&sum_kind).map_err(to_check_err)?;
        sum_ty.left.check_equal(&left_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}
