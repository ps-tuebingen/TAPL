use super::{Eval, Value};
use crate::terms::syntax::{Proj, Tup};

impl Eval for Tup {
    fn eval(self) -> Option<Value> {
        let mut vals = vec![];
        for term in self.terms.into_iter() {
            let val = term.eval()?;
            vals.push(val);
        }
        Some(Value::Tup(vals))
    }
}

impl Eval for Proj {
    fn eval(self) -> Option<Value> {
        let tup_val = self.tup.eval()?;
        if let Value::Tup(vals) = tup_val {
            vals.get(self.ind).cloned()
        } else {
            None
        }
    }
}
