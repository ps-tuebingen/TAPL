use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{Term, Unpack},
    types::Type,
    values::ValueGroup,
};

impl<T, Ty> Eval for Unpack<T, Ty>
where
    T: Term
        + Eval
        + SubstTerm<T, Target = T>
        + SubstType<Ty, Target = T>
        + From<<T as Eval>::Value>,
    <T as Eval>::Value: ValueGroup<Type = Ty>,
    <T as Eval>::EvalError: From<ValueMismatch>,
    Ty: Type + SubstType<Ty, Target = Ty>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.bound_term.eval(env)?;
        let pack_val = term_val.into_pack()?;
        self.in_term
            .subst_type(&self.ty_name, &pack_val.inner_ty)
            .subst(&self.term_name, &((*pack_val.val).into()))
            .eval(env)
    }
}
