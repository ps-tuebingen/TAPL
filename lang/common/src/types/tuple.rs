use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Tuple<Ty>
where
    Ty: Type,
{
    tys: Vec<Ty>,
}

impl<Ty> Type for Tuple<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Tuple<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tys: Vec<String> = self.tys.iter().map(|ty| ty.to_string()).collect();
        tys.sort();
        write!(f, "({})", tys.join(", "))
    }
}
