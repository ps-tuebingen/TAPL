pub trait Eval<'a> {
    type Value;
    type Err;
    type Env;
    fn eval_start(self) -> Result<Self::Value, Self::Err>;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err>;
}

pub trait Typecheck<'a> {
    type Type;
    type Err;
    type Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err>;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err>;
}

pub trait Parse: Sized {
    type Err;
    fn parse(sourcte: String) -> Result<Self, Self::Err>;
}
