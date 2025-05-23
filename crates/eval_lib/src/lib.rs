pub mod env;
pub mod terms;
pub mod types;
pub mod values;

use common::errors::Error;
use env::EvalEnvironment;
use syntax::types::Type;
use values::Value;

pub trait Eval: Sized {
    type Env: EvalEnvironment<Self::Value>;
    type Value: Value;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error>;
}

pub trait Normalize<Ty>
where
    Ty: Type,
{
    type Env: CheckEnvironment<Type = Ty>;
    fn normalize(self, env: &mut Self::Env) -> Ty;
}
