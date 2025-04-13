use super::Type;

pub struct Fun<Ty>
where
    Ty: Type,
{
    from: Box<Ty>,
    to: Box<Ty>,
}

impl<Ty> Type for Fun<Ty> where Ty: Type {}
