impl<T> Eval for IsZero<T>
where
    T: LanguageTerm,
    True<T>: Into<<T as LanguageTerm>::Value>,
    False<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        if num.num == 0 {
            Ok(True::new().into())
        } else {
            Ok(False::new().into())
        }
    }
}
