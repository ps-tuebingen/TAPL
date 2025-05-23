impl<T> Typecheck for Cons<T>
where
    T: LanguageTerm,
    List<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let hd_ty = self
            .head
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        hd_ty.check_equal(&ty_norm).map_err(to_check_err)?;
        hd_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;

        let tl_ty = self
            .tail
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        tl_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let list_ty: Self::Type = List::new(ty_norm).into();
        tl_ty.check_equal(&list_ty).map_err(to_check_err)?;
        Ok(list_ty)
    }
}
