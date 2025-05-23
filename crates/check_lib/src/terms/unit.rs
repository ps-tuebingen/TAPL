impl<T> Typecheck for Unit<T>
where
    T: LanguageTerm,
    UnitTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(UnitTy::new().into())
    }
}
