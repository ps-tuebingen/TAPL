use super::Type;

pub struct Product<Ty> where Ty:Type{
    fst:Box<Ty>,
    snd:Box<Ty>
}
