use super::{errors::Error, Eval, Value};
use crate::terms::syntax::{Pair, Proj1, Proj2};

impl Eval for Pair {
    fn eval(self) -> Result<Value, Error> {
        let val_1 = self.fst.eval()?;
        let val_2 = self.snd.eval()?;
        Ok(Value::Pair {
            fst: Box::new(val_1),
            snd: Box::new(val_2),
        })
    }
}

impl Eval for Proj1 {
    fn eval(self) -> Result<Value, Error> {
        match self.pair.eval()? {
            Value::Pair { fst: v1, .. } => Ok(*v1),
            val => Err(Error::BadValue { val }),
        }
    }
}

impl Eval for Proj2 {
    fn eval(self) -> Result<Value, Error> {
        match self.pair.eval()? {
            Value::Pair { snd: v2, .. } => Ok(*v2),
            val => Err(Error::BadValue { val }),
        }
    }
}
