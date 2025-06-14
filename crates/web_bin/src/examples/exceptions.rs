pub const LAMBDAERR: &str = include_str!("../../../../examples/exceptions/lambdaerr/lambdaerr.ex");

pub const TRYRAISE: &str = include_str!("../../../../examples/exceptions/tryraise/tryraise.ex");

pub const TRYERR: &str = include_str!("../../../../examples/exceptions/tryerr/tryerr.ex");

pub fn exceptions_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("lambdaerr",LAMBDAERR), 
         ("tryraise",TRYRAISE), 
         ("tryerr",TRYERR), 

    ]
}
