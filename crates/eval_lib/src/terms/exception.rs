
impl<T> Eval for Exception<T>
where
    T: LanguageTerm,
    ExceptionVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(ExceptionVal::new(self.ty).into())
    }
}
