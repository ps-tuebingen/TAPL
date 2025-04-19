use crate::{
    errors::ErrorKind,
    kinds::Kind,
    language::{untyped::Untyped, LanguageType},
    types::Type,
    Location, TypeVar, Var,
};
use std::collections::HashMap;

pub trait CheckEnvironment
where
    Self: Default + Clone,
{
    type Type: Type;

    fn get_var(&self, v: &Var) -> Result<Self::Type, ErrorKind>;
    fn add_var(&mut self, v: Var, ty: Self::Type);

    fn get_tyvar(&self, v: &TypeVar) -> Result<Kind, ErrorKind>;
    fn add_tyvar(&mut self, v: TypeVar, knd: Kind);

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, ErrorKind>;
}

impl<Ty: LanguageType> CheckEnvironment for HashMap<Var, Ty> {
    type Type = Ty;

    fn get_var(&self, v: &Var) -> Result<Self::Type, ErrorKind> {
        self.get(v)
            .ok_or(ErrorKind::FreeVariable(v.clone()))
            .cloned()
    }

    fn add_var(&mut self, v: Var, ty: Ty) {
        self.insert(v, ty);
    }

    fn get_tyvar(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar(&mut self, _: TypeVar, _: Kind) {}

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(*loc))
    }
}

impl CheckEnvironment for () {
    type Type = Untyped;
    fn get_var(&self, v: &Var) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::FreeVariable(v.clone()))
    }
    fn add_var(&mut self, _: Var, _: Untyped) {}

    fn get_tyvar(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar(&mut self, _: TypeVar, _: Kind) {}

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(*loc))
    }
}
