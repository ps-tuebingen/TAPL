use super::{errors::Error, TypingEnv};
use crate::{syntax::Let, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Let {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        env.used_vars.insert(self.var.clone(), bound_ty);
        self.in_term.check(env)
    }
}

#[cfg(test)]
mod let_tests {
    use super::Let;
    use crate::{syntax::Zero, types::Type};
    use common::Typecheck;

    #[test]
    fn check_let() {
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(Zero.into()),
            in_term: Box::new("x".to_owned().into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
