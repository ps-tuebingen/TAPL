pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}
