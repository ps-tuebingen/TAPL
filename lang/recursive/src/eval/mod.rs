use super::{errors::Error, terms::Term};

pub mod bool;
pub mod fix;
pub mod fold;
pub mod lambda;
pub mod let_exp;
pub mod nat;
pub mod pair;
pub mod record;
pub mod terms;
pub mod variant;

pub trait Eval {
    fn eval_once(self) -> Result<Term, Error>;
    fn eval(self) -> Result<Term, Error>
    where
        Self: Sized + Clone + Into<Term>,
    {
        let evaled = self.clone().eval_once()?;
        if evaled == self.into() {
            Ok(evaled)
        } else {
            evaled.eval()
        }
    }
}
