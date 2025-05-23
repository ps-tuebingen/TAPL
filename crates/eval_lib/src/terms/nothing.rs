impl<T> Eval for Nothing<T>
where
    T: LanguageTerm,
    NothingVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NothingVal::<T>::new(self.ty).into())
    }
}
