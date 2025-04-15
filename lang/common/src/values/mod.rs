use crate::{errors::ErrorKind, terms::Term, types::Type};
use std::{any::Any, fmt};

pub trait Value<T>
where
    T: Term,
    Self: Any + Sized + Clone + fmt::Display + fmt::Debug,
{
    type Term: Term + From<Self> + Into<T>;

    fn into_lambda<Ty>(self) -> Result<Lambda<T, Ty>, ErrorKind>
    where
        Ty: Type,
    {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_true(self) -> Result<True, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "True".to_owned(),
        })
    }

    fn into_false(self) -> Result<False, ErrorKind> {
        Err(ErrorKind::ValueMismatch {
            found: self.to_string(),
            expected: "False".to_owned(),
        })
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
pub use unit::Unit;
pub use variant::Variant;
