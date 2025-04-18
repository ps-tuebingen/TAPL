use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Fix, Fold, Lambda, Unfold, Variable},
    types::{Fun, Mu, Nat, TypeVariable, Unit},
};

pub mod list;
pub mod nat;
pub mod objects;
pub mod process;
pub mod stream;
pub mod untyped_lambda;

pub fn ty_hungry() -> Type {
    Mu::new("A", Fun::new(Nat, TypeVariable::new("A"))).into()
}

pub fn hungry() -> Term {
    Fold::new(
        Fix::new(Lambda::new(
            "f",
            Fun::new(Nat, ty_hungry()),
            Lambda::new("n", Nat, Fold::new(Variable::new("f"), ty_hungry())),
        )),
        ty_hungry(),
    )
    .into()
}

pub fn fix_t(t: Type) -> Term {
    let x_ty: Type = Mu::new("A", Fun::new(TypeVariable::new("A".into()), t.clone())).into();
    let x_lam: Term = Lambda::new(
        "x",
        x_ty.clone(),
        App::new(
            Variable::new("f"),
            App::new(
                Unfold::new(x_ty.clone(), Variable::new("x")),
                Variable::new("x"),
            ),
        ),
    )
    .into();
    Lambda::new(
        "f",
        Fun::new(t.clone(), t.clone()),
        App::new(x_lam.clone(), Fold::new(x_lam, x_ty)),
    )
    .into()
}

pub fn diverge_t(t: Type) -> Term {
    Lambda::new(
        "_",
        Unit,
        App::new(fix_t(t.clone()), Lambda::new("x", t, Variable::new("x"))),
    )
    .into()
}

#[cfg(test)]
mod example_tests {
    use super::{diverge_t, fix_t, hungry, ty_hungry};
    use common::{
        check::Typecheck,
        types::{Bool, Fun, Nat, Unit},
    };

    #[test]
    fn check_hungry() {
        let result = hungry().check_start().unwrap();
        let expected = ty_hungry();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fix() {
        let result = fix_t(Nat.into())
            .check(&mut Default::default())
            .map_err(|err| err.to_string())
            .unwrap();
        let expected = Fun::new(Fun::new(Nat, Nat), Nat).into();
        assert_eq!(result, expected);

        let result = fix_t(Bool.into()).check(&mut Default::default()).unwrap();
        let expected = Fun::new(Fun::new(Bool, Bool), Bool).into();
        assert_eq!(result, expected);

        let result = fix_t(Unit.into()).check(&mut Default::default()).unwrap();
        let expected = Fun::new(Fun::new(Unit, Unit), Unit).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_diverge() {
        let result = diverge_t(Nat.into())
            .check(&mut Default::default())
            .unwrap();
        let expected = Fun::new(Unit, Nat).into();
        assert_eq!(result, expected);

        let result = diverge_t(Bool.into())
            .check(&mut Default::default())
            .unwrap();
        let expected = Fun::new(Unit, Bool).into();
        assert_eq!(result, expected);

        let result = diverge_t(Unit.into())
            .check(&mut Default::default())
            .unwrap();
        let expected = Fun::new(Unit, Unit).into();
        assert_eq!(result, expected)
    }
}
