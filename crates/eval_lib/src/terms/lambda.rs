use crate::Eval;
use syntax::{
    terms::{Lambda, Term},
    types::Type,
    values::Lambda as LambdaVal,
};

impl<T, Ty> Eval for Lambda<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    LambdaVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(LambdaVal::new(&self.var, self.annot, *self.body).into())
    }
}
