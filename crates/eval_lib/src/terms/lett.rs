use crate::Eval;
use syntax::{
    subst::SubstTerm,
    terms::{Let, Term},
};

impl<T> Eval for Let<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let bound_val = self.bound_term.eval(env)?;
        self.in_term.subst(&self.var, &bound_val.into()).eval(env)
    }
}
