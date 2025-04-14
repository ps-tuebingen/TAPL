use super::Value;

pub struct Pair<V>
where
    V: Value,
{
    fst: Box<V>,
    snd: Box<V>,
}

impl<V> Value for Pair<V> where V: Value {}
