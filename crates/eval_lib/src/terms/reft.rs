impl<T> Eval for Ref<T>
where
    T: LanguageTerm,
    LocVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let fresh_loc = env.fresh_location();
        env.save_location(fresh_loc, term_val);
        Ok(LocVal::new(fresh_loc).into())
    }
}
