use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::{Error, ErrorKind};
use syntax::{
    subst::SubstType,
    terms::{Term, TyApp},
    types::Type,
};

impl<T, Ty> Eval for TyApp<T, Ty>
where
    T: Term + Eval + SubstType<Ty, Target = T>,
    Ty: Type,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let fun_val = self.fun.eval(env)?;
        if let Ok(tylam) = fun_val.clone().into_tylambda() {
            tylam.term.subst_type(&tylam.var, &self.arg).eval(env)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            lamsub.term.subst_type(&lamsub.var, &self.arg).eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: fun_val.to_string(),
                expected: "Type Abstraction".to_owned(),
            }))
        }
    }
}
