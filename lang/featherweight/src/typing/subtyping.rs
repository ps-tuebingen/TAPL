use crate::syntax::{ClassName, ClassTable};

pub fn is_subtype(cl1: &ClassName, cl2: &ClassName, ct: &ClassTable) -> bool {
    if cl2 == "Object" {
        return true;
    }

    if cl1 == cl2 {
        return true;
    }

    let decl1 = ct.get(cl1);
    if let Some(decl) = decl1 {
        is_subtype(&decl.parent, cl2, ct)
    } else {
        false
    }
}

#[cfg(test)]
mod subtyping_tests {
    use super::is_subtype;
    use crate::test_common::example_table;

    #[test]
    fn pair_leq_object() {
        let result = is_subtype(&"Pair".to_owned(), &"Object".to_owned(), &example_table());
        assert!(result)
    }

    #[test]
    fn a_notleq_b() {
        let result = is_subtype(&"A".to_owned(), &"B".to_owned(), &example_table());
        assert!(!result)
    }

    #[test]
    fn a_leq_a() {
        let result = is_subtype(&"A".to_owned(), &"A".to_owned(), &example_table());
        assert!(result)
    }
}
