impl<T> Eval for Nil<T>
where
    T: LanguageTerm,
    NilVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(NilVal::<T>::new(self.ty).into())
    }
}
