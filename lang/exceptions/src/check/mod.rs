use super::{
    syntax::{
        App, Error as ErrT, IsZero, Lambda, Pred, Raise, Succ, Term, Try, TryWithVal, Unit, Var,
    },
    types::Type,
};
use std::collections::HashMap;

pub mod errors;
use errors::Error;

pub type Env = HashMap<Var, Type>;

pub trait Check {
    fn check(self, env: &mut Env) -> Result<Type, Error>;
}

impl Check for Term {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        match self {
            Term::Var(v) => env.get(&v).ok_or(Error::FreeVar(v)).cloned(),
            Term::Const(_) => Ok(Type::Nat),
            Term::True => Ok(Type::Bool),
            Term::False => Ok(Type::Bool),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::Error(err) => err.check(env),
            Term::Try(t) => t.check(env),
            Term::Raise(r) => r.check(env),
            Term::TryWithVal(t) => t.check(env),
        }
    }
}

impl Check for Lambda {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        env.insert(self.var, self.annot.clone());
        let body_ty = self.body.check(env)?;
        Ok(Type::Fun {
            from: Box::new(self.annot),
            to: Box::new(body_ty),
        })
    }
}

impl Check for App {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        if let Type::Fun { from, to } = fun_ty {
            let arg_ty = self.arg.check(env)?;
            if *from == arg_ty {
                Ok(*to)
            } else {
                Err(Error::TypeMismatch {
                    found: arg_ty,
                    expected: *from,
                })
            }
        } else {
            Err(Error::NotAFunctionType(fun_ty))
        }
    }
}

impl Check for ErrT {
    fn check(self, _: &mut Env) -> Result<Type, Error> {
        Ok(self.ty)
    }
}

impl Check for Try {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let term_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        if term_ty == handler_ty {
            Ok(term_ty)
        } else {
            Err(Error::TypeMismatch {
                found: term_ty,
                expected: handler_ty,
            })
        }
    }
}

impl Check for Raise {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let err_ty = self.exception.check(env)?;
        if err_ty == self.ex_ty {
            Ok(self.cont_ty)
        } else {
            Err(Error::TypeMismatch {
                found: err_ty,
                expected: self.ex_ty,
            })
        }
    }
}

impl Check for TryWithVal {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let t_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        if let Type::Fun { from: _, to } = handler_ty {
            if t_ty == *to {
                Ok(t_ty)
            } else {
                Err(Error::TypeMismatch {
                    found: t_ty,
                    expected: *to,
                })
            }
        } else {
            Err(Error::NotAFunctionType(handler_ty))
        }
    }
}

impl Check for Unit {
    fn check(self, _: &mut Env) -> Result<Type, Error> {
        Ok(Type::Unit)
    }
}

impl Check for Succ {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::TypeMismatch {
                found: inner_ty,
                expected: Type::Nat,
            })
        }
    }
}

impl Check for Pred {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::TypeMismatch {
                found: inner_ty,
                expected: Type::Nat,
            })
        }
    }
}

impl Check for IsZero {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(Error::TypeMismatch {
                found: inner_ty,
                expected: Type::Nat,
            })
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::{Check, Type};
    use crate::syntax::term_tests::{example_term1, example_term2};

    #[test]
    fn check1() {
        let result = example_term1().check(&mut Default::default()).unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check2() {
        let result = example_term2().check(&mut Default::default()).unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}
