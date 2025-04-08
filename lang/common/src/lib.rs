pub trait Eval<'a> {
    type Value;
    type Err;
    type Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err>;
}

pub trait Typecheck<'a> {
    type Type;
    type Err;
    type Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err>;
}
