use super::Type;

pub struct Source<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for Source<Ty> where Ty: Type {}
