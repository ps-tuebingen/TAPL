use crate::errors::ErrorKind;
use std::{any::Any, fmt};

pub trait Type
where
    Self: Any + fmt::Display + fmt::Debug + Clone + PartialEq + Eq,
{
    fn into_fun<Ty>(self) -> Result<Fun<Ty>, ErrorKind>
    where
        Ty: Type,
    {
        let boxed = Box::new(self) as Box<dyn Any>;
        boxed.try_into()
    }

    fn into_nat(self) -> Result<Nat, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Nat".to_owned(),
        })
    }

    fn into_bool(self) -> Result<Bool, ErrorKind> {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Bool".to_owned(),
        })
    }
}

pub mod bool;
pub mod bot;
pub mod exists;
pub mod forall;
pub mod fun;
pub mod list;
pub mod mu;
pub mod nat;
pub mod opapp;
pub mod oplambda;
pub mod optional;
pub mod product;
pub mod record;
pub mod reference;
pub mod sink;
pub mod source;
pub mod sum;
pub mod top;
pub mod tuple;
pub mod unit;
pub mod variable;
pub mod variant;

pub use bool::Bool;
pub use bot::Bot;
pub use exists::Exists;
pub use forall::Forall;
pub use fun::Fun;
pub use list::List;
pub use mu::Mu;
pub use nat::Nat;
pub use opapp::OpApp;
pub use oplambda::OpLambda;
pub use optional::Optional;
pub use product::Product;
pub use record::Record;
pub use reference::Reference;
pub use sink::Sink;
pub use source::Source;
pub use sum::Sum;
pub use top::Top;
pub use tuple::Tuple;
pub use unit::Unit;
pub use variable::TypeVariable;
pub use variant::Variant;
