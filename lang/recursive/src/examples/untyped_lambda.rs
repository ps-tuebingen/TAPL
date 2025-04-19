use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Fold, Lambda, Unfold, Variable},
    types::{Fun, Mu, TypeVariable},
    Var,
};

pub fn d() -> Type {
    Mu::new(
        "X",
        Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
    )
    .into()
}

pub fn lam() -> Term {
    Lambda::new("f", Fun::new(d(), d()), Fold::new(Variable::new("f"), d())).into()
}

pub fn ap() -> Term {
    Lambda::new(
        "f",
        d(),
        Lambda::new(
            "a",
            d(),
            App::new(Unfold::new(d(), Variable::new("f")), Variable::new("a")),
        ),
    )
    .into()
}

pub enum LambdaTerm {
    Var(Var),
    Lambda(Var, Box<LambdaTerm>),
    App(Box<LambdaTerm>, Box<LambdaTerm>),
}

pub fn lambda_to_term(t: LambdaTerm) -> Term {
    match t {
        LambdaTerm::Var(v) => Variable::new(&v).into(),
        LambdaTerm::Lambda(v, body) => {
            App::new(lam(), Lambda::new(&v, d(), lambda_to_term(*body))).into()
        }
        LambdaTerm::App(fun, arg) => {
            App::new(App::new(ap(), lambda_to_term(*fun)), lambda_to_term(*arg)).into()
        }
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::{ap, d, lam};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn ty_lam() {
        let result = lam().check(&mut Default::default()).unwrap();
        let expected = Fun::new(Fun::new(d(), d()), d());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_app() {
        let result = ap().check(&mut Default::default()).unwrap();
        let expected = Fun::new(d(), Fun::new(d(), d()));
        assert_eq!(result, expected)
    }
}
