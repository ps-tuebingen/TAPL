use crate::{Derivation, conclusion::KindingConclusion, rules::KindingRule};
use std::fmt;
use syntax::language::Language;

#[derive(Debug)]
pub struct KindingDerivation<Lang>
where
    Lang: Language,
{
    pub conc: KindingConclusion<Lang>,
    pub label: KindingRule,
    pub premises: Vec<Derivation<Lang>>,
}

impl<Lang> KindingDerivation<Lang>
where
    Lang: Language,
{
    pub fn ret_ty(&self) -> Lang::Type {
        self.conc.ty.clone()
    }
}

impl<Lang> fmt::Display for KindingDerivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prem in self.premises.iter() {
            writeln!(f, "{prem}")?;
            writeln!(f,)?;
        }
        writeln!(f, "=== {} ===", self.label)?;
        write!(f, "{}", self.conc)
    }
}

impl<Lang> From<KindingDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: KindingDerivation<Lang>) -> Derivation<Lang> {
        Derivation::KindingDerivation(deriv)
    }
}
