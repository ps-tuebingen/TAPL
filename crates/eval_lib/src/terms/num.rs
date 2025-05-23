impl<T> Eval for Num<T>
where
    T: LanguageTerm,
    NumVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NumVal::new(self.num).into())
    }
}
