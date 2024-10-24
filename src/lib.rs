//! This crate provides "change min(max)" implementation in Rust.
//!
//! ## Example
//!
//! ```
//! use min_max_assign::*;
//!
//! let mut i = 100;
//! i.min_assign(10);
//! assert_eq!(i, 10);
//!
//! let mut f = 10.0;
//! f.max_assign(100.0);
//! assert_eq!(f, 100.0)
//! ```

/// Provides "change min" operation
pub trait MinAssign<RHS = Self> {
    fn min_assign(&mut self, rhs: RHS);
}

macro_rules! min_assign_impl {
    ($($t: ty)+) => {$(
        impl MinAssign for $t {
            #[inline]
            fn min_assign(&mut self, other: $t) {
                if *self > other {
                    *self = other
                }
            }
        }
    )+};
}

min_assign_impl!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 );

/// Provides "change max" operation
pub trait MaxAssign<RHS = Self> {
    fn max_assign(&mut self, rhs: RHS);
}

macro_rules! min_assign_impl {
    ($($t: ty)+) => {$(
        impl MaxAssign for $t {
            #[inline]
            fn max_assign(&mut self, other: $t) {
                if *self < other {
                    *self = other
                }
            }
        }
    )+};
}

min_assign_impl!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 );

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_min() {
        let mut a = 100;
        a.min_assign(10);
        assert_eq!(a, 10);
    }

    #[test]
    fn not_update_min() {
        let mut a = 10;
        a.min_assign(100);
        assert_eq!(a, 10);
    }

    #[test]
    fn update_max() {
        let mut a = 10;
        a.max_assign(100);
        assert_eq!(a, 100)
    }

    #[test]
    fn not_update_max() {
        let mut a = 100;
        a.max_assign(10);
        assert_eq!(a, 100)
    }
}