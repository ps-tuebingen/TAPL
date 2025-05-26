pub mod env;
pub mod terms;
pub mod values;

use common::errors::{Error, ErrorKind, ErrorLocation};
use env::EvalEnvironment;
use values::ValueGroup;

pub trait Eval: Sized {
    type Env: EvalEnvironment<Self::Value>;
    type Value: ValueGroup;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error>;
}

pub fn to_eval_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Eval,
    }
}
