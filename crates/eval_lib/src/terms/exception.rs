use crate::{values::Exception as ExceptionVal, Eval};
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> Eval for Exception<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    ExceptionVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(ExceptionVal::new(self.ty).into())
    }
}
