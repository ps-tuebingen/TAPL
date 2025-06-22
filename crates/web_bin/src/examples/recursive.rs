pub const HEAD: &str = include_str!("../../../../examples/recursive/head/head.rec");

pub const PROCESS: &str = include_str!("../../../../examples/recursive/process/process.rec");

pub const CURR: &str = include_str!("../../../../examples/recursive/curr/curr.rec");

pub const LAM: &str = include_str!("../../../../examples/recursive/lam/lam.rec");

pub const STREAMHEAD: &str = include_str!("../../../../examples/recursive/streamhead/streamhead.rec");

pub const SUMLIST: &str = include_str!("../../../../examples/recursive/sumlist/sumlist.rec");

pub const FIXD: &str = include_str!("../../../../examples/recursive/fixd/fixd.rec");

pub const NIL: &str = include_str!("../../../../examples/recursive/nil/nil.rec");

pub const CONS: &str = include_str!("../../../../examples/recursive/cons/cons.rec");

pub const UPFROM0: &str = include_str!("../../../../examples/recursive/upfrom0/upfrom0.rec");

pub const STREAMTAIL: &str = include_str!("../../../../examples/recursive/streamtail/streamtail.rec");

pub const COUNTER: &str = include_str!("../../../../examples/recursive/counter/counter.rec");

pub const AP: &str = include_str!("../../../../examples/recursive/ap/ap.rec");

pub const ISNIL: &str = include_str!("../../../../examples/recursive/isnil/isnil.rec");

pub const HUNGRY: &str = include_str!("../../../../examples/recursive/hungry/hungry.rec");

pub const SEND: &str = include_str!("../../../../examples/recursive/send/send.rec");

pub const PLUS: &str = include_str!("../../../../examples/recursive/plus/plus.rec");

pub const TAIL: &str = include_str!("../../../../examples/recursive/tail/tail.rec");

pub fn recursive_all() -> Vec<(&'static str,&'static str)> { 
    vec![
         ("Head",HEAD), 
         ("Process",PROCESS), 
         ("Curr",CURR), 
         ("Lam",LAM), 
         ("StreamHead",STREAMHEAD), 
         ("SumList",SUMLIST), 
         ("FixD",FIXD), 
         ("Nil",NIL), 
         ("Cons",CONS), 
         ("UpFromZero",UPFROM0), 
         ("StreamTail",STREAMTAIL), 
         ("Counter",COUNTER), 
         ("Ap",AP), 
         ("IsNil",ISNIL), 
         ("Hungry",HUNGRY), 
         ("Send",SEND), 
         ("Plus",PLUS), 
         ("Tail",TAIL), 

    ]
}
