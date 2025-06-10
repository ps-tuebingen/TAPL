use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    subst::SubstType,
    terms::{Term, TyApp},
    types::Type,
    values::{Value, ValueGroup},
};
use trace::EvalTrace;

impl<T, Ty> Eval for TyApp<T, Ty>
where
    T: Term + Eval + SubstType<Ty, Target = T>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Ty: Type,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let fun_val = self.fun.eval(env)?;
        if let Ok(tylam) = fun_val.clone().into_tylambda() {
            tylam.term.subst_type(&tylam.var, &self.arg).eval(env)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            lamsub.term.subst_type(&lamsub.var, &self.arg).eval(env)
        } else {
            Err(ValueMismatch::new(fun_val.knd(), ValueKind::LambdaSub).into())
        }
    }
}
