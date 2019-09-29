use crate::Error;
use std::convert::TryFrom;

/// A data byte that holds 7 bits of information.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct U7(u8);

impl U7 {
    /// The minimum value for a u7 data byte.
    pub const MIN: U7 = U7(0);
    /// The maximum value for a u7 data byte.
    pub const MAX: U7 = U7(127);

    /// Convert a `u8` into a `U7` without bounds checking.
    #[inline(always)]
    pub unsafe fn from_unchecked(data: u8) -> U7 {
        U7(data)
    }

    /// Convert a `u8` into a `U7`. If any of the data is out of range, then an error is returned.
    #[inline(always)]
    pub fn try_from_bytes(bytes: &[u8]) -> Result<&[U7], Error> {
        for b in bytes.iter() {
            U7::try_from(*b)?;
        }
        unsafe { Ok(U7::from_bytes_unchecked(bytes)) }
    }

    /// Convert a slice of `U7` into a slice `u8`. Since `U7` is a subset of `u8`, this is a simple
    /// cast.
    #[inline(always)]
    pub fn data_to_bytes(data: &[U7]) -> &[u8] {
        unsafe { &*(data as *const [U7] as *const [u8]) }
    }

    /// Convert a slice of `u8` to a slice of `U7` without bounds checking.
    #[inline(always)]
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &[U7] {
        &*(bytes as *const [u8] as *const [U7])
    }
}

impl From<U7> for u8 {
    #[inline(always)]
    fn from(data: U7) -> u8 {
        data.0
    }
}

impl TryFrom<u8> for U7 {
    type Error = Error;

    #[inline(always)]
    fn try_from(data: u8) -> Result<U7, Error> {
        if data > 127 {
            Err(Error::DataByteOutOfRange)
        } else {
            Ok(U7(data))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_passes() {
        for n in 0x00..0x80 {
            U7::try_from(n).unwrap();
        }
    }

    #[test]
    fn min_and_max_constant_are_valid() {
        assert_eq!(U7::try_from(u8::from(U7::MIN)).unwrap(), U7::MIN);
        assert_eq!(U7::try_from(u8::from(U7::MAX)).unwrap(), U7::MAX);
    }

    #[test]
    fn try_from_out_of_range_fails() {
        for n in 0x80..=std::u8::MAX {
            assert_eq!(U7::try_from(n), Err(Error::DataByteOutOfRange));
        }
    }

    #[test]
    fn try_from_bytes_is_ok_on_valid_bytes() {
        U7::try_from_bytes(&[]).unwrap();
        U7::try_from_bytes(&[0x00, 0x08, 0x10, 0x20, 0x30, 0x40, 0x7F]).unwrap();
    }

    #[test]
    fn try_from_bytes_fails_on_out_of_range() {
        assert_eq!(
            U7::try_from_bytes(&[0x00, 0x80]),
            Err(Error::DataByteOutOfRange)
        );
    }

    #[test]
    fn data_to_bytes_converts_exactly() {
        assert_eq!(
            &[0x00, 0x0F, 0x7F],
            U7::data_to_bytes(&[
                U7::try_from(0x00).unwrap(),
                U7::try_from(0x0F).unwrap(),
                U7::try_from(0x7F).unwrap()
            ]),
        );
    }
}
