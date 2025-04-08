use super::{
    syntax::{
        App, Error as ErrT, If, IsZero, Lambda, Pred, Raise, Succ, Term, Try, TryWithVal, Unit, Var,
    },
    types::Type,
};
use common::Typecheck;
use std::collections::HashMap;

pub mod errors;
use errors::Error;

pub type Env = HashMap<Var, Type>;

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(v) => env.get(v).ok_or(Error::FreeVar(v.clone())).cloned(),
            Term::Const(_) => Ok(Type::Nat),
            Term::True => Ok(Type::Bool),
            Term::False => Ok(Type::Bool),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::If(ift) => ift.check(env),
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

impl<'a> Typecheck<'a> for Lambda {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        env.insert(self.var.clone(), self.annot.clone());
        let body_ty = self.body.check(env)?;
        Ok(Type::Fun {
            from: Box::new(self.annot.clone()),
            to: Box::new(body_ty),
        })
    }
}

impl<'a> Typecheck<'a> for App {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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

impl<'a> Typecheck<'a> for ErrT {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(self.ty.clone())
    }
}

impl<'a> Typecheck<'a> for Try {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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

impl<'a> Typecheck<'a> for Raise {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let err_ty = self.exception.check(env)?;
        if err_ty == self.ex_ty {
            Ok(self.cont_ty.clone())
        } else {
            Err(Error::TypeMismatch {
                found: err_ty,
                expected: self.ex_ty.clone(),
            })
        }
    }
}

impl<'a> Typecheck<'a> for TryWithVal {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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

impl<'a> Typecheck<'a> for Unit {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        Ok(Type::Unit)
    }
}

impl<'a> Typecheck<'a> for Succ {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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

impl<'a> Typecheck<'a> for IsZero {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Bool)
        } else {
            Err(Error::TypeMismatch {
                found: inner_ty,
                expected: Type::Nat,
            })
        }
    }
}

impl<'a> Typecheck<'a> for If {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let if_ty = self.ift.check(&mut env.clone())?;
        if if_ty != Type::Bool {
            return Err(Error::TypeMismatch {
                found: if_ty,
                expected: Type::Bool,
            });
        }
        let then_ty = self.thent.check(&mut env.clone())?;
        let else_ty = self.elset.check(env)?;
        if then_ty != else_ty {
            Err(Error::TypeMismatch {
                found: then_ty,
                expected: else_ty,
            })
        } else {
            Ok(then_ty)
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::Type;
    use crate::syntax::term_tests::{example_term1, example_term2};
    use common::Typecheck;

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
