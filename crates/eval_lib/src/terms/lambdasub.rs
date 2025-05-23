impl<T> Eval for LambdaSub<T>
where
    T: LanguageTerm,
    LambdaSubVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LambdaSubVal::new(&self.var, self.sup_ty, *self.body).into())
    }
}
