use super::syntax::Label;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Top,
    Bot,
    Fun { from: Box<Type>, to: Box<Type> },
    Record(HashMap<Label, Type>),
    Variant(Vec<(Label, Type)>),
    List(Box<Type>),
    Ref(Box<Type>),
    Source(Box<Type>),
    Sink(Box<Type>),
    Nat,
    Unit,
    Bool,
}

impl Type {
    pub fn fun(from: Type, to: Type) -> Type {
        Type::Fun {
            from: Box::new(from),
            to: Box::new(to),
        }
    }

    pub fn rec(recs: Vec<(&str, Type)>) -> Type {
        let recs: Vec<(String, Type)> =
            recs.into_iter().map(|(s, ty)| (s.to_owned(), ty)).collect();
        Type::Record(HashMap::from_iter(recs))
    }

    pub fn ref_ty(ty: Type) -> Type {
        Type::Ref(Box::new(ty))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Top => f.write_str("⊤"),
            Type::Bot => f.write_str("⊥"),
            Type::Fun { from, to } => write!(f, "({from})→({to})"),
            Type::Record(recs) => {
                let rec_strs: Vec<String> = recs
                    .iter()
                    .map(|(label, ty)| format!("{label}:{ty}"))
                    .collect();
                write!(f, "{{ {} }}", rec_strs.join(", "))
            }
            Type::Variant(variants) => {
                let var_strs: Vec<String> = variants
                    .iter()
                    .map(|(label, ty)| format!("{label}:{ty}"))
                    .collect();
                write!(f, "<{}>", var_strs.join(","))
            }
            Type::List(ty) => write!(f, "List({ty})"),
            Type::Ref(ty) => write!(f, "Ref({ty})"),
            Type::Source(ty) => write!(f, "Source({ty})"),
            Type::Sink(ty) => write!(f, "Sink({ty})"),
            Type::Nat => f.write_str("Nat"),
            Type::Unit => f.write_str("Unit"),
            Type::Bool => f.write_str("Bool"),
        }
    }
}
