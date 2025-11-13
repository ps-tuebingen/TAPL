use crate::{Derivation, conclusion::KindingConclusion, rules::KindingRule};
use std::fmt;
use syntax::{kinds::Kind, language::Language, types::TypeVariable};

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

    pub fn ret_kind(&self) -> Kind {
        self.conc.kind.clone()
    }

    pub fn prim<Ty>(ty: Ty) -> Self
    where
        Ty: Into<Lang::Type>,
    {
        Self {
            conc: KindingConclusion::new(ty, Kind::Star),
            label: KindingRule::Prim,
            premises: vec![],
        }
    }

    pub fn annotated<Ty>(ty: Ty, knd: Kind) -> Self
    where
        Ty: Into<Lang::Type>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::Annot,
            premises: vec![],
        }
    }

    pub fn exists<Ty, Deriv>(ty: Ty, knd: Kind, prem: Deriv) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::Exists,
            premises: vec![prem.into()],
        }
    }

    pub fn exists_bound<Ty, Deriv1, Deriv2>(
        ty: Ty,
        knd: Kind,
        sup_prem: Deriv1,
        inner_prem: Deriv2,
    ) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv1: Into<Derivation<Lang>>,
        Deriv2: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::ExistsBounded,
            premises: vec![sup_prem.into(), inner_prem.into()],
        }
    }

    pub fn forall<Ty, Deriv>(ty: Ty, knd: Kind, prem: Deriv) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::Forall,
            premises: vec![prem.into()],
        }
    }

    pub fn forall_bound<Ty, Deriv1, Deriv2>(
        ty: Ty,
        knd: Kind,
        sup_prem: Deriv1,
        inner_prem: Deriv2,
    ) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv1: Into<Derivation<Lang>>,
        Deriv2: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::ForallBounded,
            premises: vec![sup_prem.into(), inner_prem.into()],
        }
    }

    pub fn fun<Ty, Deriv1, Deriv2>(
        ty: Ty,
        from_deriv: Deriv1,
        to_deriv: Deriv2,
    ) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv1: Into<Derivation<Lang>>,
        Deriv2: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, Kind::Star),
            label: KindingRule::Fun,
            premises: vec![from_deriv.into(), to_deriv.into()],
        }
    }

    pub fn op_app<Ty, Deriv1, Deriv2>(
        ty: Ty,
        knd: Kind,
        fun_deriv: Deriv1,
        arg_deriv: Deriv2,
    ) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv1: Into<Derivation<Lang>>,
        Deriv2: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::OpApp,
            premises: vec![fun_deriv.into(), arg_deriv.into()],
        }
    }

    pub fn op_lam<Ty, Deriv>(ty: Ty, knd: Kind, body_deriv: Deriv) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::OpLam,
            premises: vec![body_deriv.into()],
        }
    }

    pub fn op_lam_sub<Ty, Deriv1, Deriv2>(
        ty: Ty,
        knd: Kind,
        sup_deriv: Deriv1,
        body_deriv: Deriv2,
    ) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv1: Into<Derivation<Lang>>,
        Deriv2: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::OpLamSub,
            premises: vec![sup_deriv.into(), body_deriv.into()],
        }
    }

    pub fn record<Ty>(ty: Ty, premises: Vec<Derivation<Lang>>) -> Self
    where
        Ty: Into<Lang::Type>,
    {
        Self {
            conc: KindingConclusion::new(ty, Kind::Star),
            label: KindingRule::Record,
            premises,
        }
    }

    pub fn sum<Ty, Deriv1, Deriv2>(
        ty: Ty,
        knd: Kind,
        left_deriv: Deriv1,
        right_deriv: Deriv2,
    ) -> Self
    where
        Ty: Into<Lang::Type>,
        Deriv1: Into<Derivation<Lang>>,
        Deriv2: Into<Derivation<Lang>>,
    {
        Self {
            conc: KindingConclusion::new(ty, knd),
            label: KindingRule::Sum,
            premises: vec![left_deriv.into(), right_deriv.into()],
        }
    }

    #[must_use] pub fn var(var: &str, knd: Kind) -> Self
    where
        TypeVariable<Lang>: Into<Lang::Type>,
    {
        Self {
            conc: KindingConclusion::new(TypeVariable::new(var), knd),
            label: KindingRule::Var,
            premises: vec![],
        }
    }
}

impl<Lang> fmt::Display for KindingDerivation<Lang>
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

impl<Lang> From<KindingDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: KindingDerivation<Lang>) -> Self {
        Self::KindingDerivation(deriv)
    }
}
