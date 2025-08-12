pub const PROCESS: &str = include_str!("../../../../examples/recursive/process/process.rec");

pub const TAIL: &str = include_str!("../../../../examples/recursive/tail/tail.rec");

pub const UPFROM0: &str = include_str!("../../../../examples/recursive/upfrom0/upfrom0.rec");

pub const AP: &str = include_str!("../../../../examples/recursive/ap/ap.rec");

pub const HEAD: &str = include_str!("../../../../examples/recursive/head/head.rec");

pub const ISNIL: &str = include_str!("../../../../examples/recursive/isnil/isnil.rec");

pub const HUNGRY: &str = include_str!("../../../../examples/recursive/hungry/hungry.rec");

pub const STREAMHEAD: &str = include_str!("../../../../examples/recursive/streamhead/streamhead.rec");

pub const LAM: &str = include_str!("../../../../examples/recursive/lam/lam.rec");

pub const SEND: &str = include_str!("../../../../examples/recursive/send/send.rec");

pub const NIL: &str = include_str!("../../../../examples/recursive/nil/nil.rec");

pub const STREAMTAIL: &str = include_str!("../../../../examples/recursive/streamtail/streamtail.rec");

pub const PLUS: &str = include_str!("../../../../examples/recursive/plus/plus.rec");

pub const FIXD: &str = include_str!("../../../../examples/recursive/fixd/fixd.rec");

pub const COUNTER: &str = include_str!("../../../../examples/recursive/counter/counter.rec");

pub const CONS: &str = include_str!("../../../../examples/recursive/cons/cons.rec");

pub const SUMLIST: &str = include_str!("../../../../examples/recursive/sumlist/sumlist.rec");

pub const CURR: &str = include_str!("../../../../examples/recursive/curr/curr.rec");

pub fn recursive_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Process", PROCESS),
        ("Tail", TAIL),
        ("Upfrom0", UPFROM0),
        ("Ap", AP),
        ("Head", HEAD),
        ("Isnil", ISNIL),
        ("Hungry", HUNGRY),
        ("Streamhead", STREAMHEAD),
        ("Lam", LAM),
        ("Send", SEND),
        ("Nil", NIL),
        ("Streamtail", STREAMTAIL),
        ("Plus", PLUS),
        ("Fixd", FIXD),
        ("Counter", COUNTER),
        ("Cons", CONS),
        ("Sumlist", SUMLIST),
        ("Curr", CURR),
    ]
}
