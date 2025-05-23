impl<T> Eval for Left<T>
where
    T: LanguageTerm,
    LeftVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let left_val = self.left_term.eval(env)?;
        Ok(LeftVal::<T>::new(left_val, self.ty).into())
    }
}
