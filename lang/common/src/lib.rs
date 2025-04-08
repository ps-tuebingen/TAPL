pub trait Eval {
    type Value;
    type Error;
    type Env: Default;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::Error>;
}

pub trait Typechck {
    type Type;
    type Error;
    fn check(&self) -> Result<Self::Type, Self::Error>;
}
