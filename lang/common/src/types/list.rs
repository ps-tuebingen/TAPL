use super::Type;

pub struct List<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for List<Ty> where Ty: Type {}
