use super::{is_subtype, to_check_err};
use crate::{
    lookup::{lookup_method_type, valid_override},
    syntax::{ClassName, ClassTable, MethodDeclaration},
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};
use std::collections::HashMap;

impl<'a> Typecheck<'a> for MethodDeclaration {
    type Type = ();
    type Env = (&'a ClassName, &'a ClassTable);

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check((&"Object".to_owned(), &Default::default()))
    }

    fn check(&self, (in_class, ct): Self::Env) -> Result<Self::Type, Error> {
        let mut env = HashMap::new();
        for (ty, arg) in self.args.iter().cloned() {
            env.insert(arg, ty);
        }
        env.insert("this".to_owned(), in_class.clone());

        let ret_ty = self.ret.check((&mut env, ct))?;

        if !is_subtype(&ret_ty, &self.class, ct) {
            return Err(to_check_err(ErrorKind::Subtype {
                sub: ret_ty,
                sup: self.class.clone(),
            }));
        }

        let decl = ct
            .classes
            .get(in_class)
            .ok_or(ErrorKind::UndefinedName(in_class.clone()))
            .map_err(to_check_err)?;

        if lookup_method_type(&self.name, &decl.parent, ct).is_ok() {
            valid_override(&self.name, &decl.parent, &self.get_type(), ct).map_err(to_check_err)
        } else {
            lookup_method_type(&self.name, in_class, ct)
                .is_ok()
                .then_some(())
                .ok_or(to_check_err(ErrorKind::UndefinedName(self.name.clone())))
        }
    }
}

#[cfg(test)]
mod method_tests {
    use crate::test_common::{example_method, example_table};
    use common::Typecheck;

    #[test]
    fn setfst_ok() {
        let result = &example_method().check((&"Pair".to_owned(), &example_table()));
        assert!(result.is_ok())
    }

    #[test]
    fn setfst_notok() {
        let result = &example_method().check((&"A".to_owned(), &example_table()));
        assert!(result.is_err())
    }
}
