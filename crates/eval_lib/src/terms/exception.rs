use crate::Eval;
use syntax::{
    terms::{Exception, Term},
    types::Type,
    values::Exception as ExceptionVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Exception<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    ExceptionVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(ExceptionVal::new(self.ty).into())
    }
}
