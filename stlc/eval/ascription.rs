use super::{Eval, Value};
use crate::terms::syntax::Ascribe;

impl Eval for Ascribe {
    fn eval(self) -> Option<Value> {
        self.term.eval()
    }
}
