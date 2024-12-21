mod iter;

pub use iter::*;

pub trait SafeArith<Rhs = Self>: Sized + Copy {
    const ZERO: Self;
    const ONE: Self;

    /// Safe variant of `+` that guards against overflow.
    fn safe_add(&self, other: Rhs) -> Result<Self>;

    /// Safe variant of `-` that guards against overflow.
    fn safe_sub(&self, other: Rhs) -> Result<Self>;

    /// Safe variant of `*` that guards against overflow.
    fn safe_mul(&self, other: Rhs) -> Result<Self>;

    /// Safe variant of `/` that guards against division by 0.
    fn safe_div(&self, other: Rhs) -> Result<Self>;

    /// Safe variant of `%` that guards against division by 0.
    fn safe_rem(&self, other: Rhs) -> Result<Self>;

    /// Safe variant of `<<` that guards against overflow.
    fn safe_shl(&self, other: u32) -> Result<Self>;

    /// Safe variant of `>>` that guards against overflow.
    fn safe_shr(&self, other: u32) -> Result<Self>;

    /// Safe variant of `+=` that guards against overflow.
    fn safe_add_assign(&mut self, other: Rhs) -> Result<()> {
        *self = self.safe_add(other)?;
        Ok(())
    }

    /// Safe variant of `-=` that guards against overflow.
    fn safe_sub_assign(&mut self, other: Rhs) -> Result<()> {
        *self = self.safe_sub(other)?;
        Ok(())
    }

    /// Safe variant of `*=` that guards against overflow.
    fn safe_mul_assign(&mut self, other: Rhs) -> Result<()> {
        *self = self.safe_mul(other)?;
        Ok(())
    }

    /// Safe variant of `/=` that guards against division by 0.
    fn safe_div_assign(&mut self, other: Rhs) -> Result<()> {
        *self = self.safe_div(other)?;
        Ok(())
    }

    /// Safe variant of `%=` that guards against division by 0.
    fn safe_rem_assign(&mut self, other: Rhs) -> Result<()> {
        *self = self.safe_rem(other)?;
        Ok(())
    }

    /// Safe variant of `<<=` that guards against overflow.
    fn safe_shl_assign(&mut self, other: u32) -> Result<()> {
        *self = self.safe_shl(other)?;
        Ok(())
    }

    /// Safe variant of `>>=` that guards against overflow.
    fn safe_shr_assign(&mut self, other: u32) -> Result<()> {
        *self = self.safe_shr(other)?;
        Ok(())
    }
}

macro_rules! impl_safe_arith {
    ($typ:ty) => {
        impl SafeArith for $typ {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline]
            fn safe_add(&self, other: Self) -> Result<Self> {
                self.checked_add(other).ok_or(ArithError::Overflow)
            }

            #[inline]
            fn safe_sub(&self, other: Self) -> Result<Self> {
                self.checked_sub(other).ok_or(ArithError::Overflow)
            }

            #[inline]
            fn safe_mul(&self, other: Self) -> Result<Self> {
                self.checked_mul(other).ok_or(ArithError::Overflow)
            }

            #[inline]
            fn safe_div(&self, other: Self) -> Result<Self> {
                self.checked_div(other).ok_or(ArithError::DivisionByZero)
            }

            #[inline]
            fn safe_rem(&self, other: Self) -> Result<Self> {
                self.checked_rem(other).ok_or(ArithError::DivisionByZero)
            }

            #[inline]
            fn safe_shl(&self, other: u32) -> Result<Self> {
                self.checked_shl(other).ok_or(ArithError::Overflow)
            }

            #[inline]
            fn safe_shr(&self, other: u32) -> Result<Self> {
                self.checked_shr(other).ok_or(ArithError::Overflow)
            }
        }
    };
}

impl_safe_arith!(u8);
impl_safe_arith!(u16);
impl_safe_arith!(u32);
impl_safe_arith!(u64);
impl_safe_arith!(usize);
impl_safe_arith!(i8);
impl_safe_arith!(i16);
impl_safe_arith!(i32);
impl_safe_arith!(i64);
impl_safe_arith!(isize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        // Test with u8
        let x = 10u8;
        let y = 5u8;
        assert_eq!(x.safe_add(y), Ok(x + y));
        assert_eq!(x.safe_sub(y), Ok(x - y));
        assert_eq!(x.safe_mul(y), Ok(x * y));
        assert_eq!(x.safe_div(y), Ok(x / y));
        assert_eq!(x.safe_rem(y), Ok(x % y));

        assert_eq!(x.safe_shl(2), Ok(x << 2));
        assert_eq!(x.safe_shr(1), Ok(x >> 1));

        // Test with u16
        let x = 100u16;
        let y = 25u16;
        assert_eq!(x.safe_add(y), Ok(x + y));
        assert_eq!(x.safe_sub(y), Ok(x - y));
        assert_eq!(x.safe_mul(y), Ok(x * y));
        assert_eq!(x.safe_div(y), Ok(x / y));
        assert_eq!(x.safe_rem(y), Ok(x % y));

        assert_eq!(x.safe_shl(3), Ok(x << 3));
        assert_eq!(x.safe_shr(2), Ok(x >> 2));
    }

    #[test]
    fn mutate() {
        // Test with edge case values
        let mut x = 1u8;
        x.safe_add_assign(254).unwrap();
        assert_eq!(x, 255);
        x.safe_sub_assign(255).unwrap();
        assert_eq!(x, 0);
        x.safe_add_assign(1).unwrap();
        x.safe_shl_assign(7).unwrap();
        assert_eq!(x, 128);
        x.safe_shr_assign(7).unwrap();
        assert_eq!(x, 1);

        // Test with larger integer types
        let mut y = 100u16;
        y.safe_mul_assign(2).unwrap();
        assert_eq!(y, 200);
        y.safe_div_assign(4).unwrap();
        assert_eq!(y, 50);
        y.safe_add_assign(1000).unwrap();
        assert_eq!(y, 1050);
    }

    #[test]
    fn errors() {
        // Overflow and underflow for u32
        assert!(u32::MAX.safe_add(1).is_err());
        assert!(u32::MIN.safe_sub(1).is_err());
        assert!(u32::MAX.safe_mul(2).is_err());

        // Division by zero
        assert!(10u32.safe_div(0).is_err());
        assert!(10u32.safe_rem(0).is_err());

        // Shift overflow
        assert!(u32::MAX.safe_shl(33).is_err());
        assert!(u32::MAX.safe_shr(33).is_err());

        // Edge cases for smaller types
        assert!(u8::MAX.safe_add(1).is_err());
        assert!(u8::MIN.safe_sub(1).is_err());
        assert!(u8::MAX.safe_mul(2).is_err());

        // Shifting too far
        assert!(u8::MAX.safe_shl(9).is_err());
        assert!(u8::MAX.safe_shr(9).is_err());
    }
}
