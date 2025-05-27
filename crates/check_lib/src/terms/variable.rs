use crate::{env::CheckEnvironment, to_check_err, Typecheck};
use common::errors::Error;
use syntax::terms::{Term, Variable};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        env.get_var(&self.var).map_err(to_check_err)
    }
}
