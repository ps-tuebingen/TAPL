pub const REFNAT: &str = include_str!("../../../../examples/references/refnat/refnat.ref");

pub const INCC: &str = include_str!("../../../../examples/references/incc/incc.ref");

pub const REFASSIGN: &str = include_str!("../../../../examples/references/refassign/refassign.ref");

pub const DOUBLEREF: &str = include_str!("../../../../examples/references/doubleref/doubleref.ref");

pub const NATARRAY: &str = include_str!("../../../../examples/references/natarray/natarray.ref");

pub fn references_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Refnat", REFNAT),
        ("Incc", INCC),
        ("Refassign", REFASSIGN),
        ("Doubleref", DOUBLEREF),
        ("Natarray", NATARRAY),
    ]
}
