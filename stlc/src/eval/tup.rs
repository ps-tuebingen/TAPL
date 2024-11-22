use super::{errors::Error, Eval, Value};
use crate::terms::syntax::{Proj, Tup};

impl Eval for Tup {
    fn eval(self) -> Result<Value, Error> {
        let mut vals = vec![];
        for term in self.terms.into_iter() {
            let val = term.eval()?;
            vals.push(val);
        }
        Ok(Value::Tup(vals))
    }
}

impl Eval for Proj {
    fn eval(self) -> Result<Value, Error> {
        let tup_val = self.tup.eval()?;
        if let Value::Tup(vals) = tup_val {
            vals.get(self.ind)
                .cloned()
                .ok_or(Error::ProjectionOutOfBounds {
                    found: vals.len(),
                    expected: self.ind,
                })
        } else {
            Err(Error::BadValue { val: tup_val })
        }
    }
}
