use super::Type;

pub struct Optional<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for Optional<Ty> where Ty: Type {}
