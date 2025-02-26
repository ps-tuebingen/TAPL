use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::FixBeta, congruence},
    syntax::Fix,
};

impl AsContext for Fix {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.term).try_into() {
            Ok(val) => Ok(FixBeta { lam: val }.into()),
            Err(_) => {
                let ctx = (*self.term).to_context()?;
                Ok(congruence::Fix {
                    term: Box::new(ctx),
                }
                .into())
            }
        }
    }
}
