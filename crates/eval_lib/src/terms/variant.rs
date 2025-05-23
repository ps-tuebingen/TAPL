impl<T> Eval for Variant<T>
where
    T: LanguageTerm,
    VariantVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(VariantVal::<T>::new(&self.label, term_val, self.ty).into())
    }
}
