use std::mem;

/// Address for short auxiliary-only values.
///
/// Any value with fewer than 32 bytes of auxiliary data and no children
/// is referred to by an immediate address and not by a reference address.
/// The immediate address directly stores the auxiliary data;
/// no dereferencing needs to take place to find the auxiliary data.
///
/// The layout of an immediate address is as follows:
/// the address begins with the auxiliary bytes,
/// followed by all-zero padding bytes,
/// followed by the metadata byte.
/// The metadata byte consists of the following bits
/// (ordered from most significant to least significant):
/// an unset bit, five bits that encode the number of auxiliary bytes,
/// an unset bit, a set bit.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Immediate
{
    bytes: [u8; 32],
}

impl Immediate
{
    /// Address of the empty value.
    ///
    /// ```rust
    /// # use smith_address::Immediate;
    /// assert_eq!(Immediate::EMPTY.auxiliary(), b"");
    /// ```
    pub const EMPTY: Self = Self{
        bytes: [
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1,
        ],
    };

    /// Create an immediate address from the bytes that make it up.
    ///
    /// If the address is formatted improperly,
    /// this function returns [`None`].
    pub fn from_bytes(bytes: &[u8; 32]) -> Option<&Self>
    {
        // Check metadata byte.
        if bytes[31] & 0b1_00000_11 != 0b0_00000_01 {
            return None;
        }

        // Check padding bytes.
        let len = bytes[31] >> 2;
        if bytes[len as usize .. 32].iter().any(|&b| b != 0) {
            return None;
        }

        // SAFETY: We have no validated the format.
        unsafe {
            Some(Self::from_bytes_unchecked(bytes))
        }
    }

    /// Create an immediate address from the bytes that make it up.
    ///
    /// # Safety
    ///
    /// If the address is formatted improperly,
    /// the behavior is undefined.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8; 32]) -> &Self
    {
        // SAFETY: Immediate is repr(transparent).
        mem::transmute(bytes)
    }

    /// The bytes that make up the address.
    ///
    /// This is different from the auxiliary data.
    /// This includes the padding bytes and metadata byte.
    pub fn as_bytes(&self) -> &[u8; 32]
    {
        &self.bytes
    }

    /// Create an immediate address from auxiliary data.
    ///
    /// ```rust
    /// # use smith_address::Immediate;
    /// fn assert_roundtrips(auxiliary: &[u8])
    /// {
    ///     let address = Immediate::from_auxiliary(auxiliary).unwrap();
    ///     assert_eq!(address.auxiliary(), auxiliary);
    /// }
    /// assert_roundtrips(b"\x00\x00\x00\x01"); // 32-bit integer
    /// assert_roundtrips(b"Hello, world!");    // short string
    /// ```
    ///
    /// If there are more than 31 bytes of auxiliary data,
    /// this function returns [`None`].
    ///
    /// ```rust
    /// # use smith_address::Immediate;
    /// let long = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    /// let address = Immediate::from_auxiliary(long);
    /// assert!(address.is_none());
    /// ```
    pub fn from_auxiliary(auxiliary: &[u8]) -> Option<Self>
    {
        if auxiliary.len() > 31 {
            return None;
        }

        let mut bytes = [0; 32];
        bytes[0 .. auxiliary.len()].copy_from_slice(auxiliary);
        bytes[31] = ((auxiliary.len() as u8) << 2) | 0b01;

        Some(Self{bytes})
    }

    /// The auxiliary data in the address.
    ///
    /// The length is always less than 32.
    pub fn auxiliary(&self) -> &[u8]
    {
        let len = self.bytes[31] >> 2;
        // SAFETY: Length being less than 32 is an invariant.
        unsafe {
            self.bytes.get_unchecked(0 .. len as usize)
        }
    }

    /// The auxiliary data in the address.
    ///
    /// The length is always less than 32.
    ///
    /// # Safety
    ///
    /// You can freely mutate the auxiliary bytes,
    /// but it is not possible to change the number of them.
    /// This is safe because the padding bytes and metadata byte
    /// are the same for any two immediate addresses
    /// with the same number of auxiliary bytes.
    pub fn auxiliary_mut(&mut self) -> &mut [u8]
    {
        let len = self.bytes[31] >> 2;
        // SAFETY: Length being less than 32 is an invariant.
        unsafe {
            self.bytes.get_unchecked_mut(0 .. len as usize)
        }
    }
}
