use super::{Eval, Value};
use crate::terms::syntax::{Cons, Head, IsNil, Nil, Tail};

impl Eval for Nil {
    fn eval(self) -> Option<Value> {
        Some(Value::Nil {
            inner_type: self.inner_type,
        })
    }
}

impl Eval for Cons {
    fn eval(self) -> Option<Value> {
        let fst_val = self.fst.eval()?;
        let rst_val = self.rst.eval()?;
        Some(Value::Cons {
            inner_type: self.inner_type,
            fst: Box::new(fst_val),
            rst: Box::new(rst_val),
        })
    }
}

impl Eval for IsNil {
    fn eval(self) -> Option<Value> {
        match self.list.eval() {
            Some(Value::Nil { .. }) => Some(Value::True),
            Some(Value::Cons { .. }) => Some(Value::False),
            _ => None,
        }
    }
}

impl Eval for Head {
    fn eval(self) -> Option<Value> {
        match self.list.eval() {
            Some(Value::Cons { fst, .. }) => Some(*fst),
            _ => None,
        }
    }
}

impl Eval for Tail {
    fn eval(self) -> Option<Value> {
        match self.list.eval() {
            Some(Value::Cons { rst, .. }) => Some(*rst),
            _ => None,
        }
    }
}
