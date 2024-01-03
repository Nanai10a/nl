pub trait Expr {}

impl Expr for Int {}
pub struct Int {
    pub val: usize,
}

impl Expr for Add {}
pub struct Add {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Expr for Sub {}
pub struct Sub {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Expr for Mul {}
pub struct Mul {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Expr for Div {}
pub struct Div {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Expr for Mdl {}
pub struct Mdl {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}
