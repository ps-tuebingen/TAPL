impl<T> Typecheck for False<T>
where
    T: LanguageTerm,
    Bool<<T as LanguageTerm>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Bool::new().into())
    }
}
