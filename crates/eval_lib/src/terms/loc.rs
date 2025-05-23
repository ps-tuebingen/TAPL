impl<T> Eval for Loc<T>
where
    T: LanguageTerm,
    LocVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LocVal::new(self.loc).into())
    }
}
