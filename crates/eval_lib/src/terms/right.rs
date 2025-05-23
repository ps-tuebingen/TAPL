impl<T> Eval for Right<T>
where
    T: LanguageTerm,
    RightVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let right_val = self.right_term.eval(env)?;
        Ok(RightVal::<T>::new(right_val, self.ty.clone()).into())
    }
}
