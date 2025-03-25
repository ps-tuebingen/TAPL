use super::{check, is_subtype};
use crate::{
    lookup::{lookup_method_type, valid_override},
    syntax::{ClassName, ClassTable, MethodDeclaration},
};
use std::collections::HashMap;

pub fn method_is_ok(mdecl: &MethodDeclaration, in_class: &ClassName, ct: &ClassTable) -> bool {
    let mut env = HashMap::new();
    for (ty, arg) in mdecl.args.iter().cloned() {
        env.insert(arg, ty);
    }
    env.insert("this".to_owned(), in_class.clone());
    let ret_ty = if let Ok(ty) = check(&mdecl.ret, &mut env, ct) {
        ty
    } else {
        return false;
    };

    if !is_subtype(&ret_ty, &mdecl.class, ct) {
        return false;
    }

    let decl = if let Some(decl) = ct.classes.get(in_class) {
        decl
    } else {
        return false;
    };

    if lookup_method_type(&mdecl.name, &decl.parent, ct).is_ok() {
        valid_override(&mdecl.name, &decl.parent, &mdecl.get_type(), ct)
    } else {
        lookup_method_type(&mdecl.name, in_class, ct).is_ok()
    }
}

#[cfg(test)]
mod method_tests {
    use super::method_is_ok;
    use crate::test_common::{example_method, example_table};

    #[test]
    fn setfst_ok() {
        let result = method_is_ok(&example_method(), &"Pair".to_owned(), &example_table());
        assert!(result)
    }

    #[test]
    fn setfst_notok() {
        let result = method_is_ok(&example_method(), &"A".to_owned(), &example_table());
        assert!(!result)
    }
}
