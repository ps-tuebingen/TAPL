use super::Existential;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType};
use syntax::types::{
    Bool, Exists, Fun, Nat, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
};

#[derive(
    SubstType, LatexFmt, LangDisplay, NoNorm, NoKinds, NoSubtypes, Debug, Clone, PartialEq, Eq,
)]
#[Lang(Existential)]
pub enum Type {
    Var(TypeVariable<Existential>),
    Unit(Unit<Existential>),
    Nat(Nat<Existential>),
    Bool(Bool<Existential>),
    Fun(Fun<Existential>),
    Exists(Exists<Existential>),
    Record(Record<Existential>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Existential;
    fn into_unit(self) -> Result<Unit<Existential>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Existential>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Existential>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Existential>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_exists(self) -> Result<Exists<Existential>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<Existential>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<Existential>::rule(),
            Unit::<Existential>::rule(),
            Nat::<Existential>::rule(),
            Bool::<Existential>::rule(),
            Fun::<Existential>::rule(),
            Exists::<Existential>::rule(),
            Record::<Existential>::rule(),
        ])
    }
}

impl From<Exists<Existential>> for Type {
    fn from(ex: Exists<Existential>) -> Type {
        Type::Exists(ex)
    }
}

impl From<TypeVariable<Existential>> for Type {
    fn from(v: TypeVariable<Existential>) -> Type {
        Type::Var(v)
    }
}
impl From<Unit<Existential>> for Type {
    fn from(u: Unit<Existential>) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Existential>> for Type {
    fn from(fun: Fun<Existential>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool<Existential>> for Type {
    fn from(b: Bool<Existential>) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat<Existential>> for Type {
    fn from(n: Nat<Existential>) -> Type {
        Type::Nat(n)
    }
}

impl From<Record<Existential>> for Type {
    fn from(rec: Record<Existential>) -> Type {
        Type::Record(rec)
    }
}
