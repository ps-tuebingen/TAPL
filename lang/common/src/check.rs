use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    language::LanguageType,
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

pub trait Typecheck {
    type Type: LanguageType;
    type Env: CheckEnvironment<Type = Self::Type>;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Self::Env::default())
    }

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error>;
}

pub fn to_check_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Check,
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
