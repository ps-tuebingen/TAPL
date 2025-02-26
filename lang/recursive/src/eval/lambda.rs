use super::Eval;
use crate::{
    errors::Error,
    terms::{App, Lambda, Term},
    traits::{is_value::IsValue, subst::SubstTerm},
};

impl Eval for Lambda {
    fn eval_once(self) -> Result<Term, Error> {
        Ok(self.into())
    }
}

impl Eval for App {
    fn eval_once(self) -> Result<Term, Error> {
        //E-App1
        if !self.fun.is_value() {
            let fun_evaled = self.fun.eval_once()?;
            Ok(App {
                fun: Box::new(fun_evaled),
                arg: self.arg,
            }
            .into())
        //E-App2
        } else if !self.arg.is_value() {
            let arg_evaled = self.arg.eval_once()?;
            Ok(App {
                fun: self.fun,
                arg: Box::new(arg_evaled),
            }
            .into())
        //A-AppAbs
        } else {
            let lam = self
                .fun
                .as_lambda()
                .map_err(|knd| Error::eval(knd, &self))?;
            Ok(lam.body.subst(lam.var, *self.arg))
        }
    }
}
