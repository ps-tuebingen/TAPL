pub mod check;
pub mod errors;
pub mod eval;
pub mod objects;
pub mod parser;
pub mod syntax;
pub mod traits;

use check::check_subtype;
use errors::{Error, ErrorKind};
use std::collections::HashMap;
use syntax::{
    terms::Var,
    types::{Type, TypeVar},
};

pub type Label = String;

#[derive(Clone, Default)]
pub struct Env {
    vars: HashMap<Var, Type>,
    ty_vars: HashMap<TypeVar, Type>,
}

impl Env {
    pub fn add_var(&mut self, v: &Var, ty: &Type) {
        self.vars.insert(v.clone(), ty.clone());
    }

    pub fn get_var(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.vars
            .get(v)
            .ok_or(ErrorKind::FreeVar(v.clone()))
            .cloned()
    }

    pub fn add_tyvar(&mut self, v: &TypeVar, sup_ty: &Type) -> Result<(), Error> {
        let current = self.ty_vars.get(v);
        if let Some(ty) = current {
            match (
                check_subtype(ty, sup_ty, &mut self.clone()),
                check_subtype(sup_ty, ty, &mut self.clone()),
            ) {
                (Ok(_), _) => Ok(()),
                (_, Ok(_)) => {
                    self.ty_vars.insert(v.clone(), sup_ty.clone());
                    Ok(())
                }
                (Err(err), Err(_)) => Err(err),
            }
        } else {
            self.ty_vars.insert(v.clone(), sup_ty.clone());
            Ok(())
        }
    }

    pub fn get_tyvar(&self, v: &TypeVar) -> Result<Type, ErrorKind> {
        self.ty_vars
            .get(v)
            .ok_or(ErrorKind::FreeTypeVar(v.clone()))
            .cloned()
    }
}
