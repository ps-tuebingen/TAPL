use super::Value;

pub struct Something<V>
where
    V: Value,
{
    val: Box<V>,
}

impl<V> Value for Something<V> where V: Value {}
