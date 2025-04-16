use std::fmt;

pub trait Type
where
    Self: fmt::Display + fmt::Debug + Clone + PartialEq + Eq,
{
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
