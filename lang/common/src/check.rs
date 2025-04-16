use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    language::LanguageType,
    types::Type,
    Var,
};

pub trait CheckEnvironment
where
    Self: Default + Clone,
{
    type Type: Type;

    fn get_var(&self, v: &Var) -> Result<Self::Type, Error>;
    fn add_var(&mut self, v: Var, ty: Self::Type);
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
