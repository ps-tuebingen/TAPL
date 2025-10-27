use std::fmt;
use syntax::{env::Environment, language::Language};

#[derive(Debug)]
pub struct TypingConclusion<Lang>
where
    Lang: Language,
{
    pub env: Environment<Lang>,
    pub term: Lang::Term,
    pub ty: Lang::Type,
}

impl<Lang> TypingConclusion<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(env: Environment<Lang>, term: T1, ty: Ty1) -> TypingConclusion<Lang>
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        TypingConclusion {
            env,
            term: term.into(),
            ty: ty.into(),
        }
    }

    pub fn ty(&self) -> Lang::Type {
        self.ty.clone()
    }
}

impl<Lang> fmt::Display for TypingConclusion<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} |-> {} : {}", self.env, self.term, self.ty)
    }
}
