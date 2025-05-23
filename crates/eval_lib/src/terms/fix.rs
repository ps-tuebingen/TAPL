impl<T> Eval for Fix<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let lam_val = term_val.into_lambda().map_err(to_eval_err)?;
        lam_val.body.subst(&lam_val.var, &self.into()).eval(env)
    }
}
