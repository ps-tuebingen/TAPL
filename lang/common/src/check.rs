use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
    types::Type,
    Var,
};

pub trait CheckEnvironment<Ty>: Default + Clone
where
    Ty: Type,
{
    fn get_var(&self, v: &Var) -> Result<Ty, Error>;
    fn add_var(&mut self, v: Var, ty: Ty);
}

pub trait Typecheck<Env, Ty>
where
    Env: CheckEnvironment<Ty>,
    Ty: Type,
{
    fn check_start(&self) -> Result<Ty, Error>;
    fn check(&self, env: &mut Env) -> Result<Ty, Error>;
}

pub fn to_check_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Check,
        lang: Lang::Unknown,
    }
}
