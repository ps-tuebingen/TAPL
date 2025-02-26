use super::Eval;
use crate::{
    errors::{Error, ErrorKind},
    terms::{False, If, Term, True},
    traits::is_value::IsValue,
};

impl Eval for True {
    fn eval_once(self) -> Result<Term, Error> {
        Ok(self.into())
    }
}

impl Eval for False {
    fn eval_once(self) -> Result<Term, Error> {
        Ok(self.into())
    }
}

impl Eval for If {
    fn eval_once(self) -> Result<Term, Error> {
        if !self.ifc.is_value() {
            let ifc_evaled = self.ifc.eval_once()?;
            Ok(If {
                ifc: Box::new(ifc_evaled),
                thenc: self.thenc,
                elsec: self.elsec,
            }
            .into())
        } else {
            match *self.ifc {
                Term::True(_) => Ok(*self.thenc),
                Term::False(_) => Ok(*self.elsec),
                _ => Err(Error::eval(
                    ErrorKind::UnexpectedTerm {
                        found: *self.ifc.clone(),
                        expected: "Boolean Constant".into(),
                    },
                    &self,
                )),
            }
        }
    }
}
