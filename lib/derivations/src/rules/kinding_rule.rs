use std::fmt;

#[derive(Debug)]
pub enum KindingRule {
    Prim,
    Annot,
    Exists,
    ExistsBounded,
    Forall,
    ForallBounded,
    Fun,
    OpApp,
    OpLam,
    OpLamSub,
    Record,
    Sum,
    Var,
}

impl fmt::Display for KindingRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Prim => f.write_str("K-Prim"),
            Self::Annot => f.write_str("K-Annot"),
            Self::Exists => f.write_str("K-Exists"),
            Self::ExistsBounded => f.write_str("K-Exists<:"),
            Self::Forall => f.write_str("K-Forall"),
            Self::ForallBounded => f.write_str("K-Forall<:"),
            Self::Fun => f.write_str("K-Fun"),
            Self::OpApp => f.write_str("K-OpApp"),
            Self::OpLam => f.write_str("K-OpLam"),
            Self::OpLamSub => f.write_str("K-OpLamSub"),
            Self::Record => f.write_str("K-Record"),
            Self::Sum => f.write_str("K-Sum"),
            Self::Var => f.write_str("K-Var"),
        }
    }
}
