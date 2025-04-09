use super::is_subtype;
use crate::{
    errors::Error,
    lookup::{lookup_method_type, valid_override},
    syntax::{ClassName, ClassTable, MethodDeclaration},
};
use common::Typecheck;
use std::collections::HashMap;

impl<'a> Typecheck<'a> for MethodDeclaration {
    type Type = ();
    type Err = Error;
    type Env = (&'a ClassName, &'a ClassTable);

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check((&"Object".to_owned(), &Default::default()))
    }

    fn check(&self, (in_class, ct): Self::Env) -> Result<Self::Type, Self::Err> {
        let mut env = HashMap::new();
        for (ty, arg) in self.args.iter().cloned() {
            env.insert(arg, ty);
        }
        env.insert("this".to_owned(), in_class.clone());

        let ret_ty = self.ret.check((&mut env, ct))?;

        if !is_subtype(&ret_ty, &self.class, ct) {
            return Err(Error::NotASubClass {
                sub: ret_ty,
                sup: self.class.clone(),
            });
        }

        let decl = ct
            .classes
            .get(in_class)
            .ok_or(Error::ClassNotFound(in_class.clone()))?;

        if lookup_method_type(&self.name, &decl.parent, ct).is_ok() {
            valid_override(&self.name, &decl.parent, &self.get_type(), ct)
                .then_some(())
                .ok_or(Error::BadOverride {
                    class: decl.name.clone(),
                    sup: decl.parent.clone(),
                    method: self.name.clone(),
                })
        } else {
            lookup_method_type(&self.name, in_class, ct)
                .is_ok()
                .then_some(())
                .ok_or(Error::MethodNotFound {
                    method: self.name.clone(),
                    class: decl.name.clone(),
                })
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
