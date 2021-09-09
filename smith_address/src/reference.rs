/// Address for values with children or much auxiliary data.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Reference
{
    bytes: [u8; 32],
}
