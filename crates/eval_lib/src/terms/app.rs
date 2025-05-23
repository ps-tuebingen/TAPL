impl<T> Eval for App<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as LanguageTerm>::Value;

    fn eval(self, env: &mut <T as Eval>::Env) -> Result<<T as LanguageTerm>::Value, Error> {
        let fun_val = self.fun.eval(env)?;

        let lam = fun_val.into_lambda().map_err(to_eval_err)?;
        let arg_val: <T as LanguageTerm>::Value = self.arg.eval(env)?;
        lam.body.subst(&lam.var, &arg_val.into()).eval(env)
    }
}
