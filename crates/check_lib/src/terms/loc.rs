impl<T> Typecheck for Loc<T>
where
    T: LanguageTerm,
    Reference<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let loc_ty = env
            .get_loc(&self.loc)
            .map_err(to_check_err)?
            .normalize(&mut env.clone());
        loc_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Reference::new(loc_ty).into())
    }
}
