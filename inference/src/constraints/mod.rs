use crate::{
    syntax::Term,
    types::{Type, TypeVar},
    Var,
};
use std::collections::{HashMap, HashSet};

pub mod errors;

use errors::Error;

pub struct Constraint {
    pub left: Type,
    pub right: Type,
}

impl Constraint {
    pub fn subst(self, v: &TypeVar, ty: Type) -> Constraint {
        Constraint {
            left: self.left.subst(v, ty.clone()),
            right: self.right.subst(v, ty.clone()),
        }
    }

    pub fn subst_all(constrs: Vec<Constraint>, v: &TypeVar, ty: Type) -> Vec<Constraint> {
        constrs
            .into_iter()
            .map(|ctr| ctr.subst(v, ty.clone()))
            .collect()
    }
}

pub type TypingEnv = HashMap<Var, Type>;
pub type Unifier = HashMap<TypeVar, Type>;

pub fn fres_unification_var(used_vars: &HashSet<TypeVar>) -> TypeVar {
    let mut count = 0;
    while used_vars.contains(&("X".to_owned() + &count.to_string())) {
        count += 1;
    }
    "X".to_owned() + &count.to_string()
}

pub fn typecheck(term: Term, env: &mut TypingEnv) -> Result<Type, Error> {
    let (ty, constraints) = generate_constraints(term, env);
    let unified = unify_constraints(constraints)?;
    Ok(apply_unifier(unified, ty))
}

pub fn generate_constraints(term: Term, env: &mut TypingEnv) -> (Type, Vec<Constraint>) {
    todo!()
}

pub fn unify_constraints(constraints: Vec<Constraint>) -> Result<Unifier, Error> {
    todo!()
}

pub fn consains_var(ty: &Type, var: &TypeVar) -> bool {
    todo!()
}

pub fn apply_unifier(unifier: Unifier, ty: Type) -> Type {
    match ty {
        Type::Var(v) => unifier.get(&v).cloned().unwrap_or(Type::Var(v)),
        Type::Unit => Type::Unit,
        Type::Fun(ty1, ty2) => Type::Fun(
            Box::new(apply_unifier(unifier.clone(), *ty1)),
            Box::new(apply_unifier(unifier, *ty2)),
        ),
        Type::Bool => Type::Bool,
        Type::Nat => Type::Bool,
        Type::Prod(ty1, ty2) => Type::Prod(
            Box::new(apply_unifier(unifier.clone(), *ty1)),
            Box::new(apply_unifier(unifier, *ty2)),
        ),
        Type::Tup(tys) => Type::Tup(
            tys.into_iter()
                .map(|ty| apply_unifier(unifier.clone(), ty))
                .collect(),
        ),
        Type::Record(records) => Type::Record(
            records
                .into_iter()
                .map(|(key, ty)| (key, apply_unifier(unifier.clone(), ty)))
                .collect(),
        ),
        Type::Sum(ty1, ty2) => Type::Sum(
            Box::new(apply_unifier(unifier.clone(), *ty1)),
            Box::new(apply_unifier(unifier.clone(), *ty2)),
        ),
        Type::Variant(variants) => Type::Variant(
            variants
                .into_iter()
                .map(|(key, ty)| (key, apply_unifier(unifier.clone(), ty)))
                .collect(),
        ),
        Type::Optional(ty) => Type::Optional(Box::new(apply_unifier(unifier, *ty))),
        Type::List(ty) => Type::List(Box::new(apply_unifier(unifier, *ty))),
    }
}
