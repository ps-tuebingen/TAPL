use super::{errors::Error, Eval, Value};
use crate::terms::{subst::Subst, syntax::Let};

impl Eval for Let {
    fn eval(self) -> Result<Value, Error> {
        let bound_val = self.bound_term.eval()?;
        self.in_term.subst(self.var, bound_val.into()).eval()
    }
}
