use super::{Eval, Value};
use crate::terms::{
    subst::Subst,
    syntax::{App, Lambda},
};

impl Eval for Lambda {
    fn eval(self) -> Option<Value> {
        Some(Value::Lambda {
            var: self.var.clone(),
            annot: self.annot.clone(),
            body: *self.body.clone(),
        })
    }
}

impl Eval for App {
    fn eval(self) -> Option<Value> {
        let val1 = self.fun.eval()?;
        match val1 {
            Value::Lambda {
                var,
                annot: _,
                body,
            } => {
                let body_subst = body.subst(var, *self.arg);
                body_subst.eval()
            }
            _ => None,
        }
    }
}
