use crate::{syntax::Term, to_err, types::Type};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Var,
};
use std::collections::HashMap;
pub mod constraint;
pub mod gen_constraints;
pub mod unify;

use gen_constraints::{GenConstraints, GenState};
use unify::{unify, Unifier};

pub fn to_infer_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Inference)
}

pub type TypingEnv = HashMap<Var, Type>;

pub fn typecheck(term: Term) -> Result<Type, Error> {
    let mut state = GenState::default();
    let ty = term.gen_constraints(&mut state);
    let unified = unify(state.constrs)?;
    let num_vars = unified.keys().len();
    let mut subst = ty;
    for _ in 0..(num_vars + 1) {
        if subst.ty_vars().is_empty() {
            return Ok(subst);
        }
        subst = apply_unifier(&unified, subst);
    }
    Err(to_infer_err(ErrorKind::FreeTypeVariable(subst.to_string())))
}

pub fn apply_unifier(unifier: &Unifier, ty: Type) -> Type {
    match ty {
        Type::Var(v) => unifier.get(&v).cloned().unwrap_or(Type::Var(v)),
        Type::Unit => Type::Unit,
        Type::Fun(ty1, ty2) => Type::Fun(
            Box::new(apply_unifier(unifier, *ty1)),
            Box::new(apply_unifier(unifier, *ty2)),
        ),
        Type::Bool => Type::Bool,
        Type::Nat => Type::Nat,
        Type::Prod(ty1, ty2) => Type::Prod(
            Box::new(apply_unifier(unifier, *ty1)),
            Box::new(apply_unifier(unifier, *ty2)),
        ),
        Type::Sum(ty1, ty2) => Type::Sum(
            Box::new(apply_unifier(unifier, *ty1)),
            Box::new(apply_unifier(unifier, *ty2)),
        ),
        Type::Optional(ty) => Type::Optional(Box::new(apply_unifier(unifier, *ty))),
        Type::List(ty) => Type::List(Box::new(apply_unifier(unifier, *ty))),
    }
}

#[cfg(test)]
mod infer_tests {
    use super::typecheck;
    use crate::{
        syntax::{
            App, Cons, False, Head, If, Lambda, Left, Nil, Nothing, Pred, Something, Succ, SumCase,
            True, Zero,
        },
        types::Type,
    };

    #[test]
    fn infer_lam() {
        let result = typecheck(
            App {
                fun: Box::new(
                    Lambda {
                        var: "x".to_owned(),
                        body: Box::new(
                            Succ {
                                term: Box::new("x".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
                arg: Box::new(
                    Pred {
                        term: Box::new(Zero.into()),
                    }
                    .into(),
                ),
            }
            .into(),
        )
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn infer_list() {
        let result = typecheck(
            Head {
                list: Box::new(
                    Cons {
                        fst: Box::new(
                            If {
                                ifc: Box::new(False.into()),
                                thenc: Box::new(Zero.into()),
                                elsec: Box::new(
                                    Succ {
                                        term: Box::new(Zero.into()),
                                    }
                                    .into(),
                                ),
                            }
                            .into(),
                        ),
                        rst: Box::new(
                            Cons {
                                fst: Box::new(Zero.into()),
                                rst: Box::new(Nil.into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        )
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }

    #[test]
    fn infer_sumcase() {
        let result = typecheck(
            SumCase {
                bound_term: Box::new(
                    Left {
                        left_term: Box::new(True.into()),
                    }
                    .into(),
                ),
                left_var: "x".to_owned(),
                left_term: Box::new(
                    If {
                        ifc: Box::new("x".to_owned().into()),
                        thenc: Box::new(Nothing.into()),
                        elsec: Box::new(
                            Something {
                                term: Box::new(Zero.into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
                right_var: "y".to_owned(),
                right_term: Box::new(
                    Something {
                        term: Box::new(
                            Succ {
                                term: Box::new("y".to_owned().into()),
                            }
                            .into(),
                        ),
                    }
                    .into(),
                ),
            }
            .into(),
        )
        .unwrap();
        let expected = Type::Optional(Box::new(Type::Nat));
        assert_eq!(result, expected)
    }
}
