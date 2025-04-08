use super::Value;
use crate::{
    errors::Error,
    syntax::terms::{Fix, Lambda},
    traits::SubstTerm,
};
use common::Eval;

impl Eval<'_> for Fix {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let inner_val = self.term.clone().eval(_env)?;
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
            .eval(_env)
    }
}
