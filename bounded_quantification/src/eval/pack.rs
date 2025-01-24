use super::{Eval, Value};
use crate::{
    errors::Error,
    syntax::{Pack, Unpack},
    traits::{SubstTerm, SubstTy},
};

impl Eval for Pack {
    fn eval(self) -> Result<Value, Error> {
        let t_evaled = self.term.eval()?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(t_evaled),
            outer_ty: self.outer_ty,
        })
    }
}
impl Eval for Unpack {
    fn eval(self) -> Result<Value, Error> {
        let bound_evaled = self.bound_term.clone().eval()?;
        let (inner_ty, val, _) = bound_evaled
            .as_pack()
            .map_err(|knd| Error::eval(knd, &self))?;
        self.in_term
            .subst(&self.bound_var, val.into())
            .subst_ty(&self.ty_var, inner_ty)
            .eval()
    }
}
