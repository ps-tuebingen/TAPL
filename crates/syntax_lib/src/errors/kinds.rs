use crate::kinds::Kind;
use std::fmt;

#[derive(Debug)]
pub enum KindKind {
    Star,
    Arrow,
}

#[derive(Debug)]
pub enum TypeKind {
    Untyped,
    Variable,
    Function,
    Universal,
    Product,
    Tuple,
    Record,
    Variant,
    Sum,
    Option,
    List,
    Reference,
    Source,
    Sink,
    Existential,
    Mu,
    OpLambda,
    Nat,
    Bool,
    Unit,
    Top,
    Bot,
    OpApp,
}

impl From<Kind> for KindKind {
    fn from(knd: Kind) -> KindKind {
        match knd {
            Kind::Star => KindKind::Star,
            Kind::Arrow(_, _) => KindKind::Arrow,
        }
    }
}

impl fmt::Display for KindKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KindKind::Star => f.write_str("*"),
            KindKind::Arrow => f.write_str("Higher Kind"),
        }
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeKind::Untyped => f.write_str("untyped"),
            TypeKind::Variable => f.write_str("Type Variable"),
            TypeKind::Function => f.write_str("Function Type"),
            TypeKind::Universal => f.write_str("Universal Type"),
            TypeKind::Product => f.write_str("Product Type"),
            TypeKind::Tuple => f.write_str("Tuple Type"),
            TypeKind::Record => f.write_str("Record Type"),
            TypeKind::Variant => f.write_str("Variant Type"),
            TypeKind::Sum => f.write_str("Sum Type"),
            TypeKind::Option => f.write_str("Option Type"),
            TypeKind::List => f.write_str("List Type"),
            TypeKind::Reference => f.write_str("Reference Type"),
            TypeKind::Source => f.write_str("Source Type"),
            TypeKind::Sink => f.write_str("Sink Type"),
            TypeKind::Existential => f.write_str("Existential Type"),
            TypeKind::Mu => f.write_str("Mu Type"),
            TypeKind::OpLambda => f.write_str("Operator Abstraction"),
            TypeKind::Nat => f.write_str("Nat"),
            TypeKind::Bool => f.write_str("Bool"),
            TypeKind::Unit => f.write_str("Unit"),
            TypeKind::Top => f.write_str("Top"),
            TypeKind::Bot => f.write_str("Bot"),
            TypeKind::OpApp => f.write_str("Operator Application"),
        }
    }
}
