use std::cmp::Ordering;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone, Default, PartialOrd, PartialEq)]
pub struct Surreal {
    left: Option<Rc<Surreal>>,
    right: Option<Rc<Surreal>>,
    numerator: i64,
    denominator: u64,
}

impl Surreal {
    pub fn new(left: Option<Rc<Surreal>>, right: Option<Rc<Surreal>>) -> Surreal {
        println!(
            "new({}, {})",
            left.as_ref().map(|l| l.frac()).unwrap_or('∅'.to_string()),
            right.as_ref().map(|r| r.frac()).unwrap_or('∅'.to_string())
        );
        if let Some(l) = left {
            if let Some(r) = right {
                assert!(l.as_ref().lt(&r));
                let denominator = u64::max(l.denominator, r.denominator);
                let l_num = if l.denominator == denominator {
                    l.numerator
                } else {
                    l.numerator * (denominator / l.denominator) as i64
                };
                let r_num = if r.denominator == denominator {
                    r.numerator
                } else {
                    r.numerator * (denominator / r.denominator) as i64
                };
                let numerator = l_num + r_num;
                let reducing = numerator.trailing_zeros();
                Surreal {
                    left: Some(l),
                    right: Some(r),
                    numerator: numerator >> reducing,
                    denominator: denominator >> reducing,
                }
            } else {
                Surreal {
                    left: Some(l.clone()),
                    right: None,
                    numerator: l.numerator + 1,
                    denominator: 1,
                }
            }
        } else if let Some(r) = right {
            Surreal {
                left: None,
                right: Some(r.clone()),
                numerator: r.numerator - 1,
                denominator: 1,
            }
        } else {
            Surreal {
                left: None,
                right: None,
                numerator: 0,
                denominator: 1,
            }
        }
    }

    pub fn zero() -> Surreal {
        Self::new(None, None)
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

    fn frac(&self) -> String {
        if self.denominator == 1 {
            format!("{}", self.numerator)
        } else {
            format!("{}/{}", self.numerator, self.denominator)
        }
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

impl Display for Surreal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}|{}}}={}",
            self.left
                .as_ref()
                .map(|l| l.frac())
                .unwrap_or('∅'.to_string()),
            self.right
                .as_ref()
                .map(|r| r.frac())
                .unwrap_or('∅'.to_string()),
            self.frac()
        )
    }
}
