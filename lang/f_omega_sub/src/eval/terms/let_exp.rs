use crate::{
    errors::Error,
    eval::{Env, Eval, Value},
    syntax::terms::Let,
    traits::SubstTerm,
};

impl Eval for Let {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let bound_val = self.bound_term.eval(&mut env.clone())?;
        self.in_term.subst(&self.var, bound_val.into()).eval(env)
    }
}
