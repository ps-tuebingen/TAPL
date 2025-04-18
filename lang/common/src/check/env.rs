use super::to_check_err;
use crate::{
    errors::{Error, ErrorKind},
    language::{untyped::Untyped, LanguageType},
    types::Type,
    Location, Var,
};
use std::collections::HashMap;

pub trait CheckEnvironment
where
    Self: Default + Clone,
{
    type Type: Type;

    fn get_var(&self, v: &Var) -> Result<Self::Type, Error>;
    fn add_var(&mut self, v: Var, ty: Self::Type);

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, Error> {
        Err(to_check_err(ErrorKind::UndefinedLocation(*loc)))
    }
}

impl<Ty: LanguageType> CheckEnvironment for HashMap<Var, Ty> {
    type Type = Ty;

    fn get_var(&self, v: &Var) -> Result<Self::Type, Error> {
        self.get(v)
            .ok_or(to_check_err(ErrorKind::FreeVariable(v.clone())))
            .cloned()
    }

    fn add_var(&mut self, v: Var, ty: Ty) {
        self.insert(v, ty);
    }
}

impl CheckEnvironment for () {
    type Type = Untyped;
    fn get_var(&self, v: &Var) -> Result<Self::Type, Error> {
        Err(to_check_err(ErrorKind::FreeVariable(v.clone())))
    }

    fn add_var(&mut self, _: Var, _: Untyped) {}
}
