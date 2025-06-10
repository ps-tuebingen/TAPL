use crate::Eval;
use syntax::{
    terms::{Term, Variant},
    types::Type,
    values::Variant as VariantVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Variant<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    VariantVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        Ok(VariantVal::<<T as Eval>::Value, Ty>::new(&self.label, term_val, self.ty).into())
    }
}
