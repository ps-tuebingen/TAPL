impl<T> Eval for Tuple<T>
where
    T: LanguageTerm,
    TupleVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut vals = vec![];
        for t in self.terms.into_iter() {
            let val = t.eval(env)?;
            vals.push(val);
        }
        Ok(TupleVal::<T>::new(vals).into())
    }
}
