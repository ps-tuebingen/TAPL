use std::collections::HashMap;
pub const LAMBDAERR: &str = include_str!("../../../../examples/exceptions/lambdaerr/lambdaerr.ex");

pub const TRYRAISE: &str = include_str!("../../../../examples/exceptions/tryraise/tryraise.ex");

pub const TRYERR: &str = include_str!("../../../../examples/exceptions/tryerr/tryerr.ex");

pub fn exceptions_all() -> HashMap<&'static str,&'static str> { 
    HashMap::from([
        ("LAMBDAERR",LAMBDAERR), 
("TRYRAISE",TRYRAISE), 
("TRYERR",TRYERR), 

    ])
}
