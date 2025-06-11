use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{Term, Unpack},
    types::Type,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Unpack<T, Ty>
where
    T: Term
        + Eval<Term = T>
        + SubstTerm<T, Target = T>
        + SubstType<Ty, Target = T>
        + From<<T as Eval>::Value>,
    <T as Eval>::Value: ValueGroup<Type = Ty>,
    Unpack<T, Ty>: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
    Ty: Type + SubstType<Ty, Target = Ty>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;
    type Term = T;

    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_res = self.bound_term.eval(env)?;
        let term_val = term_res.val();
        let pack_val = term_val.clone().into_pack()?;
        let in_subst = self
            .in_term
            .clone()
            .subst_type(&self.ty_name, &pack_val.inner_ty)
            .subst(&self.term_name, &((*pack_val.val).into()));
        let next_step = EvalStep::unpackpack(
            Unpack::new(
                &self.ty_name,
                &self.term_name,
                term_val,
                *self.in_term.clone(),
            ),
            in_subst.clone(),
        );
        let in_res = in_subst.eval(env)?;
        let val = in_res.val();

        let mut steps = term_res.congruence(&move |t| {
            Unpack::new(&self.ty_name, &self.term_name, t, *self.in_term.clone()).into()
        });
        steps.push(next_step);
        steps.extend(in_res.steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
