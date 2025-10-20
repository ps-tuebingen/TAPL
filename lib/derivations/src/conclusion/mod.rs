use std::fmt;
use syntax::language::Language;

mod normalizing;
mod subtyping;
mod typing;

pub use normalizing::NormalizingConclusion;
pub use subtyping::SubtypeConclusion;
pub use typing::TypingConclusion;

#[derive(Debug)]
pub enum Conclusion<Lang>
where
    Lang: Language,
{
    Typing(TypingConclusion<Lang>),
    Subtyping(SubtypeConclusion<Lang>),
}

impl<Lang> Conclusion<Lang> where Lang: Language {}

impl<Lang> fmt::Display for Conclusion<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Conclusion::Typing(conc) => conc.fmt(f),
            Conclusion::Subtyping(conc) => conc.fmt(f),
        }
    }
}
