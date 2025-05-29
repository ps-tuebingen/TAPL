pub const LAMBDAERR: &str = include_str!("../../../../examples/exceptions/lambdaerr/lambdaerr.ex");

pub const TRYRAISE: &str = include_str!("../../../../examples/exceptions/tryraise/tryraise.ex");

pub const TRYERR: &str = include_str!("../../../../examples/exceptions/tryerr/tryerr.ex");

pub fn exceptions_all() -> Vec<&'static str> { 
    vec![
        LAMBDAERR, TRYRAISE, TRYERR, 
    ]
}
