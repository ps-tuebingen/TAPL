use super::Subst;
use crate::syntax::{Cons, Head, IsNil, Nil, Tail, Term};
use common::Var;

impl Subst for Nil {
    type Target = Nil;
    fn subst(self, _: &Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for Cons {
    type Target = Cons;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Cons {
            inner_type: self.inner_type,
            fst: self.fst.subst(var, term.clone()),
            rst: self.rst.subst(var, term),
        }
    }
}

impl Subst for IsNil {
    type Target = IsNil;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        IsNil {
            inner_type: self.inner_type,
            list: self.list.subst(var, term),
        }
    }
}

impl Subst for Tail {
    type Target = Tail;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Tail {
            inner_type: self.inner_type,
            list: self.list.subst(var, term),
        }
    }
}

impl Subst for Head {
    type Target = Head;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Head {
            inner_type: self.inner_type,
            list: self.list.subst(var, term),
        }
    }
}

#[cfg(test)]
mod list_tests {
    use super::{Cons, Head, IsNil, Nil, Subst, Tail};
    use crate::types::Type;

    #[test]
    fn subst_nil() {
        let result = Nil {
            inner_type: Type::Bool,
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Nil {
            inner_type: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cons() {
        let result = Cons {
            inner_type: Type::Bool,
            fst: Box::new("x".to_owned().into()),
            rst: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Cons {
            inner_type: Type::Bool,
            fst: Box::new("y".to_owned().into()),
            rst: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_isnil() {
        let result = IsNil {
            inner_type: Type::Bool,
            list: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = IsNil {
            inner_type: Type::Bool,
            list: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_head() {
        let result = Head {
            inner_type: Type::Bool,
            list: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Head {
            inner_type: Type::Bool,
            list: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_tail() {
        let result = Tail {
            inner_type: Type::Bool,
            list: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Tail {
            inner_type: Type::Bool,
            list: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
