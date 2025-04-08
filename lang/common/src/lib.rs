pub trait Eval<'a> {
    type Value;
    type Error;
    type Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error>;
}

pub trait Typechck {
    type Type;
    type Error;
    fn check(&self) -> Result<Self::Type, Self::Error>;
}
