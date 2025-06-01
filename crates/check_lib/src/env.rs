use crate::errors::{EnvError, FreeVariable, NotImplemented};
use std::collections::HashMap;
use syntax::{kinds::Kind, types::Type, untyped::Untyped, Location, TypeVar, Var};

pub trait CheckEnvironment
where
    Self: Default + Clone,
{
    type Type: Type;
    type CheckError: std::error::Error;

    fn get_var(&self, v: &Var) -> Result<Self::Type, Self::CheckError>;
    fn add_var(&mut self, v: Var, ty: Self::Type);

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, Self::CheckError>;
    fn add_tyvar_kind(&mut self, v: TypeVar, knd: Kind);

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, Self::CheckError>;
    fn add_tyvar_super(&mut self, v: TypeVar, ty: Self::Type);

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, Self::CheckError>;
}

impl<Ty: Type> CheckEnvironment for HashMap<Var, Ty> {
    type Type = Ty;
    type CheckError = EnvError;

    fn get_var(&self, v: &Var) -> Result<Self::Type, Self::CheckError> {
        self.get(v).ok_or(FreeVariable::new(v).into()).cloned()
    }

    fn add_var(&mut self, v: Var, ty: Ty) {
        self.insert(v, ty);
    }

    fn get_tyvar_kind(&self, _: &TypeVar) -> Result<Kind, Self::CheckError> {
        Err(NotImplemented.into())
    }

    fn get_tyvar_super(&self, _: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }

    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}
    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}

    fn get_loc(&self, _: &Location) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
}

impl CheckEnvironment for () {
    type Type = Untyped;
    type CheckError = NotImplemented;

    fn get_var(&self, _: &Var) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_var(&mut self, _: Var, _: Untyped) {}

    fn get_tyvar_kind(&self, _: &TypeVar) -> Result<Kind, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}

    fn get_tyvar_super(&self, _: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}

    fn get_loc(&self, _: &Location) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
}
