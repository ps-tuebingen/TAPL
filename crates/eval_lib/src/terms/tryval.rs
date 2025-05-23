impl<T> Eval for TryWithVal<T>
where
    T: LanguageTerm,
    Raise<T>: Into<<T as LanguageTerm>::Value>,
    App<T>: Into<T>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_evaled = self.term.eval(env)?;
        if let Ok(raise) = term_evaled.clone().into_raise() {
            let raise_term: T = (*raise.val).into();
            App::new(*self.handler, raise_term).into().eval(env)
        } else {
            Ok(term_evaled)
        }
    }
}
