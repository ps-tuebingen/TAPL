use super::Type;

pub struct OpApp<Ty>
where
    Ty: Type,
{
    fun: Box<Ty>,
    arg: Box<Ty>,
}

impl<Ty> Type for OpApp<Ty> where Ty: Type {}
