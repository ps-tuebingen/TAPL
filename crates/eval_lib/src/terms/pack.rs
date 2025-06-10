use crate::Eval;
use syntax::{
    terms::{Pack, Term},
    types::Type,
    values::Pack as PackVal,
};

impl<T, Ty> Eval for Pack<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    PackVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        Ok(PackVal::<<T as Eval>::Value, Ty>::new(
            self.inner_ty.clone(),
            term_val,
            self.outer_ty.clone(),
        )
        .into())
    }
}
