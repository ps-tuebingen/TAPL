use common::errors::{TypeKind, TypeMismatch};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{
        Bool, Exists, Forall, Fun, Nat, OpApp, OpLambda, Record, Type as TypeTrait, TypeGroup,
        TypeVariable, Unit,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Fun(Fun<Type>),
    Forall(Forall<Type>),
    OpLambda(OpLambda<Type>),
    OpApp(OpApp<Type>),
    Exists(Exists<Type>),
    Record(Record<Type>),
    Bool(Bool<Type>),
    Unit(Unit<Type>),
    Nat(Nat<Type>),
}

impl TypeTrait for Type {
    fn knd(&self) -> TypeKind {
        match self {
            Type::Var(t) => t.knd(),
            Type::Fun(t) => t.knd(),
            Type::Forall(t) => t.knd(),
            Type::OpLambda(t) => t.knd(),
            Type::OpApp(t) => t.knd(),
            Type::Exists(t) => t.knd(),
            Type::Record(t) => t.knd(),
            Type::Bool(t) => t.knd(),
            Type::Unit(t) => t.knd(),
            Type::Nat(t) => t.knd(),
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<Type>::rule(),
            Fun::<Type>::rule(),
            Forall::<Type>::rule(),
            OpLambda::<Type>::rule(),
            OpApp::<Type>::rule(),
            Exists::<Type>::rule(),
            Record::<Type>::rule(),
            Bool::<Type>::rule(),
            Unit::<Type>::rule(),
            Nat::<Type>::rule(),
        ])
    }
}

impl TypeGroup for Type {
    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Function))
        }
    }

    fn into_forall(self) -> Result<Forall<Type>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Universal))
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<Type>, TypeMismatch> {
        if let Type::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::OpLambda))
        }
    }

    fn into_opapp(self) -> Result<OpApp<Type>, TypeMismatch> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::OpApp))
        }
    }

    fn into_exists(self) -> Result<Exists<Type>, TypeMismatch> {
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

    fn into_bool(self) -> Result<Bool<Type>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Bool))
        }
    }

    fn into_unit(self) -> Result<Unit<Type>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Unit))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
            Type::OpLambda(lam) => lam.subst_type(v, ty),
            Type::OpApp(app) => app.subst_type(v, ty),
            Type::Exists(ex) => ex.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
            Type::Bool(b) => b.subst_type(v, ty),
            Type::Unit(u) => u.subst_type(v, ty),
            Type::Nat(nat) => nat.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::OpLambda(lambda) => lambda.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
            Type::Exists(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(var) => var.to_latex(conf),
            Type::OpLambda(lambda) => lambda.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::OpApp(app) => app.to_latex(conf),
            Type::Forall(forall) => forall.to_latex(conf),
            Type::Exists(ex) => ex.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
            Type::Unit(u) => u.to_latex(conf),
            Type::Nat(nat) => nat.to_latex(conf),
        }
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(var: TypeVariable<Type>) -> Type {
        Type::Var(var)
    }
}
impl From<OpLambda<Type>> for Type {
    fn from(oplam: OpLambda<Type>) -> Type {
        Type::OpLambda(oplam)
    }
}
impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}
impl From<OpApp<Type>> for Type {
    fn from(opapp: OpApp<Type>) -> Type {
        Type::OpApp(opapp)
    }
}
impl From<Forall<Type>> for Type {
    fn from(forall: Forall<Type>) -> Type {
        Type::Forall(forall)
    }
}
impl From<Exists<Type>> for Type {
    fn from(exists: Exists<Type>) -> Type {
        Type::Exists(exists)
    }
}
impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}
impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}
impl From<Unit<Type>> for Type {
    fn from(u: Unit<Type>) -> Type {
        Type::Unit(u)
    }
}
impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}
