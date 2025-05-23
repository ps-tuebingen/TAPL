impl<T> Eval for Unit<T>
where
    T: LanguageTerm,
    UnitVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(UnitVal::new().into())
    }
}
