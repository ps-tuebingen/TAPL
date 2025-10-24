pub mod bounded_quantification;
pub mod exceptions;
pub mod existential;
pub mod f_omega;
pub mod f_omega_sub;
pub mod lambda_omega;
pub mod recursive;
pub mod references;
pub mod stlc;
pub mod subtypes;
pub mod system_f;
pub mod typed_arithmetic;
pub mod untyped_arithmetic;
pub mod untyped_lambda;

pub mod all_languages;
pub use all_languages::AllLanguages;

use syntax::language::Language;

pub use bounded_quantification::BoundedQuantification;
pub use exceptions::Exceptions;
pub use existential::Existential;
pub use f_omega::FOmega;
pub use f_omega_sub::FOmegaSub;
pub use lambda_omega::LambdaOmega;
pub use recursive::Recursive;
pub use references::References;
pub use stlc::Stlc;
pub use subtypes::Subtypes;
pub use system_f::SystemF;
pub use typed_arithmetic::TypedArithmetic;
pub use untyped_arithmetic::UntypedArithmetic;
pub use untyped_lambda::UntypedLambda;
