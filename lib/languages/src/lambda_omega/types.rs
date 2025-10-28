use super::LambdaOmega;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{
        Bool, Forall, Fun, Nat, OpApp, OpLambda, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
    },
};

pub type TypeVar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<LambdaOmega>),
    Unit(Unit<LambdaOmega>),
    Nat(Nat<LambdaOmega>),
    Bool(Bool<LambdaOmega>),
    OpLambda(OpLambda<LambdaOmega>),
    OpApp(OpApp<LambdaOmega>),
    Fun(Fun<LambdaOmega>),
    Forall(Forall<LambdaOmega>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = LambdaOmega;
    fn into_variable(self) -> Result<TypeVariable<LambdaOmega>, TypeMismatch> {
        if let Type::Var(v) = self {
            Ok(v)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
        }
    }

    fn into_unit(self) -> Result<Unit<LambdaOmega>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<LambdaOmega>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<LambdaOmega>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<LambdaOmega>, TypeMismatch> {
        if let Type::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
        }
    }

    fn into_opapp(self) -> Result<OpApp<LambdaOmega>, TypeMismatch> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpApp".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<LambdaOmega>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<LambdaOmega>, TypeMismatch> {
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
            TypeVariable::<LambdaOmega>::rule(),
            Unit::<LambdaOmega>::rule(),
            Bool::<LambdaOmega>::rule(),
            Nat::<LambdaOmega>::rule(),
            OpLambda::<LambdaOmega>::rule(),
            OpApp::<LambdaOmega>::rule(),
            Fun::<LambdaOmega>::rule(),
            Forall::<LambdaOmega>::rule(),
        ])
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::OpLambda(oplam) => oplam.fmt(f),
            Type::OpApp(opapp) => opapp.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(var) => var.to_latex(conf),
            Type::Unit(u) => u.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
            Type::Nat(n) => n.to_latex(conf),
            Type::OpLambda(oplam) => oplam.to_latex(conf),
            Type::OpApp(opapp) => opapp.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Forall(forall) => forall.to_latex(conf),
        }
    }
}

impl SubstType for Type {
    type Lang = LambdaOmega;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Unit(u) => u.subst_type(v, ty).into(),
            Type::Bool(b) => b.subst_type(v, ty).into(),
            Type::Nat(n) => n.subst_type(v, ty).into(),
            Type::OpLambda(oplam) => oplam.subst_type(v, ty).into(),
            Type::OpApp(opapp) => opapp.subst_type(v, ty).into(),
            Type::Fun(fun) => fun.subst_type(v, ty).into(),
            Type::Forall(forall) => forall.subst_type(v, ty).into(),
        }
    }
}

impl From<TypeVariable<LambdaOmega>> for Type {
    fn from(var: TypeVariable<LambdaOmega>) -> Type {
        Type::Var(var)
    }
}
impl From<Unit<LambdaOmega>> for Type {
    fn from(u: Unit<LambdaOmega>) -> Type {
        Type::Unit(u)
    }
}
impl From<Nat<LambdaOmega>> for Type {
    fn from(n: Nat<LambdaOmega>) -> Type {
        Type::Nat(n)
    }
}
impl From<Bool<LambdaOmega>> for Type {
    fn from(b: Bool<LambdaOmega>) -> Type {
        Type::Bool(b)
    }
}
impl From<OpLambda<LambdaOmega>> for Type {
    fn from(lam: OpLambda<LambdaOmega>) -> Type {
        Type::OpLambda(lam)
    }
}
impl From<OpApp<LambdaOmega>> for Type {
    fn from(app: OpApp<LambdaOmega>) -> Type {
        Type::OpApp(app)
    }
}
impl From<Fun<LambdaOmega>> for Type {
    fn from(fun: Fun<LambdaOmega>) -> Type {
        Type::Fun(fun)
    }
}
impl From<Forall<LambdaOmega>> for Type {
    fn from(forall: Forall<LambdaOmega>) -> Type {
        Type::Forall(forall)
    }
}
