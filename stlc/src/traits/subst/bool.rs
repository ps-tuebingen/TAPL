use super::Subst;
use crate::{
    syntax::{False, If, Term, True},
    Var,
};

impl Subst for True {
    type Target = True;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for False {
    type Target = False;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for If {
    type Target = If;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        If {
            ifc: self.ifc.subst(var.clone(), term.clone()),
            thenc: self.thenc.subst(var.clone(), term.clone()),
            elsec: self.elsec.subst(var, term),
        }
    }
}

#[cfg(test)]
mod bool_tests {
    use super::{False, If, Subst, True};

    #[test]
    fn subst_true() {
        let result = True.subst("x".to_owned(), "y".to_owned().into());
        let expected = True;
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_false() {
        let result = False.subst("x".to_owned(), "y".to_owned().into());
        let expected = False;
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_if() {
        let result = If {
            ifc: Box::new("x".to_owned().into()),
            thenc: Box::new("y".to_owned().into()),
            elsec: Box::new("x".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = If {
            ifc: Box::new("y".to_owned().into()),
            thenc: Box::new("y".to_owned().into()),
            elsec: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
