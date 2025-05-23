impl<T> Eval for Fold<T>
where
    T: LanguageTerm,
    FoldVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(FoldVal::<T>::new(self.ty, term_val).into())
    }
}
