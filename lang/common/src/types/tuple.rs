use super::Type;

pub struct Tuple<Ty>
where
    Ty: Type,
{
    tys: Vec<Ty>,
}

impl<Ty> Type for Tuple<Ty> where Ty: Type {}
