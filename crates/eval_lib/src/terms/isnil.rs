use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    terms::{False as FalseT, IsNil, Term, True as TrueT},
    types::Type,
    values::{False, True, Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for IsNil<T, Ty>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
    IsNil<T, Ty>: Into<T>,
    True<T>: Into<<T as Eval>::Value>,
    TrueT<T>: Into<T>,
    False<T>: Into<<T as Eval>::Value>,
    FalseT<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let mut steps = term_res.congruence(&move |t| IsNil::new(t, self.ty.clone()).into());
        if term_val.clone().into_nil().is_ok() {
            let last_step = EvalStep::isnil_true(*self.ty.clone());
            steps.push(last_step);
            Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, True::new()));
        } else if term_val.clone().into_cons().is_ok() {
            let last_step = EvalStep::isnil_false(self.ty.clone());
            steps.push(last_step);
            Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, False::new()))
        } else {
            Err(ValueMismatch::new(term_val.knd(), ValueKind::List).into())
        }
    }
}
