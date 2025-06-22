use crate::{Eval, errors::EvalError};
use syntax::{
    store::Store,
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
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = VariantVal::<<T as Eval>::Value, Ty>::new(&self.label, term_val, self.ty.clone());

        let steps =
            term_res.congruence(&move |t| Variant::new(&self.label, t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }
}
