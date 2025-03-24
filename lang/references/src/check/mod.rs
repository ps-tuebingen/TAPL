use super::{
    terms::{Loc, Term, Var},
    types::Type,
};
use std::collections::HashMap;

pub mod errors;
use errors::Error;

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Loc, Type>;

pub fn check(t: Term, env: &mut Env, st: &mut StoreTy) -> Result<Type, Error> {
    match t {
        Term::Var(v) => env.get(&v).ok_or(Error::FreeVar(v)).cloned(),
        Term::Const(_) => Ok(Type::Nat),
        Term::Succ(t) => {
            let ty = check(*t, env, st)?;
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
            let ty = check(*t, env, st)?;
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
            env.insert(var, annot.clone());
            let body_ty = check(*body, env, st)?;
            Ok(Type::Fun {
                from: Box::new(annot),
                to: Box::new(body_ty),
            })
        }
        Term::App { fun, arg } => {
            let fun_ty = check(*fun, &mut env.clone(), st)?;
            if let Type::Fun { from, to } = fun_ty {
                let arg_ty = check(*arg, env, st)?;
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
            let inner = check(*t, env, st)?;
            Ok(Type::Ref(Box::new(inner)))
        }
        Term::Deref(t) => {
            let inner = check(*t, env, st)?;
            if let Type::Ref(ty) = inner {
                Ok(*ty)
            } else {
                Err(Error::NotAReference(inner))
            }
        }
        Term::Assign { to, body } => {
            let ty1 = check(*to, &mut env.clone(), st)?;
            let ref_inner = if let Type::Ref(ty) = ty1 {
                ty
            } else {
                return Err(Error::NotAReference(ty1));
            };
            let ty2 = check(*body, env, st)?;
            if ty2 == *ref_inner {
                Ok(Type::Unit)
            } else {
                Err(Error::TypeMismatch {
                    found: ty2,
                    expected: *ref_inner,
                })
            }
        }
        Term::Loc(loc) => st
            .get(&loc)
            .ok_or(Error::UnknownLocation(loc))
            .map(|ty| Type::Ref(Box::new(ty.clone()))),
        Term::Let {
            var,
            bound_term,
            in_term,
        } => {
            let bound_ty = check(*bound_term, &mut env.clone(), st)?;
            env.insert(var, bound_ty);
            check(*in_term, env, st)
        }
        Term::If {
            left,
            right,
            then_term,
            else_term,
            ..
        } => {
            let left_ty = check(*left, &mut env.clone(), st)?;
            if left_ty != Type::Nat {
                return Err(Error::TypeMismatch {
                    found: left_ty,
                    expected: Type::Nat,
                });
            }
            let right_ty = check(*right, &mut env.clone(), st)?;
            if right_ty != Type::Nat {
                return Err(Error::TypeMismatch {
                    found: right_ty,
                    expected: Type::Nat,
                });
            }
            let then_ty = check(*then_term, &mut env.clone(), st)?;
            let else_ty = check(*else_term, env, st)?;
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

#[cfg(test)]
mod check_tests {
    use super::{check, Term, Type};
    use std::collections::HashMap;

    #[test]
    fn check1() {
        let result = check(
            Term::app(
                Term::lam(
                    "x",
                    Type::Ref(Box::new(Type::Unit)),
                    Term::deref("x".into()),
                ),
                Term::app(
                    Term::lam("y", Type::Unit, Term::reft("y".into())),
                    Term::Unit,
                ),
            ),
            &mut Default::default(),
            &mut Default::default(),
        )
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check2() {
        let result = check(
            Term::App {
                fun: Box::new(Term::Lambda {
                    var: "x".to_owned(),
                    annot: Type::Ref(Box::new(Type::Unit)),
                    body: Box::new(Term::Assign {
                        to: Box::new(Term::Var("x".to_owned())),
                        body: Box::new(Term::Deref(Box::new(Term::Var("x".to_owned())))),
                    }),
                }),
                arg: Box::new(Term::Ref(Box::new(Term::Unit))),
            },
            &mut Default::default(),
            &mut Default::default(),
        )
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fail() {
        let result = check(
            Term::seq(
                Term::assign(
                    Term::reft(Term::Unit),
                    Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
                ),
                Term::deref(0.into()),
            ),
            &mut Default::default(),
            &mut Default::default(),
        );
        assert!(result.is_err())
    }

    #[test]
    fn check_store() {
        let result = check(
            Term::seq(
                Term::assign(
                    Term::reft(Term::Unit),
                    Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
                ),
                Term::deref(0.into()),
            ),
            &mut Default::default(),
            &mut HashMap::from([(0, Type::Unit)]),
        )
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}
