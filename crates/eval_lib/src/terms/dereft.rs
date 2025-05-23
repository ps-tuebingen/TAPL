
impl<T> Eval for Deref<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let loc_val = term_val.into_loc().map_err(to_eval_err)?;
        env.get_location(loc_val.loc).map_err(to_eval_err)
    }
}
