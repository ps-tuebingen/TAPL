pub const ID: &str = include_str!("../../../../examples/lambda-omega/id/id.lamo");

pub const PAIRNATBOOL: &str = include_str!("../../../../examples/lambda-omega/pairnatbool/pairnatbool.lamo");

pub const PAIR: &str = include_str!("../../../../examples/lambda-omega/pair/pair.lamo");

pub const IDNAT: &str = include_str!("../../../../examples/lambda-omega/idnat/idnat.lamo");

pub fn lambda-omega_all() -> Vec<&'static str> { 
    vec![
        ID,
        PAIRNATBOOL,
        PAIR,
        IDNAT,
    ]
}
