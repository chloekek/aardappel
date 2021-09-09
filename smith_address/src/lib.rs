pub use self::immediate::*;
pub use self::reference::*;

mod immediate;
mod reference;

#[derive(Clone, Copy)]
pub union Address
{
    immediate: Immediate,
    reference: Reference,
    bytes: [u8; 32],
}

impl Address
{
    pub fn as_immediate(&self)
        -> Result<&Immediate, &Reference>
    {
        unsafe {
            match self.bytes[31] & 0b1 {
                0b0 => Ok(&self.immediate),
                _   => Err(&self.reference),
            }
        }
    }

    pub fn as_immediate_mut(&mut self)
        -> Result<&mut Immediate, &mut Reference>
    {
        unsafe {
            match self.bytes[31] & 0b1 {
                0b0 => Ok(&mut self.immediate),
                _   => Err(&mut self.reference),
            }
        }
    }

    pub fn as_external(&self)
        -> Result<&Reference, &Immediate>
    {
        match self.as_immediate() {
            Ok(immediate) => Err(immediate),
            Err(reference) => Ok(reference),
        }
    }

    pub fn as_external_mut(&mut self)
        -> Result<&mut Reference, &mut Immediate>
    {
        match self.as_immediate_mut() {
            Ok(immediate) => Err(immediate),
            Err(reference) => Ok(reference),
        }
    }

    pub fn as_bytes(&self) -> &[u8; 32]
    {
        unsafe {
            &self.bytes
        }
    }
}

impl PartialEq for Address
{
    fn eq(&self, other: &Address) -> bool
    {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for Address
{
}
