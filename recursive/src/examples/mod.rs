use crate::{
    terms::{App, Fix, Fold, Lambda, Term, Unfold},
    types::Type,
};

pub mod list;
pub mod nat;
pub mod objects;
pub mod process;
pub mod stream;
pub mod untyped_lambda;

pub fn ty_hungry() -> Type {
    Type::mu("A", Type::fun(Type::Nat, "A".into()))
}

pub fn hungry() -> Term {
    Fold::new(
        Fix::new(
            Lambda::new(
                "f",
                Type::fun(Type::Nat, ty_hungry()),
                Lambda::new("n", Type::Nat, Fold::new("f".into(), ty_hungry()).into()).into(),
            )
            .into(),
        )
        .into(),
        ty_hungry(),
    )
    .into()
}

pub fn fix_t(t: Type) -> Term {
    let x_ty = Type::mu("A", Type::fun("A".into(), t.clone()));
    let x_lam = Lambda::new(
        "x",
        x_ty.clone(),
        App::new(
            "f".into(),
            App::new(Unfold::new("x".into(), x_ty.clone()).into(), "x".into()).into(),
        )
        .into(),
    );
    Lambda::new(
        "f",
        Type::fun(t.clone(), t.clone()),
        App::new(x_lam.clone().into(), Fold::new(x_lam.into(), x_ty).into()).into(),
    )
    .into()
}

pub fn diverge_t(t: Type) -> Term {
    Lambda::new(
        "_",
        Type::Unit,
        App::new(fix_t(t.clone()), Lambda::new("x", t, "x".into()).into()).into(),
    )
    .into()
}

#[cfg(test)]
mod example_tests {
    use super::{diverge_t, fix_t, hungry, ty_hungry};
    use crate::{check::Check, types::Type};

    #[test]
    fn check_hungry() {
        let result = hungry().check(&mut Default::default()).unwrap();
        let expected = ty_hungry();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fix() {
        let result = fix_t(Type::Nat)
            .check(&mut Default::default())
            .map_err(|err| err.to_string())
            .unwrap();
        let expected = Type::fun(Type::fun(Type::Nat, Type::Nat), Type::Nat);
        assert_eq!(result, expected);

        let result = fix_t(Type::Bool).check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::fun(Type::Bool, Type::Bool), Type::Bool);
        assert_eq!(result, expected);

        let result = fix_t(Type::Unit).check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::fun(Type::Unit, Type::Unit), Type::Unit);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_diverge() {
        let result = diverge_t(Type::Nat).check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, Type::Nat);
        assert_eq!(result, expected);

        let result = diverge_t(Type::Bool)
            .check(&mut Default::default())
            .unwrap();
        let expected = Type::fun(Type::Unit, Type::Bool);
        assert_eq!(result, expected);

        let result = diverge_t(Type::Unit)
            .check(&mut Default::default())
            .unwrap();
        let expected = Type::fun(Type::Unit, Type::Unit);
        assert_eq!(result, expected)
    }
}
