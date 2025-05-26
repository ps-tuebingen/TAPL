use crate::values::Value;
use common::errors::{Error, ErrorKind};
use syntax::Location;

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
    type Value: Value;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error>;
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
