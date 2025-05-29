use std::collections::HashMap;
pub const IF_NESTED: &str = include_str!("../../../../examples/typed_arithmetic/if_nested/if_nested.arith");

pub const ISZZ: &str = include_str!("../../../../examples/typed_arithmetic/iszz/iszz.arith");

pub const IFTHENELSE: &str = include_str!("../../../../examples/typed_arithmetic/ifthenelse/ifthenelse.arith");

pub fn typed_arithmetic_all() -> HashMap<&'static str,&'static str> { 
    HashMap::from([
        ("IF_NESTED",IF_NESTED), 
("ISZZ",ISZZ), 
("IFTHENELSE",IFTHENELSE), 

    ])
}
