use crate::types::Type;
use common::{
    kinds::Kind,
    types::{ExistsBounded, OpApp, OpLambda, Record, TypeVariable},
    Label, TypeVar,
};
use std::collections::HashMap;

pub mod counter;
pub mod reset_counter;

pub fn object() -> Type {
    OpLambda::new(
        "M",
        Kind::Star.abs(),
        ExistsBounded::new_unbounded(
            "X",
            Record::new(HashMap::<Label, Type>::from([
                ("state".to_owned(), TypeVariable::new("X").into()),
                (
                    "methods".to_owned(),
                    OpApp::new(TypeVariable::new("M"), TypeVariable::new("X")).into(),
                ),
            ])),
        ),
    )
    .into()
}

#[cfg(test)]
mod object_tests {
    use super::object;
    use common::{check::Kindcheck, kinds::Kind};

    #[test]
    fn check_object() {
        let result = object().check_kind(&mut Default::default()).unwrap();
        let expected = Kind::Arrow(
            Box::new(Kind::Arrow(Box::new(Kind::Star), Box::new(Kind::Star))),
            Box::new(Kind::Star),
        );
        assert_eq!(result, expected)
    }
}
