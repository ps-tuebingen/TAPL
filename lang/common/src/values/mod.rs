use crate::terms::Term;
use std::fmt;

pub trait Value
where
    Self: Clone + fmt::Display + fmt::Debug + PartialEq + Eq,
{
    type Term: Term + From<Self>;

    fn into_term(self) -> Self::Term {
        self.into()
    }
}

pub mod cons;
pub mod exception;
pub mod fls;
pub mod fold;
pub mod lambda;
pub mod lambdasub;
pub mod left;
pub mod loc;
pub mod nil;
pub mod nothing;
pub mod num;
pub mod pack;
pub mod pair;
pub mod raise;
pub mod record;
pub mod right;
pub mod something;
pub mod tru;
pub mod tuple;
pub mod tylambda;
pub mod tylambdasub;
pub mod unit;
pub mod variant;

pub use cons::Cons;
pub use exception::Exception;
pub use fls::False;
pub use fold::Fold;
pub use lambda::Lambda;
pub use lambdasub::LambdaSub;
pub use left::Left;
pub use loc::Loc;
pub use nil::Nil;
pub use nothing::Nothing;
pub use num::Num;
pub use pack::Pack;
pub use pair::Pair;
pub use raise::Raise;
pub use record::Record;
pub use right::Right;
pub use something::Something;
pub use tru::True;
pub use tuple::Tuple;
pub use tylambda::TyLambda;
pub use tylambdasub::TyLambdaSub;
pub use unit::Unit;
pub use variant::Variant;
