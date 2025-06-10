use crate::Eval;
use syntax::{
    terms::{Term, Variant},
    types::Type,
    values::Variant as VariantVal,
};

impl<T, Ty> Eval for Variant<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    VariantVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        Ok(VariantVal::<<T as Eval>::Value, Ty>::new(&self.label, term_val, self.ty).into())
    }
}
