use super::Type;

pub struct Sum<Ty>
where
    Ty: Type,
{
    left: Box<Ty>,
    right: Box<Ty>,
}

impl<Ty> Type for Sum<Ty> where Ty: Type {}
