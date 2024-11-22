use crate::{Var,terms::syntax::{Term,Nothing,Something}};
use super::Subst;

impl Subst for Nothing{
    type Target = Nothing;
    fn subst(self,_:Var,_:Term) -> Self::Target{
        self
    }
}
impl Subst for Something{
    type Target = Something;
    fn subst(self,var:Var,term:Term) -> Self::Target{
        Something{term:self.term.subst(var,term)
        }
    }
}
