pub trait Eval {
    type Value;
    type Error;
    fn eval(self) -> Result<Self::Value, Self::Error>;
}

pub trait Typechck {
    type Type;
    type Error;
    fn check(&self) -> Result<Self::Type, Self::Error>;
}
