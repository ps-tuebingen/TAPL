use super::Value;

pub struct Tuple<V>
where
    V: Value,
{
    vals: Vec<V>,
}

impl<V> Value for Tuple<V> where V: Value {}
