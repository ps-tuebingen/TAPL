impl<T> Eval for Lambda<T>
where
    T: LanguageTerm,
    LambdaVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LambdaVal::new(&self.var, self.annot, *self.body).into())
    }
}
