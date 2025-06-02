use crate::{
    errors::{ValueKind, ValueMismatch},
    values::ValueGroup,
    Eval,
};
use syntax::{
    subst::SubstType,
    terms::{Term, TyApp},
    types::Type,
};

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

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let fun_val = self.fun.eval(env)?;
        if let Ok(tylam) = fun_val.clone().into_tylambda() {
            tylam.term.subst_type(&tylam.var, &self.arg).eval(env)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            lamsub.term.subst_type(&lamsub.var, &self.arg).eval(env)
        } else {
            Err(ValueMismatch::new(&fun_val, ValueKind::LambdaSub).into())
        }
    }
}
