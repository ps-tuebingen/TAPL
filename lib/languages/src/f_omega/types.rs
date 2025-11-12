use super::FOmega;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{Kindcheck, LangDisplay, LatexFmt, NoSubtypes, Normalize};
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{
        Bool, Exists, Forall, Fun, Nat, OpApp, OpLambda, Record, Type as TypeTrait, TypeGroup,
        TypeVariable, Unit,
    },
};

#[derive(LatexFmt, LangDisplay, Normalize, Kindcheck, NoSubtypes, Debug, Clone, PartialEq, Eq)]
#[Lang(FOmega)]
pub enum Type {
    Var(TypeVariable<FOmega>),
    Fun(Fun<FOmega>),
    Forall(Forall<FOmega>),
    OpLambda(OpLambda<FOmega>),
    OpApp(OpApp<FOmega>),
    Exists(Exists<FOmega>),
    Record(Record<FOmega>),
    Bool(Bool<FOmega>),
    Unit(Unit<FOmega>),
    Nat(Nat<FOmega>),
}

impl TypeTrait for Type {}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<FOmega>::rule(),
            Fun::<FOmega>::rule(),
            Forall::<FOmega>::rule(),
            OpLambda::<FOmega>::rule(),
            OpApp::<FOmega>::rule(),
            Exists::<FOmega>::rule(),
            Record::<FOmega>::rule(),
            Bool::<FOmega>::rule(),
            Unit::<FOmega>::rule(),
            Nat::<FOmega>::rule(),
        ])
    }
}

impl TypeGroup for Type {
    type Lang = FOmega;
    fn into_fun(self) -> Result<Fun<FOmega>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<FOmega>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<FOmega>, TypeMismatch> {
        if let Type::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
        }
    }

    fn into_opapp(self) -> Result<OpApp<FOmega>, TypeMismatch> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpApp".to_owned()))
        }
    }

    fn into_exists(self) -> Result<Exists<FOmega>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<FOmega>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<FOmega>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_unit(self) -> Result<Unit<FOmega>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<FOmega>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }
}

impl SubstType for Type {
    type Lang = FOmega;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty).into(),
            Type::Forall(forall) => forall.subst_type(v, ty).into(),
            Type::OpLambda(lam) => lam.subst_type(v, ty).into(),
            Type::OpApp(app) => app.subst_type(v, ty).into(),
            Type::Exists(ex) => ex.subst_type(v, ty).into(),
            Type::Record(rec) => rec.subst_type(v, ty).into(),
            Type::Bool(b) => b.subst_type(v, ty).into(),
            Type::Unit(u) => u.subst_type(v, ty).into(),
            Type::Nat(nat) => nat.subst_type(v, ty).into(),
        }
    }
}

impl From<TypeVariable<FOmega>> for Type {
    fn from(var: TypeVariable<FOmega>) -> Type {
        Type::Var(var)
    }
}
impl From<OpLambda<FOmega>> for Type {
    fn from(oplam: OpLambda<FOmega>) -> Type {
        Type::OpLambda(oplam)
    }
}
impl From<Fun<FOmega>> for Type {
    fn from(fun: Fun<FOmega>) -> Type {
        Type::Fun(fun)
    }
}
impl From<OpApp<FOmega>> for Type {
    fn from(opapp: OpApp<FOmega>) -> Type {
        Type::OpApp(opapp)
    }
}
impl From<Forall<FOmega>> for Type {
    fn from(forall: Forall<FOmega>) -> Type {
        Type::Forall(forall)
    }
}
impl From<Exists<FOmega>> for Type {
    fn from(exists: Exists<FOmega>) -> Type {
        Type::Exists(exists)
    }
}
impl From<Record<FOmega>> for Type {
    fn from(rec: Record<FOmega>) -> Type {
        Type::Record(rec)
    }
}
impl From<Bool<FOmega>> for Type {
    fn from(b: Bool<FOmega>) -> Type {
        Type::Bool(b)
    }
}
impl From<Unit<FOmega>> for Type {
    fn from(u: Unit<FOmega>) -> Type {
        Type::Unit(u)
    }
}
impl From<Nat<FOmega>> for Type {
    fn from(n: Nat<FOmega>) -> Type {
        Type::Nat(n)
    }
}
