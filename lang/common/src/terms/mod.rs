use std::fmt;

pub mod app;
pub mod ascribe;
pub mod assign;
pub mod cast;
pub mod cons;
pub mod dereft;
pub mod exception;
pub mod fix;
pub mod fls;
pub mod fold;
pub mod fst;
pub mod head;
pub mod ift;
pub mod isnil;
pub mod iszero;
pub mod lambda;
pub mod lambdasub;
pub mod left;
pub mod lett;
pub mod listcase;
pub mod loc;
pub mod nil;
pub mod nothing;
pub mod num;
pub mod pack;
pub mod pair;
pub mod pred;
pub mod projection;
pub mod raise;
pub mod record;
pub mod recordproj;
pub mod reft;
pub mod right;
pub mod snd;
pub mod somecase;
pub mod something;
pub mod succ;
pub mod sumcase;
pub mod tail;
pub mod tru;
pub mod tryt;
pub mod tryval;
pub mod tuple;
pub mod tyapp;
pub mod tylambda;
pub mod unfold;
pub mod unit;
pub mod unpack;
pub mod variable;
pub mod variant;
pub mod variantcase;

pub trait Term: fmt::Display + fmt::Debug + Clone {}
