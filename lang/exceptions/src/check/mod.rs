use super::{
    syntax::{
        App, Error as ErrT, If, IsZero, Lambda, Pred, Raise, Succ, Term, Try, TryWithVal, Unit, Var,
    },
    to_err,
    types::Type,
};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Typecheck,
};
use std::collections::HashMap;

pub type Env = HashMap<Var, Type>;

fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(v) => env
                .get(v)
                .ok_or(ErrorKind::FreeVariable(v.clone()))
                .map_err(to_check_err)
                .cloned(),
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
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
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
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        if let Type::Fun { from, to } = fun_ty {
            let arg_ty = self.arg.check(env)?;
            if *from == arg_ty {
                Ok(*to)
            } else {
                Err(to_check_err(ErrorKind::TypeMismatch {
                    found: arg_ty.to_string(),
                    expected: from.to_string(),
                }))
            }
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: fun_ty.to_string(),
                expected: "Function Type".to_owned(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for ErrT {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(self.ty.clone())
    }
}

impl<'a> Typecheck<'a> for Try {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        if term_ty == handler_ty {
            Ok(term_ty)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: term_ty.to_string(),
                expected: handler_ty.to_string(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for Raise {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let err_ty = self.exception.check(env)?;
        if err_ty == self.ex_ty {
            Ok(self.cont_ty.clone())
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: err_ty.to_string(),
                expected: self.ex_ty.to_string(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for TryWithVal {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let t_ty = self.term.check(&mut env.clone())?;
        let handler_ty = self.handler.check(env)?;
        if let Type::Fun { from: _, to } = handler_ty {
            if t_ty == *to {
                Ok(t_ty)
            } else {
                Err(to_check_err(ErrorKind::TypeMismatch {
                    found: t_ty.to_string(),
                    expected: to.to_string(),
                }))
            }
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: handler_ty.to_string(),
                expected: "Function Type".to_owned(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for Unit {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Error> {
        Ok(Type::Unit)
    }
}

impl<'a> Typecheck<'a> for Succ {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: Type::Nat.to_string(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Nat)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: Type::Nat.to_string(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for IsZero {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        if inner_ty == Type::Nat {
            Ok(Type::Bool)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner_ty.to_string(),
                expected: Type::Nat.to_string(),
            }))
        }
    }
}

impl<'a> Typecheck<'a> for If {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let if_ty = self.ift.check(&mut env.clone())?;
        if if_ty != Type::Bool {
            return Err(to_check_err(ErrorKind::TypeMismatch {
                found: if_ty.to_string(),
                expected: Type::Bool.to_string(),
            }));
        }
        let then_ty = self.thent.check(&mut env.clone())?;
        let else_ty = self.elset.check(env)?;
        if then_ty != else_ty {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: then_ty.to_string(),
                expected: else_ty.to_string(),
            }))
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
