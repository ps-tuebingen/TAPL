use super::{Eval, Value};
use crate::terms::syntax::{Pair, Proj1, Proj2};

impl Eval for Pair {
    fn eval(self) -> Option<Value> {
        let val_1 = self.fst.eval()?;
        let val_2 = self.snd.eval()?;
        Some(Value::Pair {
            fst: Box::new(val_1),
            snd: Box::new(val_2),
        })
    }
}

impl Eval for Proj1 {
    fn eval(self) -> Option<Value> {
        match self.pair.eval() {
            Some(Value::Pair { fst: v1, .. }) => Some(*v1),
            _ => None,
        }
    }
}

impl Eval for Proj2 {
    fn eval(self) -> Option<Value> {
        match self.pair.eval() {
            Some(Value::Pair { snd: v2, .. }) => Some(*v2),
            _ => None,
        }
    }
}
