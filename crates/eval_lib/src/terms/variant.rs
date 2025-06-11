use crate::Eval;
use syntax::{
    terms::{Term, Variant},
    types::Type,
    values::Variant as VariantVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Variant<T, Ty>
where
    T: Term + Eval<Term = T>,
    Variant<T, Ty>: Into<T>,
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
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = VariantVal::<<T as Eval>::Value, Ty>::new(&self.label, term_val, self.ty.clone());

        let steps =
            term_res.congruence(&move |t| Variant::new(&self.label, t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }
}
