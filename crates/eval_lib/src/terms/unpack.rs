use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{Term, Unpack},
    types::Type,
};

impl<T, Ty> Eval for Unpack<T, Ty>
where
    T: Term
        + Eval
        + SubstTerm<T, Target = T>
        + SubstType<Ty, Target = T>
        + From<<T as Eval>::Value>,
    <T as Eval>::Value: ValueGroup<Type = Ty>,
    Ty: Type + SubstType<Ty, Target = Ty>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.bound_term.eval(env)?;
        let pack_val = term_val.into_pack().map_err(to_eval_err)?;
        self.in_term
            .subst_type(&self.ty_name, &pack_val.inner_ty)
            .subst(&self.term_name, &((*pack_val.val).into()))
            .eval(env)
    }
}
