use super::{to_check_err, TypingContext};
use crate::{syntax::Fix, types::Type};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Fix {
    type Type = Type;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let inner = self.term.check(env)?;
        if let Type::Fun { from, to } = inner {
            if from == to {
                Ok(*from)
            } else {
                Err(to_check_err(ErrorKind::TypeMismatch {
                    found: from.to_string(),
                    expected: to.to_string(),
                }))
            }
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: inner.to_string(),
                expected: "Function Type".to_owned(),
            }))
        }
    }
}
