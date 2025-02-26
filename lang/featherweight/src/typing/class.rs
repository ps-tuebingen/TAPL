use super::methods::method_is_ok;
use crate::{
    lookup::lookup_fields,
    syntax::{ClassDeclaration, ClassTable},
};

pub fn class_ok(class: ClassDeclaration, ct: &ClassTable) -> bool {
    let ctor_name_correct = class.constructor.name == class.name;

    let parent_fields = if let Ok(fields) = lookup_fields(&class.parent, ct) {
        fields
    } else {
        return false;
    };
    let ctor_super_args = parent_fields == class.constructor.super_args;

    let mut all_ok = true;
    for method in class.methods {
        all_ok = all_ok && method_is_ok(&method, &class.name, ct);
    }

    ctor_name_correct && ctor_super_args && all_ok
}

#[cfg(test)]
mod class_tests {
    use super::class_ok;
    use crate::test_common::{example_a, example_pair, example_table};

    #[test]
    fn a_ok() {
        let result = class_ok(example_a(), &example_table());
        assert!(result)
    }

    #[test]
    fn pair_ok() {
        let result = class_ok(example_pair(), &example_table());
        assert!(result)
    }
}
