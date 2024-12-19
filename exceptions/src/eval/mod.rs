use super::{
    syntax::{App, Error as ErrT, Lambda, Raise, Term, Try, TryWithVal, Unit},
    traits::subst::Subst,
};

pub mod errors;
pub mod values;
use errors::Error;
use values::Value;

pub trait Eval {
    fn eval_once(self) -> Result<Term, Error>;
    fn eval(self) -> Result<Value, Error>
    where
        Self: Into<Term> + Clone,
    {
        let evaled = self.clone().eval_once()?;
        if evaled == self.into() {
            match evaled {
                Term::Lambda(lam) => Ok(lam.into()),
                Term::Unit(u) => Ok(u.into()),
                Term::Error(_) => Err(Error::Exception),
                Term::Raise(r) => match *r.exception {
                    Term::Lambda(lam) => Err(Error::ExceptionVal(lam.into())),
                    Term::Unit(u) => Err(Error::ExceptionVal(u.into())),
                    _ => Err(Error::Stuck(r.into())),
                },
                _ => Err(Error::Stuck(evaled)),
            }
        } else {
            evaled.eval()
        }
    }
}

impl Eval for Term {
    fn eval_once(self) -> Result<Term, Error> {
        match self {
            Term::Var(v) => Err(Error::FreeVar(v)),
            Term::Lambda(lam) => lam.eval_once(),
            Term::App(app) => app.eval_once(),
            Term::Unit(u) => u.eval_once(),
            Term::Error(e) => e.eval_once(),
            Term::Try(t) => t.eval_once(),
            Term::TryWithVal(t) => t.eval_once(),
            Term::Raise(r) => r.eval_once(),
        }
    }
}

impl Eval for Lambda {
    fn eval_once(self) -> Result<Term, Error> {
        //Lambda is Value
        Ok(self.into())
    }
}

impl Eval for App {
    fn eval_once(self) -> Result<Term, Error> {
        match *self.fun {
            // E-AppRaise1
            Term::Raise(r) if r.exception.is_value() => Ok(r.into()),
            // E-AppErr1
            Term::Error(err) => Ok(err.into()),
            // E-AppAbs
            Term::Lambda(lam) if self.arg.is_value() => Ok(lam.body.subst(&lam.var, *self.arg)),
            Term::Lambda(lam) => {
                // E-AppRaise2
                if let Term::Raise(r) = *self.arg {
                    if r.exception.is_value() {
                        Ok(r.into())
                    } else {
                        //E-App2
                        let arg_evaled = r.eval_once()?;
                        Ok(App::new(lam.into(), arg_evaled).into())
                    }
                } else
                // E-AppErr2
                if let Term::Error(_) = *self.arg {
                    return Err(Error::Exception);
                // E-App2
                } else {
                    let arg_evaled = self.arg.eval_once()?;
                    Ok(App::new(lam.into(), arg_evaled).into())
                }
            }
            // E-App1
            _ => {
                let fun_evaled = self.fun.eval_once()?;
                Ok(App::new(fun_evaled, *self.arg).into())
            }
        }
    }
}

impl Eval for Unit {
    fn eval_once(self) -> Result<Term, Error> {
        //Unit is Value
        Ok(self.into())
    }
}

impl Eval for ErrT {
    fn eval_once(self) -> Result<Term, Error> {
        //Err is error
        Err(Error::Exception)
    }
}

impl Eval for Try {
    fn eval_once(self) -> Result<Term, Error> {
        //E-TryV
        if self.term.is_value() {
            return Ok(*self.term);
        }
        //E-TryError
        if let Term::Error(_) = *self.term {
            return Ok(*self.handler);
        }

        //E-Try
        let term_evaled = self.term.eval_once()?;
        Ok(Try::new(term_evaled, *self.handler).into())
    }
}

impl Eval for TryWithVal {
    fn eval_once(self) -> Result<Term, Error> {
        // E-TryV
        if self.term.is_value() {
            return Ok(*self.term);
        }
        if let Term::Raise(r) = *self.term {
            // E-TryRaise
            if r.exception.is_value() {
                return Ok(App::new(*self.handler, *r.exception).into());
            // E-Try
            } else {
                let r_evaled = r.eval_once()?;
                return Ok(TryWithVal::new(r_evaled, *self.handler).into());
            }
        }

        //E-Try
        let t_evaled = self.term.eval_once()?;
        Ok(TryWithVal::new(t_evaled, *self.handler).into())
    }
}

impl Eval for Raise {
    fn eval_once(self) -> Result<Term, Error> {
        if let Term::Raise(r) = *self.exception {
            // E-RaiseRaise
            if r.exception.is_value() {
                Ok(r.into())
            // E-Raise
            } else {
                let r_evaled = r.eval_once()?;
                Ok(Raise::new(r_evaled, self.ex_ty, self.cont_ty).into())
            }
        // E-Raise
        } else {
            let t_evaled = self.exception.eval_once()?;
            Ok(Raise::new(t_evaled, self.ex_ty, self.cont_ty).into())
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::{Eval, Value};
    use crate::syntax::term_tests::{example_term1, example_term2};

    #[test]
    fn eval1() {
        let result = example_term1().eval().unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval().unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
