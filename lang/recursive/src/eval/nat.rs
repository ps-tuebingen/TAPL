use super::Eval;
use crate::{
    errors::{Error, ErrorKind},
    terms::{False, IsZero, Pred, Succ, Term, True, Zero},
    traits::is_value::IsValue,
};

impl Eval for Zero {
    fn eval_once(self) -> Result<Term, Error> {
        Ok(Zero.into())
    }
}

impl Eval for Succ {
    fn eval_once(self) -> Result<Term, Error> {
        if self.term.is_value() {
            if let Term::Pred(pred) = *self.term {
                Ok(*pred.term)
            } else {
                Ok(self.into())
            }
        } else {
            let t_evaled = self.term.eval_once()?;
            Ok(Succ {
                term: Box::new(t_evaled),
            }
            .into())
        }
    }
}

impl Eval for Pred {
    fn eval_once(self) -> Result<Term, Error> {
        if self.term.is_value() {
            if let Term::Succ(succ) = *self.term {
                Ok(*succ.term)
            } else {
                Ok(self.into())
            }
        } else {
            let t_evaled = self.term.eval_once()?;
            Ok(Succ {
                term: Box::new(t_evaled),
            }
            .into())
        }
    }
}

impl Eval for IsZero {
    fn eval_once(self) -> Result<Term, Error> {
        if self.term.is_value() {
            match *self.term {
                Term::Zero(_) => Ok(True.into()),
                Term::Succ(_) => Ok(False.into()),
                Term::Pred(_) => Ok(False.into()),
                t => Err(Error::eval(
                    ErrorKind::unexpected_term(&t, "Natural number"),
                    &IsZero { term: Box::new(t) },
                )),
            }
        } else {
            let term_evaled = self.term.eval()?;
            Ok(IsZero::new(term_evaled).into())
        }
    }
}
