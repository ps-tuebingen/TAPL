use super::Type;

pub struct Sink<Ty> where Ty:Type{
    ty:Box<Ty>
}

impl<Ty> Type for Sink<Ty> where Ty:Type{}
