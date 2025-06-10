use crate::{env::EvalEnvironment, Eval};
use syntax::{
    terms::{Ref, Term},
    values::Loc,
};

impl<T> Eval for Ref<T>
where
    T: Term + Eval,
    Loc<T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.clone().eval(env)?;
        let fresh_loc = env.fresh_location();
        env.save_location(fresh_loc, term_val);
        Ok(Loc::new(fresh_loc).into())
    }
}
