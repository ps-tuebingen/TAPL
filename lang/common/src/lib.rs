pub trait Eval<'a> {
    type Value;
    type Error;
    type Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error>;
}

pub trait Typecheck<'a> {
    type Type;
    type Error;
    type Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error>;
}
