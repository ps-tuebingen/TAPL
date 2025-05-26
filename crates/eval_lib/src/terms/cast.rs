use crate::Eval;
use common::errors::Error;
use syntax::{
    terms::{Cast, Term},
    types::Type,
};

impl<T, Ty> Eval for Cast<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        self.term.eval(env)
    }
}
