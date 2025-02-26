use super::{Eval, Value};
use crate::{
    errors::Error,
    syntax::terms::Unpack,
    traits::{SubstTerm, SubstTy},
};

impl Eval for Unpack {
    fn eval(self) -> Result<Value, Error> {
        let bound_val = self.bound_term.clone().eval()?;
        let (inner, val, _) = bound_val.as_pack().map_err(|knd| Error::eval(knd, &self))?;
        self.in_term
            .subst(&self.bound_var, val.into())
            .subst_ty(&self.ty_var, inner)
            .eval()
    }
}
