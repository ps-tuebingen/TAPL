use crate::Eval;
use syntax::{
    terms::{App, Term, TryWithVal},
    values::{Raise, ValueGroup},
};
use trace::EvalTrace;

impl<T> Eval for TryWithVal<T>
where
    T: Term + Eval + From<<T as Eval>::Value>,
    Raise<<T as Eval>::Value, <<T as Eval>::Value as ValueGroup>::Type>: Into<<T as Eval>::Value>,
    App<T>: Into<T>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_evaled = self.term.eval(env)?;
        if let Ok(raise) = term_evaled.clone().into_raise() {
            let raise_term: T = (*raise.val).into();
            App::new(*self.handler, raise_term).into().eval(env)
        } else {
            Ok(term_evaled)
        }
    }
}
