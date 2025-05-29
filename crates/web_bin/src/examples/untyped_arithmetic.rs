pub const IF_NESTED: &str = include_str!("../../../../examples/untyped_arithmetic/if_nested/if_nested.arith");

pub const ISZZ: &str = include_str!("../../../../examples/untyped_arithmetic/iszz/iszz.arith");

pub const IFTHENELSE: &str = include_str!("../../../../examples/untyped_arithmetic/ifthenelse/ifthenelse.arith");

pub fn untyped_arithmetic_all() -> Vec<&'static str> { 
    vec![
        IF_NESTED, ISZZ, IFTHENELSE, 
    ]
}
