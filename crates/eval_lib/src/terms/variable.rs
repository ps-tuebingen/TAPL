impl<T> Eval for Variable<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Err(to_eval_err(ErrorKind::FreeVariable(self.var)))
    }
}
