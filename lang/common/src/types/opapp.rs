use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct OpApp<Ty>
where
    Ty: Type,
{
    fun: Box<Ty>,
    arg: Box<Ty>,
}

impl<Ty> Type for OpApp<Ty> where Ty: Type {}

impl<Ty> fmt::Display for OpApp<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
