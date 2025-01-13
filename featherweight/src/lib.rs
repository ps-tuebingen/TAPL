pub mod errors;
pub mod eval;
pub mod lookup;
pub mod syntax;
pub mod typing;

#[cfg(test)]
pub mod test_common {
    use crate::syntax::{
        ClassDeclaration, ClassTable, ConstructorDeclaration, MethodDeclaration, Term,
    };
    use std::collections::HashMap;

    pub fn example_method() -> MethodDeclaration {
        MethodDeclaration {
            name: "setfst".to_owned(),
            class: "Pair".to_owned(),
            args: vec![("Object".to_owned(), "newfst".to_owned())],
            ret: Term::New(
                "Pair".to_owned(),
                vec![
                    Term::Var("newfst".to_owned()),
                    Term::FieldProjection(Box::new(Term::Var("this".to_owned())), "snd".to_owned()),
                ],
            ),
        }
    }

    pub fn example_pair() -> ClassDeclaration {
        ClassDeclaration {
            name: "Pair".to_owned(),
            parent: "Object".to_owned(),
            fields: vec![
                ("Object".to_owned(), "fst".to_owned()),
                ("Object".to_owned(), "snd".to_owned()),
            ],
            constructor: ConstructorDeclaration {
                name: "Pair".to_owned(),
                super_args: vec![],
                self_args: vec![
                    ("Object".to_owned(), "fst".to_owned()),
                    ("Object".to_owned(), "snd".to_owned()),
                ],
                self_fields: vec!["fst".to_owned(), "snd".to_owned()],
            },
            methods: vec![example_method()],
        }
    }

    pub fn example_a() -> ClassDeclaration {
        ClassDeclaration {
            name: "A".to_owned(),
            parent: "Object".to_owned(),
            fields: vec![],
            constructor: ConstructorDeclaration {
                name: "A".to_owned(),
                super_args: vec![],
                self_args: vec![],
                self_fields: vec![],
            },
            methods: vec![],
        }
    }

    pub fn example_b() -> ClassDeclaration {
        ClassDeclaration {
            name: "B".to_owned(),
            parent: "Object".to_owned(),
            fields: vec![],
            constructor: ConstructorDeclaration {
                name: "B".to_owned(),
                super_args: vec![],
                self_args: vec![],
                self_fields: vec![],
            },
            methods: vec![],
        }
    }

    pub fn example_table() -> ClassTable {
        HashMap::from([
            ("Object".to_owned(), ClassDeclaration::default()),
            ("A".to_owned(), example_a()),
            ("B".to_owned(), example_b()),
            ("Pair".to_owned(), example_pair()),
        ])
    }
}
