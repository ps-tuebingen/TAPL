impl<T> Typecheck for Variable<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        env.get_var(&self.var).map_err(to_check_err)
    }
}
