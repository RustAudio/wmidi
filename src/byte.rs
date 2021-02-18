use crate::Error;
use core::convert::TryFrom;

/// A data byte that holds 7 bits of information.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct U7(pub(crate) u8);

impl U7 {
    /// The minimum value for a u7 data byte.
    pub const MIN: U7 = U7(0x00);
    /// The maximum value for a u7 data byte.
    pub const MAX: U7 = U7(0x80 - 0x01);

    /// Convert a `u8` into a `U7` without bounds checking.
    ///
    /// # Safety
    /// Behavior is undefined if data > 127.
    #[inline(always)]
    pub unsafe fn from_unchecked(data: u8) -> U7 {
        U7(data)
    }

    /// Create a `U7` from a `u8`. Only the 7 least significant bits of `note` are kept.
    #[inline(always)]
    pub const fn from_u8_lossy(data: u8) -> U7 {
        U7(data & 0x7F)
    }

    /// Convert a slice of `u8` into a slice of `U7`. If any of the data is out of range, then an
    /// error is returned.
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
    ///
    /// # Safety
    /// Behavior is undefined if any byte is > 127.
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
        if data > u8::from(U7::MAX) {
            Err(Error::DataByteOutOfRange)
        } else {
            Ok(U7(data))
        }
    }
}

/// A combination of 2 data bytes that holds 14 bits of information.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct U14(u16);

impl U14 {
    /// The minimum value for a u14 data byte.
    pub const MIN: U14 = U14(0);
    /// The maximum value for a u7 data byte.
    pub const MAX: U14 = U14(0x4000 - 0x0001);

    /// Convert a `u8` into a `U7` without bounds checking.
    ///
    /// # Safety
    /// Behavior is undefined if data is > 16383.
    #[inline(always)]
    pub unsafe fn from_unchecked(data: u16) -> U14 {
        U14(data)
    }

    /// Convert a slice of `u16` into a slice of `U14`. If any of the data is out of range, then an
    /// error is returned.
    #[inline(always)]
    pub fn try_from_slice(slice: &[u16]) -> Result<&[U14], Error> {
        for d in slice.iter() {
            U14::try_from(*d)?;
        }
        unsafe { Ok(U14::from_slice_unchecked(slice)) }
    }

    /// Convert a slice of `U14` into a slice `u16`. Since `U14` is a subset of `u16`, this is a
    /// simple cast.
    #[inline(always)]
    pub fn data_to_slice(data: &[U14]) -> &[u16] {
        unsafe { &*(data as *const [U14] as *const [u16]) }
    }

    /// Convert a slice of `u16` to a slice of `U14` without bounds checking.
    ///
    /// # Safety
    /// Behavior is undefined if any byte is > 16383.
    #[inline(always)]
    pub unsafe fn from_slice_unchecked(slice: &[u16]) -> &[U14] {
        &*(slice as *const [u16] as *const [U14])
    }
}

impl From<U14> for u16 {
    #[inline(always)]
    fn from(data: U14) -> u16 {
        data.0
    }
}

impl TryFrom<u16> for U14 {
    type Error = Error;

    #[inline(always)]
    fn try_from(data: u16) -> Result<U14, Error> {
        if data > u16::from(U14::MAX) {
            Err(Error::U14OutOfRange)
        } else {
            Ok(U14(data))
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

    #[test]
    fn try_from_16_passes() {
        for n in 0x0000..0x4000 {
            U14::try_from(n).unwrap();
        }
    }

    #[test]
    fn min_and_max_14_constant_are_valid() {
        assert_eq!(U14::try_from(u16::from(U14::MIN)).unwrap(), U14::MIN);
        assert_eq!(U14::try_from(u16::from(U14::MAX)).unwrap(), U14::MAX);
    }

    #[test]
    fn try_from_out_of_range_16_fails() {
        for n in 0x4000..=std::u16::MAX {
            assert_eq!(U14::try_from(n), Err(Error::U14OutOfRange));
        }
    }

    #[test]
    fn try_from_slice_is_ok_on_valid_range() {
        U14::try_from_slice(&[]).unwrap();
        U14::try_from_slice(&[0x0000, 0x0080, 0x0180, 0x01FF]).unwrap();
    }

    #[test]
    fn try_from_slice_fails_on_out_of_range() {
        assert_eq!(
            U14::try_from_slice(&[0x0000, 0x5000]),
            Err(Error::U14OutOfRange)
        );
    }

    #[test]
    fn data_to_slice_converts_exactly() {
        assert_eq!(
            &[0x0000, 0x010F, 0x017F],
            U14::data_to_slice(&[
                U14::try_from(0x0000).unwrap(),
                U14::try_from(0x010F).unwrap(),
                U14::try_from(0x017F).unwrap()
            ]),
        );
    }

    #[test]
    fn test_from_u8_lossy() {
        assert_eq!(U7::from_u8_lossy(0), U7::try_from(0).unwrap());
        assert_eq!(U7::from_u8_lossy(64), U7::try_from(64).unwrap());
        assert_eq!(U7::from_u8_lossy(127), U7::try_from(127).unwrap());
        assert_eq!(U7::from_u8_lossy(128), U7::try_from(0).unwrap());
        assert_eq!(U7::from_u8_lossy(200), U7::try_from(72).unwrap());
    }
}
