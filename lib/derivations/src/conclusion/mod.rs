use std::fmt;
use syntax::{terms::Term, types::Type};

mod subtyping;
mod typing;

pub use subtyping::SubtypeConclusion;
pub use typing::TypingConclusion;

#[derive(Debug)]
pub enum Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    Typing(TypingConclusion<T, Ty>),
    Subtyping(SubtypeConclusion<Ty>),
}

impl<T, Ty> Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Conclusion::Typing(conc) => conc.fmt(f),
            Conclusion::Subtyping(conc) => conc.fmt(f),
        }
    }
}
