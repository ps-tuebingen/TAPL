use super::Eval;
use crate::{
    errors::Error,
    terms::{Let, Term},
    traits::{is_value::IsValue, subst::SubstTerm},
};

impl Eval for Let {
    fn eval_once(self) -> Result<Term, Error> {
        if self.bound_term.is_value() {
            Ok(self
                .in_term
                .clone()
                .subst(self.var, *self.bound_term.clone()))
        } else {
            let bound_evaled = self.bound_term.eval_once()?;
            Ok(Let::new(&self.var, bound_evaled, *self.in_term.clone()).into())
        }
    }
}
