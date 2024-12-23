use crate::SafeArith;

/// Error representing the failure of an arithmetic operation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ArithError {
    Overflow,
    DivisionByZero,
}

pub type Result<T, E = crate::ArithError> = core::result::Result<T, E>;

/// Extension trait for iterators, providing a safe replacement for `sum`.
pub trait SafeArithIter<T> {
    fn safe_sum(self) -> Result<T>;
}

impl<I, T> SafeArithIter<T> for I
where
    I: Iterator<Item = T> + Sized,
    T: SafeArith,
{
    fn safe_sum(mut self) -> Result<T> {
        self.try_fold(T::ZERO, |acc, x| acc.safe_add(x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ArithError;

    #[test]
    fn empty_sum() {
        let v: Vec<u64> = vec![];
        assert_eq!(v.into_iter().safe_sum(), Ok(0));
    }

    #[test]
    fn unsigned_sum_small() {
        let arr = [500u64, 501, 502, 503, 504, 505, 506];
        assert_eq!(
            arr.iter().copied().safe_sum().unwrap(),
            arr.iter().copied().sum()
        );

        // Additional case with different values
        let arr = [10u64, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(arr.iter().copied().safe_sum().unwrap(), arr.iter().copied().sum());
    }

    #[test]
    fn unsigned_sum_overflow() {
        let v = vec![u64::MAX, 2];
        assert_eq!(v.into_iter().safe_sum(), Err(ArithError::Overflow));

        // Additional edge case with close-to-limit values
        let v = vec![u64::MAX - 1, 2];
        assert_eq!(v.into_iter().safe_sum(), Err(ArithError::Overflow));
    }

    #[test]
    fn signed_sum_small() {
        let v = vec![-2i64, -3i64, -4i64, 4, 3, 2];
        assert_eq!(v.into_iter().safe_sum(), Ok(0));

        // Additional case with alternating signs
        let v = vec![-15i64, 25, -35, 45, -55, 65];
        assert_eq!(v.into_iter().safe_sum(), Ok(30));
    }

    #[test]
    fn signed_sum_overflow_above() {
        let v = vec![1, 2, 3, 4, i16::MAX, 5, 6, 7];
        assert_eq!(v.into_iter().safe_sum(), Err(ArithError::Overflow));

        // Additional edge case with large positive numbers
        let v = vec![i64::MAX - 1, 2];
        assert_eq!(v.into_iter().safe_sum(), Err(ArithError::Overflow));
    }

    #[test]
    fn signed_sum_overflow_below() {
        let v = vec![i16::MIN + 1, -2];
        assert_eq!(v.into_iter().safe_sum(), Err(ArithError::Overflow));

        // Additional edge case with large negative numbers
        let v = vec![i64::MIN + 1, -2];
        assert_eq!(v.into_iter().safe_sum(), Err(ArithError::Overflow));
    }

    #[test]
    fn signed_sum_almost_overflow() {
        let arr = [i64::MIN + 1, 2, -2i64, i64::MAX - 1, i64::MAX, -1];
        assert_eq!(
            arr.iter().copied().safe_sum().unwrap(),
            arr.iter().copied().sum()
        );

        // Additional case with values close to the limits
        let arr = [i64::MAX / 3, i64::MAX / 3, i64::MAX / 3];
        assert_eq!(arr.iter().copied().safe_sum().unwrap(), arr.iter().copied().sum());
    }
}