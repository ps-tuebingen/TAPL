impl<T> Eval for Ascribe<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        self.term.eval(env)
    }
}
