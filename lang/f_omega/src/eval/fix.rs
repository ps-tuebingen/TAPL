use super::{to_eval_err, Value};
use crate::{
    syntax::terms::{Fix, Lambda},
    traits::SubstTerm,
};
use common::{errors::Error, Eval};

impl Eval<'_> for Fix {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let inner_val = self.term.clone().eval(_env)?;
        let (var, annot, body) = inner_val.as_lambda().map_err(to_eval_err)?;

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
