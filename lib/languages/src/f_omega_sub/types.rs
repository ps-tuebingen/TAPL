use super::FOmegaSub;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{
        ExistsBounded, ForallBounded, Fun, Nat, OpApp, OpLambdaSub, Record, Top, Type as TypeTrait,
        TypeGroup, TypeVariable,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<FOmegaSub>),
    Top(Top<FOmegaSub>),
    Fun(Fun<FOmegaSub>),
    Forall(ForallBounded<FOmegaSub>),
    OpLambdaSub(OpLambdaSub<FOmegaSub>),
    OpApp(OpApp<FOmegaSub>),
    Exists(ExistsBounded<FOmegaSub>),
    Record(Record<FOmegaSub>),
    Nat(Nat<FOmegaSub>),
}

impl TypeTrait for Type {}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<FOmegaSub>::rule(),
            Top::<FOmegaSub>::rule(),
            Fun::<FOmegaSub>::rule(),
            ForallBounded::<FOmegaSub>::rule(),
            OpLambdaSub::<FOmegaSub>::rule(),
            OpApp::<FOmegaSub>::rule(),
            ExistsBounded::<FOmegaSub>::rule(),
            Record::<FOmegaSub>::rule(),
            Nat::<FOmegaSub>::rule(),
        ])
    }
}

impl TypeGroup for Type {
    type Lang = FOmegaSub;
    fn into_variable(self) -> Result<TypeVariable<FOmegaSub>, TypeMismatch> {
        if let Type::Var(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
        }
    }
    fn into_top(self) -> Result<Top<FOmegaSub>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<FOmegaSub>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<FOmegaSub>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<FOmegaSub>, TypeMismatch> {
        if let Type::OpLambdaSub(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
        }
    }

    fn into_opapp(self) -> Result<OpApp<FOmegaSub>, TypeMismatch> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpApp".to_owned()))
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<FOmegaSub>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<FOmegaSub>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<FOmegaSub>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }
}

impl SubstType for Type {
    type Lang = FOmegaSub;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Top(top) => top.subst_type(v, ty).into(),
            Type::Fun(fun) => fun.subst_type(v, ty).into(),
            Type::Forall(forall) => forall.subst_type(v, ty).into(),
            Type::OpLambdaSub(lam) => lam.subst_type(v, ty).into(),
            Type::OpApp(app) => app.subst_type(v, ty).into(),
            Type::Exists(ex) => ex.subst_type(v, ty).into(),
            Type::Record(rec) => rec.subst_type(v, ty).into(),
            Type::Nat(nat) => nat.subst_type(v, ty).into(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::Top(top) => top.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
            Type::OpLambdaSub(lam) => lam.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Exists(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(var) => var.to_latex(conf),
            Type::Top(top) => top.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Forall(forall) => forall.to_latex(conf),
            Type::OpLambdaSub(lam) => lam.to_latex(conf),
            Type::OpApp(app) => app.to_latex(conf),
            Type::Exists(ex) => ex.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
            Type::Nat(nat) => nat.to_latex(conf),
        }
    }
}
impl From<Top<FOmegaSub>> for Type {
    fn from(t: Top<FOmegaSub>) -> Type {
        Type::Top(t)
    }
}

impl From<TypeVariable<FOmegaSub>> for Type {
    fn from(var: TypeVariable<FOmegaSub>) -> Type {
        Type::Var(var)
    }
}
impl From<OpLambdaSub<FOmegaSub>> for Type {
    fn from(oplam: OpLambdaSub<FOmegaSub>) -> Type {
        Type::OpLambdaSub(oplam)
    }
}
impl From<Fun<FOmegaSub>> for Type {
    fn from(fun: Fun<FOmegaSub>) -> Type {
        Type::Fun(fun)
    }
}
impl From<OpApp<FOmegaSub>> for Type {
    fn from(opapp: OpApp<FOmegaSub>) -> Type {
        Type::OpApp(opapp)
    }
}
impl From<ForallBounded<FOmegaSub>> for Type {
    fn from(forall: ForallBounded<FOmegaSub>) -> Type {
        Type::Forall(forall)
    }
}
impl From<ExistsBounded<FOmegaSub>> for Type {
    fn from(exists: ExistsBounded<FOmegaSub>) -> Type {
        Type::Exists(exists)
    }
}
impl From<Record<FOmegaSub>> for Type {
    fn from(rec: Record<FOmegaSub>) -> Type {
        Type::Record(rec)
    }
}
impl From<Nat<FOmegaSub>> for Type {
    fn from(n: Nat<FOmegaSub>) -> Type {
        Type::Nat(n)
    }
}
