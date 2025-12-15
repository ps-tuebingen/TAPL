//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const LAMBDAERR: &str = include_str!("../../../../examples/exceptions/lambdaerr/lambdaerr.ex");

pub const TRYERR: &str = include_str!("../../../../examples/exceptions/tryerr/tryerr.ex");

pub const TRYRAISE: &str = include_str!("../../../../examples/exceptions/tryraise/tryraise.ex");

pub fn exceptions_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Lambdaerr", LAMBDAERR),
        ("Tryerr", TRYERR),
        ("Tryraise", TRYRAISE),
    ]
}
