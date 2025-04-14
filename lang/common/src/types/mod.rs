use std::fmt;

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

pub trait Type: fmt::Display + fmt::Debug + Clone {}
