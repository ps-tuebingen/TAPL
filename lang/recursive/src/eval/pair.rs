use super::Eval;
use crate::{
    errors::Error,
    terms::{Fst, Pair, Snd, Term},
    traits::is_value::IsValue,
};

impl Eval for Pair {
    fn eval_once(self) -> Result<Term, Error> {
        //E-Pair1
        if !self.fst.is_value() {
            let fst_evaled = self.fst.eval_once()?;
            Ok(Pair {
                fst: Box::new(fst_evaled),
                snd: self.snd,
            }
            .into())
        //E-Pair2
        } else if !self.snd.is_value() {
            let snd_evaled = self.snd.eval_once()?;
            Ok(Pair {
                fst: self.fst,
                snd: Box::new(snd_evaled),
            }
            .into())
        } else {
            Ok(self.into())
        }
    }
}

impl Eval for Fst {
    fn eval_once(self) -> Result<Term, Error> {
        //E-Fst
        if !self.term.is_value() {
            let term_evaled = self.term.eval_once()?;
            Ok(Fst {
                term: Box::new(term_evaled),
            }
            .into())
            //E-FstBeta
        } else {
            let pair = self.term.as_pair().map_err(|knd| Error::eval(knd, &self))?;
            Ok(*pair.fst)
        }
    }
}

impl Eval for Snd {
    fn eval_once(self) -> Result<Term, Error> {
        //E-Snd
        if !self.term.is_value() {
            let term_evaled = self.term.eval_once()?;
            Ok(Snd {
                term: Box::new(term_evaled),
            }
            .into())
        //E-SndBeta
        } else {
            let pair = self.term.as_pair().map_err(|knd| Error::eval(knd, &self))?;
            Ok(*pair.snd)
        }
    }
}
