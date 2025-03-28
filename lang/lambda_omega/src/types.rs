use crate::{errors::Error, kinds::Kind};
use std::fmt;

pub type TypeVar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Unit,
    Lambda {
        var: TypeVar,
        annot: Kind,
        body: Box<Type>,
    },
    App {
        fun: Box<Type>,
        arg: Box<Type>,
    },
    Fun {
        from: Box<Type>,
        to: Box<Type>,
    },
    Forall {
        var: TypeVar,
        ty: Box<Type>,
    },
}

impl Type {
    pub fn as_fun(self) -> Result<(Type, Type), Error> {
        if let Type::Fun { from, to } = self {
            Ok((*from, *to))
        } else {
            Err(Error::TypeMismatch {
                found: self,
                expected: "Function Type".to_owned(),
            })
        }
    }

    pub fn as_forall(self) -> Result<(TypeVar, Type), Error> {
        if let Type::Forall { var, ty } = self {
            Ok((var, *ty))
        } else {
            Err(Error::TypeMismatch {
                found: self,
                expected: "Forall Type".to_owned(),
            })
        }
    }

    pub fn equiv(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Var(_), Type::Var(_)) => true,
            (Type::Lambda { body: body1, .. }, Type::Lambda { body: body2, .. }) => {
                body1.equiv(body2)
            }
            (
                Type::App {
                    fun: fun1,
                    arg: arg1,
                },
                Type::App {
                    fun: fun2,
                    arg: arg2,
                },
            ) => fun1.equiv(fun2) && arg1.equiv(arg2),
            (
                Type::Fun {
                    from: from1,
                    to: to1,
                },
                Type::Fun {
                    from: from2,
                    to: to2,
                },
            ) => from1.equiv(from2) && to1.equiv(to2),
            (Type::App { fun, arg }, ty) => {
                if let Type::Lambda {
                    var,
                    annot: _,
                    body,
                } = *(*fun).clone()
                {
                    body.subst(&var, (**arg).clone()) == *ty
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn subst(self, v: &TypeVar, ty: Type) -> Type {
        match self {
            Type::Var(var) => {
                if var == *v {
                    ty
                } else {
                    Type::Var(var)
                }
            }
            Type::Unit => Type::Unit,
            Type::Lambda { var, annot, body } => {
                if var == *v {
                    Type::Lambda { var, annot, body }
                } else {
                    Type::Lambda {
                        var,
                        annot,
                        body: Box::new(body.subst(v, ty)),
                    }
                }
            }
            Type::App { fun, arg } => Type::App {
                fun: Box::new(fun.subst(v, ty.clone())),
                arg: Box::new(arg.subst(v, ty)),
            },
            Type::Fun { from, to } => Type::Fun {
                from: Box::new(from.subst(v, ty.clone())),
                to: Box::new(to.subst(v, ty)),
            },
            Type::Forall { var, ty: body } => {
                if var == *v {
                    Type::Forall { var, ty: body }
                } else {
                    Type::Forall {
                        var,
                        ty: Box::new(body.subst(v, ty)),
                    }
                }
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => f.write_str(v),
            Type::Unit => f.write_str("Unit"),
            Type::Lambda { var, annot, body } => write!(f, "λ{}::{}.{}", var, annot, body),
            Type::App { fun, arg } => write!(f, "({fun}) ({arg})"),
            Type::Fun { from, to } => write!(f, "({from}) → ({to})"),
            Type::Forall { var, ty } => write!(f, "forall {var}.{ty}"),
        }
    }
}
