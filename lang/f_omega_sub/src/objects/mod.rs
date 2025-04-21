use common::{
    kinds::Kind,
    types::{Exists, OpApp, OpLambda, Record, Type},
};

pub mod counter;
pub mod reset_counter;

pub fn object() -> Type {
    OpLambda::new(
        "M",
        Kind::Star.abs(),
        Exists::new_unbounded(
            "X",
            Kind::Star,
            RecordTy::new(vec![
                ("state", "X".into()),
                ("methods", OpApp::new("M", "X").into()),
            ]),
        ),
    )
    .into()
}

#[cfg(test)]
mod object_tests {
    use super::object;
    use crate::syntax::kinds::Kind;
    use common::Typecheck;

    #[test]
    fn check_object() {
        let result = object().check(&mut Default::default()).unwrap();
        let expected = Kind::Star.abs().abs();
        assert_eq!(result, expected)
    }
}
