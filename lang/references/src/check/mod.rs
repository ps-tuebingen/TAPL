use super::{
    terms::{Loc, Term, Var},
    types::Type,
};
use common::Typecheck;
use std::collections::HashMap;

pub mod errors;
use errors::Error;

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Loc, Type>;

#[derive(Default, Clone)]
pub struct Environment {
    pub env: Env,
    pub store_ty: StoreTy,
}

impl Environment {
    pub fn get_var(&self, v: &Var) -> Result<Type, Error> {
        self.env.get(v).ok_or(Error::FreeVar(v.clone())).cloned()
    }

    pub fn get_loc(&self, loc: &Loc) -> Result<Type, Error> {
        self.store_ty
            .get(loc)
            .ok_or(Error::UnknownLocation(*loc))
            .cloned()
    }

    pub fn add_var(&mut self, v: &Var, ty: &Type) {
        self.env.insert(v.clone(), ty.clone());
    }
}

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Environment;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(v) => env.get_var(v),
            Term::Const(_) => Ok(Type::Nat),
            Term::Succ(t) => {
                let ty = t.check(env)?;
                if ty != Type::Nat {
                    Err(Error::TypeMismatch {
                        found: ty,
                        expected: Type::Nat,
                    })
                } else {
                    Ok(Type::Nat)
                }
            }
            Term::Pred(t) => {
                let ty = t.check(env)?;
                if ty != Type::Nat {
                    Err(Error::TypeMismatch {
                        found: ty,
                        expected: Type::Nat,
                    })
                } else {
                    Ok(Type::Nat)
                }
            }
            Term::Lambda { var, annot, body } => {
                env.add_var(var, annot);
                let body_ty = body.check(env)?;
                Ok(Type::Fun {
                    from: Box::new(annot.clone()),
                    to: Box::new(body_ty),
                })
            }
            Term::App { fun, arg } => {
                let fun_ty = fun.check(&mut env.clone())?;
                if let Type::Fun { from, to } = fun_ty {
                    let arg_ty = arg.check(env)?;
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
            Term::Unit => Ok(Type::Unit),
            Term::Ref(t) => {
                let inner = t.check(env)?;
                Ok(Type::Ref(Box::new(inner)))
            }
            Term::Deref(t) => {
                let inner = t.check(env)?;
                if let Type::Ref(ty) = inner {
                    Ok(*ty)
                } else {
                    Err(Error::NotAReference(inner))
                }
            }
            Term::Assign { to, body } => {
                let ty1 = to.check(&mut env.clone())?;
                let ref_inner = if let Type::Ref(ty) = ty1 {
                    ty
                } else {
                    return Err(Error::NotAReference(ty1));
                };
                let ty2 = body.check(env)?;
                if ty2 == *ref_inner {
                    Ok(Type::Unit)
                } else {
                    Err(Error::TypeMismatch {
                        found: ty2,
                        expected: *ref_inner,
                    })
                }
            }
            Term::Loc(loc) => env.get_loc(loc).map(|ty| Type::Ref(Box::new(ty.clone()))),
            Term::Let {
                var,
                bound_term,
                in_term,
            } => {
                let bound_ty = bound_term.check(&mut env.clone())?;
                env.add_var(var, &bound_ty);
                in_term.check(env)
            }
            Term::If {
                left,
                right,
                then_term,
                else_term,
                ..
            } => {
                let left_ty = left.check(&mut env.clone())?;
                if left_ty != Type::Nat {
                    return Err(Error::TypeMismatch {
                        found: left_ty,
                        expected: Type::Nat,
                    });
                }
                let right_ty = right.check(&mut env.clone())?;
                if right_ty != Type::Nat {
                    return Err(Error::TypeMismatch {
                        found: right_ty,
                        expected: Type::Nat,
                    });
                }
                let then_ty = then_term.check(&mut env.clone())?;
                let else_ty = else_term.check(env)?;
                if then_ty != else_ty {
                    return Err(Error::TypeMismatch {
                        found: then_ty,
                        expected: else_ty,
                    });
                }
                Ok(then_ty)
            }
        }
    }
}
#[cfg(test)]
mod check_tests {
    use super::{Environment, Term, Type};
    use common::Typecheck;
    use std::collections::HashMap;

    #[test]
    fn check1() {
        let result = Term::app(
            Term::lam(
                "x",
                Type::Ref(Box::new(Type::Unit)),
                Term::deref("x".into()),
            ),
            Term::app(
                Term::lam("y", Type::Unit, Term::reft("y".into())),
                Term::Unit,
            ),
        )
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check2() {
        let result = Term::App {
            fun: Box::new(Term::Lambda {
                var: "x".to_owned(),
                annot: Type::Ref(Box::new(Type::Unit)),
                body: Box::new(Term::Assign {
                    to: Box::new(Term::Var("x".to_owned())),
                    body: Box::new(Term::Deref(Box::new(Term::Var("x".to_owned())))),
                }),
            }),
            arg: Box::new(Term::Ref(Box::new(Term::Unit))),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fail() {
        let result = Term::seq(
            Term::assign(
                Term::reft(Term::Unit),
                Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
            ),
            Term::deref(0.into()),
        )
        .check(&mut Default::default());
        assert!(result.is_err())
    }

    #[test]
    fn check_store() {
        let result = Term::seq(
            Term::assign(
                Term::reft(Term::Unit),
                Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
            ),
            Term::deref(0.into()),
        )
        .check(&mut Environment {
            env: Default::default(),
            store_ty: HashMap::from([(0, Type::Unit)]),
        })
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}
