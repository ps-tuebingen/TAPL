impl<T> Eval for TyLambda<T>
where
    T: LanguageTerm,
    TyLambdaVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(TyLambdaVal::new(&self.var, self.annot, *self.term).into())
    }
}
