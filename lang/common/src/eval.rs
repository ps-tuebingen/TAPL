use crate::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
    subst::SubstTerm,
    terms::Term,
    types::Type,
    values::Value,
};

pub trait EvalEnvironment: Default {}

pub trait Eval<Val, Env, T, Ty>
where
    Val: Value<T>,
    Env: EvalEnvironment,
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
{
    fn eval_start(self) -> Result<Val, Error>;
    fn eval(self, env: &mut Env) -> Result<Val, Error>;
}

impl EvalEnvironment for () {}

pub fn to_eval_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Eval,
        lang: Lang::Unknown,
    }
}
