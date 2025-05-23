use crate::{
    check::CheckEnvironment,
    language::{LanguageType, LanguageValue},
    values::Value,
};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Location,
};

pub trait EvalEnvironment<V>
where
    V: Value,
    Self: Default,
{
    fn fresh_location(&self) -> Location;

    fn save_location(&mut self, loc: Location, v: V);
    fn get_location(&self, loc: Location) -> Result<V, ErrorKind>;
}

pub trait Eval: Sized {
    type Env: EvalEnvironment<Self::Value>;
    type Value: LanguageValue;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error>;
}

pub trait Normalize<Ty>
where
    Ty: LanguageType,
{
    type Env: CheckEnvironment<Type = Ty>;
    fn normalize(self, env: &mut Self::Env) -> Ty;
}

impl<V> EvalEnvironment<V> for ()
where
    V: Value,
{
    fn fresh_location(&self) -> Location {
        0
    }

    fn save_location(&mut self, _: Location, _: V) {}
    fn get_location(&self, loc: Location) -> Result<V, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(loc))
    }
}

pub fn to_eval_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Eval,
    }
}
