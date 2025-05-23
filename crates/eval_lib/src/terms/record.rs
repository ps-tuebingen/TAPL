impl<T> Eval for Record<T>
where
    T: LanguageTerm,
    RecordVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut recs: HashMap<Label, Self::Value> = HashMap::new();
        for (lb, t) in self.records.into_iter() {
            let val = t.eval(env)?;
            recs.insert(lb, val);
        }
        Ok(RecordVal::<T>::new::<Self::Value>(recs).into())
    }
}
