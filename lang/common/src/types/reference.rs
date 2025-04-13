use super::Type;

pub struct Reference<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for Reference<Ty> where Ty: Type {}
