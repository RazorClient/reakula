macro_rules! impl_safe_arith {
    ($type: ident, $rhs_ty: ident) => {
        impl safe_arith::SafeArith<$rhs_ty> for $type {
            const ZERO: Self = $type::new(0);
            const ONE: Self = $type::new(1);

            fn safe_add(&self, other: $rhs_ty) -> safe_arith::Result<Self> {
                self.value
                    .checked_add(other.into())
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::Overflow)
            }

            fn safe_sub(&self, other: $rhs_ty) -> safe_arith::Result<Self> {
                self.value
                    .checked_sub(other.into())
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::Overflow)
            }

            fn safe_mul(&self, other: $rhs_ty) -> safe_arith::Result<Self> {
                self.value
                    .checked_mul(other.into())
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::Overflow)
            }

            fn safe_div(&self, other: $rhs_ty) -> safe_arith::Result<Self> {
                self.value
                    .checked_div(other.into())
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::DivisionByZero)
            }

            fn safe_rem(&self, other: $rhs_ty) -> safe_arith::Result<Self> {
                self.value
                    .checked_rem(other.into())
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::DivisionByZero)
            }

            fn safe_shl(&self, other: u32) -> safe_arith::Result<Self> {
                self.value
                    .checked_shl(other)
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::Overflow)
            }

            fn safe_shr(&self, other: u32) -> safe_arith::Result<Self> {
                self.value
                    .checked_shr(other)
                    .map(Self::new)
                    .ok_or(safe_arith::ArithError::Overflow)
            }
        }
    };
}
pub(crate) use impl_safe_arith;