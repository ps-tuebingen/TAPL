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
            KindingRule::Prim => f.write_str("K-Prim"),
            KindingRule::Annot => f.write_str("K-Annot"),
            KindingRule::Exists => f.write_str("K-Exists"),
            KindingRule::ExistsBounded => f.write_str("K-Exists<:"),
            KindingRule::Forall => f.write_str("K-Forall"),
            KindingRule::ForallBounded => f.write_str("K-Forall<:"),
            KindingRule::Fun => f.write_str("K-Fun"),
            KindingRule::OpApp => f.write_str("K-OpApp"),
            KindingRule::OpLam => f.write_str("K-OpLam"),
            KindingRule::OpLamSub => f.write_str("K-OpLamSub"),
            KindingRule::Record => f.write_str("K-Record"),
            KindingRule::Sum => f.write_str("K-Sum"),
            KindingRule::Var => f.write_str("K-Var"),
        }
    }
}
