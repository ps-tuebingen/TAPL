impl<T> Eval for Raise<T>
where
    T: LanguageTerm,
    RaiseVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let exc_val = self.exception.eval(env)?;
        let raise_val = RaiseVal::<T>::new(exc_val, self.cont_ty, self.exception_ty);
        Ok(raise_val.into())
    }
}
