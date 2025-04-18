use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    language::LanguageValue,
    Location,
};

pub trait EvalEnvironment: Default {
    fn fresh_location(&self) -> Location;
}

pub trait Eval: Sized {
    type Env: EvalEnvironment;
    type Value: LanguageValue;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error>;
}

impl EvalEnvironment for () {
    fn fresh_location(&self) -> Location {
        0
    }
}

pub fn to_eval_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Eval,
    }
}
