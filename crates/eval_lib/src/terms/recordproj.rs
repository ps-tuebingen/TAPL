impl<T> Eval for RecordProj<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.record.eval(env)?;
        let rec_val = term_val.into_record().map_err(to_eval_err)?;
        rec_val
            .records
            .get(&self.label)
            .ok_or(to_eval_err(ErrorKind::UndefinedLabel(self.label.clone())))
            .cloned()
    }
}
