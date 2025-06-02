use crate::values::Value;
use syntax::Location;

pub trait EvalEnvironment<V>
where
    V: Value,
    Self: Default,
{
    type EvalError: std::error::Error;

    fn fresh_location(&self) -> Location;

    fn save_location(&mut self, loc: Location, v: V);
    fn get_location(&self, loc: Location) -> Result<V, Self::EvalError>;
}

impl<V> EvalEnvironment<V> for ()
where
    V: Value,
{
    type EvalError = NotImplemented;

    fn fresh_location(&self) -> Location {
        0
    }

    fn save_location(&mut self, _: Location, _: V) {}
    fn get_location(&self, loc: Location) -> Result<V, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(loc))
    }
}
