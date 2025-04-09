use crate::{
    terms::{App, Fold, Lambda, Term, Unfold, Var},
    types::Type,
};

pub fn d() -> Type {
    Type::mu("X", Type::fun("X".into(), "X".into()))
}

pub fn lam() -> Term {
    Lambda::new("f", Type::fun(d(), d()), Fold::new("f".into(), d()).into()).into()
}

pub fn ap() -> Term {
    Lambda::new(
        "f",
        d(),
        Lambda::new(
            "a",
            d(),
            App::new(Unfold::new("f".into(), d()).into(), "a".into()).into(),
        )
        .into(),
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
        LambdaTerm::Var(v) => v.into(),
        LambdaTerm::Lambda(v, body) => {
            App::new(lam(), Lambda::new(&v, d(), lambda_to_term(*body)).into()).into()
        }
        LambdaTerm::App(fun, arg) => App::new(
            App::new(ap(), lambda_to_term(*fun)).into(),
            lambda_to_term(*arg),
        )
        .into(),
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
        let expected = Type::fun(Type::fun(d(), d()), d());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_app() {
        let result = ap().check(&mut Default::default()).unwrap();
        let expected = Type::fun(d(), Type::fun(d(), d()));
        assert_eq!(result, expected)
    }
}
