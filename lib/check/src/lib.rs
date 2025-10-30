mod definition;
mod program;
mod terms;
mod types;
mod untyped;

mod kindcheck;
mod normalize;
mod subtypecheck;
mod typecheck;
pub use kindcheck::Kindcheck;
pub use normalize::Normalize;
pub use subtypecheck::Subtypecheck;
pub use typecheck::Typecheck;
