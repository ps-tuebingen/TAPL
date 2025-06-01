use crate::{CheckEnvironment, Typecheck};
use syntax::terms::{Term, Variable};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        env.get_var(&self.var)
    }
}
