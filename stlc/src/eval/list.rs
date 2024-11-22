use super::{errors::Error, Eval, Value};
use crate::terms::syntax::{Cons, Head, IsNil, Nil, Tail};

impl Eval for Nil {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Nil {
            inner_type: self.inner_type,
        })
    }
}

impl Eval for Cons {
    fn eval(self) -> Result<Value, Error> {
        let fst_val = self.fst.eval()?;
        let rst_val = self.rst.eval()?;
        Ok(Value::Cons {
            inner_type: self.inner_type,
            fst: Box::new(fst_val),
            rst: Box::new(rst_val),
        })
    }
}

impl Eval for IsNil {
    fn eval(self) -> Result<Value, Error> {
        match self.list.eval()? {
            Value::Nil { .. } => Ok(Value::True),
            Value::Cons { .. } => Ok(Value::False),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl Eval for Head {
    fn eval(self) -> Result<Value, Error> {
        match self.list.eval()? {
            Value::Cons { fst, .. } => Ok(*fst),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl Eval for Tail {
    fn eval(self) -> Result<Value, Error> {
        match self.list.eval()? {
            Value::Cons { rst, .. } => Ok(*rst),
            val => Err(Error::BadValue { val }),
        }
    }
}
