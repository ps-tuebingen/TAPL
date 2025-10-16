pub mod definition;
pub mod program;
pub mod terms;
pub mod types;

mod kindcheck;
mod normalize;
mod subtypecheck;
mod typecheck;
pub use kindcheck::Kindcheck;
pub use normalize::Normalize;
pub use subtypecheck::Subtypecheck;
pub use typecheck::Typecheck;
