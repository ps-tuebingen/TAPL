
impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, _: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}
