impl<T> Eval for Something<T>
where
    T: LanguageTerm,
    SomethingVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(SomethingVal::<T>::new(term_val).into())
    }
}
