use std::collections::HashMap;
pub const ID: &str = include_str!("../../../../examples/lambda_omega/id/id.lamo");

pub const PAIRNATBOOL: &str = include_str!("../../../../examples/lambda_omega/pairnatbool/pairnatbool.lamo");

pub const PAIR: &str = include_str!("../../../../examples/lambda_omega/pair/pair.lamo");

pub const IDNAT: &str = include_str!("../../../../examples/lambda_omega/idnat/idnat.lamo");

pub fn lambda_omega_all() -> HashMap<&'static str,&'static str> { 
    HashMap::from([
        ("ID",ID), 
("PAIRNATBOOL",PAIRNATBOOL), 
("PAIR",PAIR), 
("IDNAT",IDNAT), 

    ])
}
