use super::Subst;
use crate::{
    syntax::{App, Lambda, Term},
    traits::free_vars::{fresh_var, FreeVars},
};
use common::Var;

impl Subst for Lambda {
    type Target = Lambda;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        let mut free_v = self.free_vars();
        free_v.insert(var.clone());
        free_v.insert(self.var.clone());
        let new_v = fresh_var(&free_v);
        let term_subst = self.body.subst(&self.var, new_v.clone().into());

        Lambda {
            var: new_v,
            annot: self.annot,
            body: term_subst.subst(var, term),
        }
    }
}

impl Subst for App {
    type Target = App;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        App {
            fun: self.fun.subst(var, term.clone()),
            arg: self.arg.subst(var, term),
        }
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::{App, Lambda, Subst};
    use crate::types::Type;

    #[test]
    fn subst_lambda() {
        let result = Lambda {
            var: "y".to_owned(),
            annot: Type::Bool,
            body: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Lambda {
            var: "x0".to_owned(),
            annot: Type::Bool,
            body: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_lambda_capture() {
        let result = Lambda {
            var: "x".to_owned(),
            annot: Type::Bool,
            body: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Lambda {
            var: "x0".to_owned(),
            annot: Type::Bool,
            body: Box::new("x0".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_app() {
        let result = App {
            fun: Box::new("x".to_owned().into()),
            arg: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = App {
            fun: Box::new("y".to_owned().into()),
            arg: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
