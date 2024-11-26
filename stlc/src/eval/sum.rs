use super::{errors::Error, Eval, Value};
use crate::{
    syntax::{Left, Right, SumCase},
    traits::subst::Subst,
};

impl Eval for Left {
    fn eval(self) -> Result<Value, Error> {
        let left_val = self.left_term.eval()?;
        Ok(Value::Left {
            left_term: Box::new(left_val),
            right_ty: self.right_ty,
        })
    }
}

impl Eval for Right {
    fn eval(self) -> Result<Value, Error> {
        let right_val = self.right_term.eval()?;
        Ok(Value::Right {
            right_term: Box::new(right_val),
            left_ty: self.left_ty,
        })
    }
}

impl Eval for SumCase {
    fn eval(self) -> Result<Value, Error> {
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
            _ => Err(Error::BadValue { val: bound_val }),
        }
    }
}
