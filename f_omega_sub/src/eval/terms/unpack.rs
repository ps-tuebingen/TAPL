use crate::{
    errors::Error,
    eval::{Env, Eval, Value},
    syntax::terms::Unpack,
    traits::{SubstTerm, SubstTy},
};

impl Eval for Unpack {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let bound_val = self.bound_term.clone().eval(&mut env.clone())?;
        let (inner, val, _) = bound_val
            .as_pack()
            .map_err(|knd| Error::eval(knd, self.clone()))?;
        self.in_term
            .subst_ty(&self.ty_var, inner)
            .subst(&self.bound_var, val.into())
            .eval(env)
    }
}
