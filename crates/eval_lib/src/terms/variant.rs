use crate::{values::Variant as VariantVal, Eval};
use common::errors::Error;
use syntax::{
    terms::{Term, Variant},
    types::Type,
};

impl<T, Ty> Eval for Variant<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    VariantVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(VariantVal::<<T as Eval>::Value, Ty>::new(&self.label, term_val, self.ty).into())
    }
}
