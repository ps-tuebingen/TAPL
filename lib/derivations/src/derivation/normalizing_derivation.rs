use crate::{Derivation, conclusion::NormalizingConclusion, rules::NormalizingRule};
use std::fmt;
use syntax::language::Language;

#[derive(Debug)]
pub struct NormalizingDerivation<Lang>
where
    Lang: Language,
{
    pub conc: NormalizingConclusion<Lang>,
    pub label: NormalizingRule,
    pub premises: Vec<Derivation<Lang>>,
}

impl<Lang> NormalizingDerivation<Lang>
where
    Lang: Language,
{
    pub fn ret_ty(&self) -> Lang::Type {
        self.conc.right.clone()
    }

    pub fn empty<Ty>(ty: Ty) -> Self
    where
        Ty: Into<Lang::Type> + Clone,
    {
        Self {
            conc: NormalizingConclusion::new(ty.clone(), ty),
            label: NormalizingRule::Refl,
            premises: vec![],
        }
    }

    pub fn cong<Ty1, Ty2>(
        from: Ty1,
        to: Ty2,
        prem: Vec<Derivation<Lang>>,
    ) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            conc: NormalizingConclusion::new(from, to),
            label: NormalizingRule::Cong,
            premises: prem,
        }
    }

    pub fn opapp<Ty1, Ty2>(
        from: Ty1,
        to: Ty2,
        premises: Vec<Derivation<Lang>>,
    ) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            conc: NormalizingConclusion::new(from, to),
            label: NormalizingRule::OpApp,
            premises,
        }
    }
}

impl<Lang> fmt::Display for NormalizingDerivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prem in &self.premises {
            writeln!(f, "{prem}")?;
            writeln!(f,)?;
        }
        writeln!(f, "=== {} ===", self.label)?;
        write!(f, "{}", self.conc)
    }
}

impl<Lang> From<NormalizingDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: NormalizingDerivation<Lang>) -> Self {
        Self::NormalizingDerivation(deriv)
    }
}
