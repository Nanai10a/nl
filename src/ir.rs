use core::fmt::Debug;

pub trait Expr: Debug {}

impl Expr for Int {}
pub struct Int {
    pub val: usize,
}

impl Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(int: {})", self.val)
    }
}

impl Expr for Add {}
pub struct Add {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Debug for Add {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?} + {:?})", self.lhs, self.rhs)
    }
}

impl Expr for Sub {}
pub struct Sub {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Debug for Sub {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?} - {:?})", self.lhs, self.rhs)
    }
}

impl Expr for Mul {}
pub struct Mul {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Debug for Mul {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?} * {:?})", self.lhs, self.rhs)
    }
}

impl Expr for Div {}
pub struct Div {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Debug for Div {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?} / {:?})", self.lhs, self.rhs)
    }
}

impl Expr for Mdl {}
pub struct Mdl {
    pub lhs: Box<dyn Expr>,
    pub rhs: Box<dyn Expr>,
}

impl Debug for Mdl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?} % {:?})", self.lhs, self.rhs)
    }
}
