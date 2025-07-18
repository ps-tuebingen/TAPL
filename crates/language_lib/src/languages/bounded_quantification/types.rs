use check::Normalize;
use errors::{TypeKind, TypeMismatch};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    env::Environment,
    subst::SubstType,
    types::{
        ExistsBounded, ForallBounded, Fun, Nat, Record, Top, Type as TypeTrait, TypeGroup,
        TypeVariable,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Top(Top<Type>),
    Nat(Nat<Type>),
    Fun(Fun<Type>),
    Forall(ForallBounded<Type>),
    Exists(ExistsBounded<Type>),
    Record(Record<Type>),
}

impl syntax::types::Type for Type {
    fn knd(&self) -> TypeKind {
        match self {
            Type::Var(t) => t.knd(),
            Type::Top(t) => t.knd(),
            Type::Nat(t) => t.knd(),
            Type::Fun(t) => t.knd(),
            Type::Forall(t) => t.knd(),
            Type::Exists(t) => t.knd(),
            Type::Record(t) => t.knd(),
        }
    }
}

impl TypeGroup for Type {
    fn into_variable(self) -> Result<TypeVariable<Type>, TypeMismatch> {
        if let Type::Var(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Variable))
        }
    }
    fn into_top(self) -> Result<Top<Type>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Top))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Function))
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Type>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Universal))
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<Type>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Existential))
        }
    }

    fn into_record(self) -> Result<Record<Type>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Record))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<Type>::rule(),
            Top::<Type>::rule(),
            Nat::<Type>::rule(),
            Fun::<Type>::rule(),
            ForallBounded::<Type>::rule(),
            ExistsBounded::<Type>::rule(),
            Record::<Type>::rule(),
        ])
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(v: TypeVariable<Type>) -> Type {
        Type::Var(v)
    }
}
impl From<Top<Type>> for Type {
    fn from(t: Top<Type>) -> Type {
        Type::Top(t)
    }
}

impl From<Nat<Type>> for Type {
    fn from(nat: Nat<Type>) -> Type {
        Type::Nat(nat)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<ForallBounded<Type>> for Type {
    fn from(forall: ForallBounded<Type>) -> Type {
        Type::Forall(forall)
    }
}

impl From<ExistsBounded<Type>> for Type {
    fn from(exists: ExistsBounded<Type>) -> Type {
        Type::Exists(exists)
    }
}

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}

impl SubstType<Self> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Self) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Top(t) => t.subst_type(v, ty),
            Type::Nat(n) => n.subst_type(v, ty),
            Type::Fun(f) => f.subst_type(v, ty),
            Type::Forall(f) => f.subst_type(v, ty),
            Type::Exists(e) => e.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
        }
    }
}
impl Normalize<Type> for Type {
    fn normalize(self, _: Environment<Type>) -> Type {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => v.fmt(f),
            Type::Top(t) => t.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
            Type::Exists(e) => e.fmt(f),
            Type::Record(rec) => rec.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(v) => v.to_latex(conf),
            Type::Top(t) => t.to_latex(conf),
            Type::Nat(n) => n.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Forall(forall) => forall.to_latex(conf),
            Type::Exists(e) => e.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
        }
    }
}
