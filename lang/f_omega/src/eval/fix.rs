use super::{Eval, Value};
use crate::{
    errors::Error,
    syntax::terms::{Fix, Lambda},
    traits::SubstTerm,
};

impl Eval for Fix {
    fn eval(self) -> Result<Value, Error> {
        let inner_val = self.term.clone().eval()?;
        let (var, annot, body) = inner_val
            .as_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;

        body.clone()
            .subst(
                &var,
                Fix {
                    term: Box::new(
                        Lambda {
                            var: var.clone(),
                            annot,
                            body: Box::new(body),
                        }
                        .into(),
                    ),
                }
                .into(),
            )
            .eval()
    }
}
