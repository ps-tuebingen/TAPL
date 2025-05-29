use std::collections::HashMap;
pub const PAIR: &str = include_str!("../../../../examples/featherweight/pair/pair.java");

pub const LIST: &str = include_str!("../../../../examples/featherweight/list/list.java");

pub fn featherweight_all() -> HashMap<&'static str,&'static str> { 
    HashMap::from([
        ("PAIR",PAIR), 
("LIST",LIST), 

    ])
}
