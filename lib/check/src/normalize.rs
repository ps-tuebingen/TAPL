use std::rc::Rc;
use syntax::{env::Environment, language::Language};

pub trait Normalize {
    type Lang: Language;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type;
}

impl<T> Normalize for Rc<T>
where
    T: Normalize + Clone,
{
    type Lang = T::Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        Rc::unwrap_or_clone(self).normalize(env)
    }
}
