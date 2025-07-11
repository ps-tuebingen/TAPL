use std::fmt;

pub trait Term: fmt::Display + fmt::Debug + Clone + PartialEq + Eq {}

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
pub mod untyped_lambda;
pub mod variable;
pub mod variant;
pub mod variantcase;

pub use app::App;
pub use ascribe::Ascribe;
pub use assign::Assign;
pub use cast::Cast;
pub use cons::Cons;
pub use dereft::Deref;
pub use exception::Exception;
pub use fix::Fix;
pub use fls::False;
pub use fold::Fold;
pub use fst::Fst;
pub use head::Head;
pub use ift::If;
pub use isnil::IsNil;
pub use iszero::IsZero;
pub use lambda::Lambda;
pub use lambdasub::LambdaSub;
pub use left::Left;
pub use lett::Let;
pub use listcase::ListCase;
pub use loc::Loc;
pub use nil::Nil;
pub use nothing::Nothing;
pub use num::Num;
pub use pack::Pack;
pub use pair::Pair;
pub use pred::Pred;
pub use projection::Projection;
pub use raise::Raise;
pub use record::Record;
pub use recordproj::RecordProj;
pub use reft::Ref;
pub use right::Right;
pub use snd::Snd;
pub use somecase::SomeCase;
pub use something::Something;
pub use succ::Succ;
pub use sumcase::SumCase;
pub use tail::Tail;
pub use tru::True;
pub use tryt::Try;
pub use tryval::TryWithVal;
pub use tuple::Tuple;
pub use tyapp::TyApp;
pub use tylambda::TyLambda;
pub use unfold::Unfold;
pub use unit::Unit;
pub use unpack::Unpack;
pub use untyped_lambda::UntypedLambda;
pub use variable::Variable;
pub use variant::Variant;
pub use variantcase::VariantCase;
