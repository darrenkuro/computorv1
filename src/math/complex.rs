use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display + PartialEq + Default + PartialOrd + Clone + std::ops::Neg<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let zero = T::default(); // 0 for numeric-like types

        if self.im == zero {
            // purely real
            write!(f, "{}", self.re)
        } else if self.re == zero {
            // purely imaginary
            write!(f, "{}i", self.im)
        } else if self.im > zero {
            // positive imaginary part
            write!(f, "{} + {}i", self.re, self.im)
        } else {
            // negative imaginary part → print minus explicitly
            write!(f, "{} - {}i", self.re, -self.im.clone())
        }
    }
}
