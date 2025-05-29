use std::collections::HashMap;

pub mod typed_arithmetic;
pub use typed_arithmetic::typed_arithmetic_all;

pub mod subtypes;
pub use subtypes::subtypes_all;

pub mod bounded_quantification;
pub use bounded_quantification::bounded_quantification_all;

pub mod exceptions;
pub use exceptions::exceptions_all;

pub mod featherweight;
pub use featherweight::featherweight_all;

pub mod references;
pub use references::references_all;

pub mod lambda_omega;
pub use lambda_omega::lambda_omega_all;

pub mod inference;
pub use inference::inference_all;

pub mod untyped_arithmetic;
pub use untyped_arithmetic::untyped_arithmetic_all;

pub mod untyped_lambda;
pub use untyped_lambda::untyped_lambda_all;

pub mod stlc;
pub use stlc::stlc_all;

pub mod system_f;
pub use system_f::system_f_all;

pub mod f_omega_sub;
pub use f_omega_sub::f_omega_sub_all;

pub mod f_omega;
pub use f_omega::f_omega_all;

pub mod existential;
pub use existential::existential_all;

pub mod recursive;
pub use recursive::recursive_all;

pub fn all_examples() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("typed-arithmetic", typed_arithmetic_all()),
        ("subtypes", subtypes_all()),
        ("bounded-quantification", bounded_quantification_all()),
        ("exceptions", exceptions_all()),
        ("featherweight", featherweight_all()),
        ("references", references_all()),
        ("lambda-omega", lambda_omega_all()),
        ("inference", inference_all()),
        ("untyped-arithmetic", untyped_arithmetic_all()),
        ("untyped-lambda", untyped_lambda_all()),
        ("stlc", stlc_all()),
        ("system-f", system_f_all()),
        ("f-omega-sub", f_omega_sub_all()),
        ("f-omega", f_omega_all()),
        ("existential", existential_all()),
        ("recursive", recursive_all()),
    ])
}
