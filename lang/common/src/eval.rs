use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    language::LanguageValue,
};

pub trait EvalEnvironment: Default {}

pub trait Eval: Sized {
    type Env: EvalEnvironment;
    type Value: LanguageValue;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error>;
}

impl EvalEnvironment for () {}

pub fn to_eval_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Eval,
    }
}
