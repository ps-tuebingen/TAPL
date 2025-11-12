use super::SystemF;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes};
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{Forall, Fun, Type as TypeTrait, TypeGroup, TypeVariable},
};

#[derive(LatexFmt, LangDisplay, NoNorm, NoKinds, NoSubtypes, Debug, Clone, PartialEq, Eq)]
#[Lang(SystemF)]
pub enum Type {
    Var(TypeVariable<SystemF>),
    Fun(Fun<SystemF>),
    Forall(Forall<SystemF>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = SystemF;
    fn into_fun(self) -> Result<Fun<Self::Lang>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<Self::Lang>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<SystemF>::rule(),
            Fun::<SystemF>::rule(),
            Forall::<SystemF>::rule(),
        ])
    }
}

impl SubstType for Type {
    type Lang = SystemF;
    type Target = Self;

    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty).into(),
            Type::Forall(forall) => forall.subst_type(v, ty).into(),
        }
    }
}

impl From<TypeVariable<SystemF>> for Type {
    fn from(v: TypeVariable<SystemF>) -> Type {
        Type::Var(v)
    }
}

impl From<Fun<SystemF>> for Type {
    fn from(fun: Fun<SystemF>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Forall<SystemF>> for Type {
    fn from(forall: Forall<SystemF>) -> Type {
        Type::Forall(forall)
    }
}
