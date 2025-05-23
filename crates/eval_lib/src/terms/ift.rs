impl<T> Eval for If<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let cond_val = self.if_cond.eval(env)?;
        let cond_val_str = cond_val.to_string();
        if cond_val.clone().into_true().is_ok() {
            self.then_term.eval(env)
        } else if cond_val.into_false().is_ok() {
            self.else_term.eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: cond_val_str,
                expected: "Boolean".to_owned(),
            }))
        }
    }
}
