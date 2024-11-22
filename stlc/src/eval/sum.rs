use super::{Eval, Value};
use crate::terms::{
    subst::Subst,
    syntax::{Left, Right, SumCase},
};

impl Eval for Left {
    fn eval(self) -> Option<Value> {
        let left_val = self.left_term.eval()?;
        Some(Value::Left {
            left_term: Box::new(left_val),
            right_ty: self.right_ty,
        })
    }
}

impl Eval for Right {
    fn eval(self) -> Option<Value> {
        let right_val = self.right_term.eval()?;
        Some(Value::Right {
            right_term: Box::new(right_val),
            left_ty: self.left_ty,
        })
    }
}

impl Eval for SumCase {
    fn eval(self) -> Option<Value> {
        let bound_val = self.bound_term.eval()?;
        match bound_val {
            Value::Left {
                left_term: val,
                right_ty: _,
            } => self.left_term.subst(self.left_var, (*val).into()).eval(),
            Value::Right {
                right_term: val,
                left_ty: _,
            } => self.right_term.subst(self.right_var, (*val).into()).eval(),
            _ => None,
        }
    }
}
