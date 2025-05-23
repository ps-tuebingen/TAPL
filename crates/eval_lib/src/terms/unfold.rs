impl<T> Eval for Unfold<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let term_fold = term_val.into_fold().map_err(to_eval_err)?;
        Ok(*term_fold.val)
    }
}
