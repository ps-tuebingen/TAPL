use super::{errors::Error, Check, TypingEnv};
use crate::{syntax::Let, types::Type};

impl Check for Let {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check_local(env)?;
        env.used_vars.insert(self.var.clone(), bound_ty);
        self.in_term.check(env)
    }
}

#[cfg(test)]
mod let_tests {
    use super::{Check, Let};
    use crate::{syntax::Zero, types::Type};

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
