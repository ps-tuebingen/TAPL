//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const IDNAT: &str = include_str!("../../../../examples/lambda_omega/idnat/idnat.lamo");

pub const ID: &str = include_str!("../../../../examples/lambda_omega/id/id.lamo");

pub const PAIRNATBOOL: &str = include_str!("../../../../examples/lambda_omega/pairnatbool/pairnatbool.lamo");

pub const PAIR: &str = include_str!("../../../../examples/lambda_omega/pair/pair.lamo");

pub fn lambda_omega_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Idnat", IDNAT),
        ("Id", ID),
        ("Pairnatbool", PAIRNATBOOL),
        ("Pair", PAIR),
    ]
}
