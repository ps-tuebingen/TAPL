impl<T> Typecheck for Num<T>
where
    T: LanguageTerm,
    Nat<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Nat::new().into())
    }
}
