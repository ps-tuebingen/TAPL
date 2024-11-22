use super::{Eval, Value};
use crate::terms::{subst::Subst, syntax::Fix};

impl Eval for Fix {
    fn eval(self) -> Option<Value> {
        let fix_val = self.term.eval()?;
        if let Value::Lambda { var, annot, body } = fix_val {
            body.clone()
                .subst(var.clone(), Value::Lambda { var, annot, body }.into())
                .eval()
        } else {
            None
        }
    }
}
