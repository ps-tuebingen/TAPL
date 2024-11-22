use super::{Eval, Value};
use crate::terms::{
    subst::Subst,
    syntax::{Variant, VariantCase, VariantPattern},
};

impl Eval for Variant {
    fn eval(self) -> Option<Value> {
        let val = self.term.eval()?;
        Some(Value::Variant {
            label: self.label,
            ty: self.ty,
            val: Box::new(val),
        })
    }
}

impl Eval for VariantCase {
    fn eval(self) -> Option<Value> {
        let bound_val = self.bound_term.eval()?;
        let (lb, val) = if let Value::Variant { label, ty: _, val } = bound_val {
            Some((label, val))
        } else {
            None
        }?;
        let VariantPattern {
            label: _,
            bound_var,
            rhs,
        } = self.cases.into_iter().find(
            |VariantPattern {
                 label,
                 bound_var: _,
                 rhs: _,
             }| *label == lb,
        )?;
        rhs.subst(bound_var, (*val).into()).eval()
    }
}
