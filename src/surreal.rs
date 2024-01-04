use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, Clone, Default, PartialOrd, PartialEq)]
pub struct Surreal {
    left: Option<Rc<Surreal>>,
    right: Option<Rc<Surreal>>,
}

impl Surreal {
    pub fn new(left: Option<Rc<Surreal>>, right: Option<Rc<Surreal>>) -> Surreal {
        Surreal { left, right }
    }

    pub fn zero() -> Surreal {
        Surreal {
            left: None,
            right: None,
        }
    }

    fn le(&self, other: &Surreal) -> bool {
        !self.left.iter().any(|l| other.le(l.as_ref()))
            && !other.right.iter().any(|r| r.as_ref().le(self))
    }

    fn ge(&self, other: &Surreal) -> bool {
        other.le(self)
    }

    fn lt(&self, other: &Surreal) -> bool {
        !self.ge(other)
    }

    fn gt(&self, other: &Surreal) -> bool {
        !self.le(other)
    }
}

impl Eq for Surreal {}

impl Ord for Surreal {
    fn cmp(&self, other: &Surreal) -> Ordering {
        if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
