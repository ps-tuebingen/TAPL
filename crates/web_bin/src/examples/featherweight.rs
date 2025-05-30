pub const PAIR: &str = include_str!("../../../../examples/featherweight/pair/pair.java");

pub const LIST: &str = include_str!("../../../../examples/featherweight/list/list.java");

pub fn featherweight_all() -> Vec<(&'static str,&'static str)> { 
    vec![
        ("pair",PAIR), 
("list",LIST), 

    ]
}
