use crate::{Conclusion, TypingRule};
use std::{fmt, marker::PhantomData};
use syntax::types::Type;

#[derive(Debug)]
pub struct SubtypeDerivation<Ty>
where
    Ty: Type,
{
    //TODOpub conc: Conclusion< Ty>,
    phantom: PhantomData<Ty>,
    pub label: TypingRule,
    pub premises: Vec<SubtypeDerivation<Ty>>,
}

impl<Ty> SubtypeDerivation<Ty> where Ty: Type {}

impl<Ty> fmt::Display for SubtypeDerivation<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prem in self.premises.iter() {
            writeln!(f, "{prem}")?;
            writeln!(f,)?;
        }
        writeln!(f, "=== {} ===", self.label)
        //TODO        write!(f, "{}", self.conc)
    }
}
