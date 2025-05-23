impl<T> Eval for True<T>
where
    T: LanguageTerm,
    TrueVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TrueVal::new().into())
    }
}
