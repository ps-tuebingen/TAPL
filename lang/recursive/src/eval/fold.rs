use super::Eval;
use crate::{
    errors::Error,
    terms::{Fold, Term, Unfold},
    traits::is_value::IsValue,
};

impl Eval for Fold {
    fn eval_once(self) -> Result<Term, Error> {
        //E-Fold
        if !self.term.is_value() {
            let t_evaled = self.term.eval_once()?;
            Ok(Fold {
                ty: self.ty,
                term: Box::new(t_evaled),
            }
            .into())
        } else {
            Ok(self.into())
        }
    }
}

impl Eval for Unfold {
    fn eval_once(self) -> Result<Term, Error> {
        //E-Unfold
        if !self.term.is_value() {
            let t_evaled = self.term.eval_once()?;
            Ok(Unfold {
                ty: self.ty,
                term: Box::new(t_evaled),
            }
            .into())
        //E-UnfoldFold
        } else {
            let fold = self.term.as_fold().map_err(|knd| Error::eval(knd, &self))?;
            Ok(*fold.term)
        }
    }
}
