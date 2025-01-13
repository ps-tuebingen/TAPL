use super::Eval;
use crate::{
    errors::Error,
    terms::{Fix, Term},
    traits::{is_value::IsValue, subst::SubstTerm},
};

impl Eval for Fix {
    fn eval_once(self) -> Result<Term, Error> {
        if self.term.is_value() {
            let lam = self
                .term
                .as_lambda()
                .map_err(|knd| Error::eval(knd, &self))?;
            Ok(lam
                .body
                .clone()
                .subst(lam.var.clone(), Fix::new(lam.into()).into()))
        } else {
            let term_evaled = self.term.eval_once()?;
            Ok(Fix::new(term_evaled).into())
        }
    }
}
