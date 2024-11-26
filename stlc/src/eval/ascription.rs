use super::{errors::Error, Eval, Value};
use crate::syntax::Ascribe;

impl Eval for Ascribe {
    fn eval(self) -> Result<Value, Error> {
        self.term.eval()
    }
}
