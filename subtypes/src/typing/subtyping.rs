use crate::types::Type;

pub fn is_subtype(lower: &Type, upper: &Type) -> bool {
    match (lower, upper) {
        //SA-Bot
        (Type::Bot, _) => true,
        //SA-Top
        (_, Type::Top) => true,
        //SA-Arrow
        (
            Type::Fun {
                from: from1,
                to: to1,
            },
            Type::Fun {
                from: from2,
                to: to2,
            },
        ) => is_subtype(from2, from1) && is_subtype(to1, to2),
        //SA-Rcs
        (Type::Record(rec1), Type::Record(rec2)) => {
            let mut subtype = true;
            for (label, ty2) in rec2.iter() {
                let ty1 = if let Some(ty) = rec1.get(label) {
                    ty
                } else {
                    return false;
                };
                subtype = subtype && is_subtype(ty1, ty2);
            }
            subtype
        }
        //SA-Variant
        (Type::Variant(vars1), Type::Variant(vars2)) => {
            let mut subtype = true;
            for (label1, ty1) in vars1.iter() {
                let ty2 = match vars2.iter().find(|(label2, _)| label1 == label2) {
                    Some((_, ty)) => ty,
                    None => return false,
                };
                subtype = subtype && is_subtype(ty1, ty2);
            }
            subtype
        }
        //SA-List
        (Type::List(ty1), Type::List(ty2)) => is_subtype(ty1, ty2),
        //SA-Ref
        (Type::Ref(ty1), Type::Ref(ty2)) => is_subtype(ty1, ty2) && is_subtype(ty2, ty1),
        //SA-Source
        (Type::Source(ty1), Type::Source(ty2)) => is_subtype(ty1, ty2),
        //SA-Sink
        (Type::Sink(ty1), Type::Sink(ty2)) => is_subtype(ty2, ty1),
        //SA-RefSource
        (Type::Ref(ty1), Type::Source(ty2)) => ty1 == ty2,
        //SA-RefSink
        (Type::Sink(ty1), Type::Ref(ty2)) => ty1 == ty2,
        //SA-Refl
        (ty1, ty2) => ty1 == ty2,
    }
}

#[cfg(test)]
mod subtyping_tests {
    use super::{is_subtype, Type};
    use std::collections::HashMap;

    #[test]
    fn rec_xyz() {
        let lower = Type::Record(HashMap::from([
            ("x".to_owned(), Type::Nat),
            ("y".to_owned(), Type::Nat),
            ("z".to_owned(), Type::Nat),
        ]));
        let upper = Type::Record(HashMap::from([("y".to_owned(), Type::Nat)]));
        assert!(is_subtype(&lower, &upper))
    }
}
