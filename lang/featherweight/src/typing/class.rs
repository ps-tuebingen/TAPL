use crate::{
    errors::Error,
    lookup::lookup_fields,
    syntax::{ClassDeclaration, ClassTable},
};
use common::Typecheck;

impl<'a> Typecheck<'a> for ClassDeclaration {
    type Type = ();
    type Err = Error;
    type Env = &'a ClassTable;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&Default::default())
    }

    fn check(&self, ct: Self::Env) -> Result<Self::Type, Self::Err> {
        if self.constructor.name != self.name {
            return Err(Error::NameMismatch {
                found: self.constructor.name.clone(),
                expected: self.name.clone(),
                name: "Constructor Name".to_owned(),
            });
        }

        let parent_fields = lookup_fields(&self.parent, ct)?;
        for field in parent_fields.iter() {
            if !self.constructor.self_args.contains(field) {
                return Err(Error::FieldNotFound {
                    class: self.parent.clone(),
                    field: field.1.clone(),
                });
            }
        }

        for method in self.methods.iter() {
            method.check((&self.name, ct))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod class_tests {
    use crate::test_common::{example_a, example_pair, example_table};
    use common::Typecheck;

    #[test]
    fn a_ok() {
        let result = example_a().check(&example_table());
        assert!(result.is_ok())
    }

    #[test]
    fn pair_ok() {
        let result = example_pair().check(&example_table());
        assert!(result.is_ok())
    }
}
