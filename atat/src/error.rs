use heapless::Vec;

/// Errors returned used internally within the crate
#[derive(Clone, Debug, PartialEq)]
pub enum InternalError {
    /// Serial read error
    Read,
    /// Serial write error
    Write,
    /// Timed out while waiting for a response
    Timeout,
    /// Invalid response from module
    InvalidResponse,
    /// Command was aborted
    Aborted,
    /// Buffer overflow
    Overflow,
    /// Failed to parse received response
    Parse,
    /// Error response containing any error message
    Error(Vec<u8, 85>),
}

impl InternalError {
    pub fn as_byte(&self) -> u8 {
        match self {
            InternalError::Read => 0x00,
            InternalError::Write => 0x01,
            InternalError::Timeout => 0x02,
            InternalError::InvalidResponse => 0x03,
            InternalError::Aborted => 0x04,
            InternalError::Overflow => 0x05,
            InternalError::Parse => 0x06,
            InternalError::Error(_) => 0x07,
        }
    }

    pub fn from_bytes(b: &[u8]) -> Self {
        match &b[0] {
            0x00 => InternalError::Read,
            0x01 => InternalError::Write,
            0x02 => InternalError::Timeout,
            0x03 => InternalError::InvalidResponse,
            0x04 => InternalError::Aborted,
            0x05 => InternalError::Overflow,
            0x06 => InternalError::Parse,
            0x07 if b.len() > 1 => InternalError::Error(Vec::from_slice(&b[1..]).unwrap()),
            0x07 => InternalError::Error(Vec::new()),
            _ => InternalError::Parse,
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for InternalError {
    fn format(&self, f: defmt::Formatter) {
        match self {
            InternalError::Read => defmt::write!(f, "Read"),
            InternalError::Write => defmt::write!(f, "Write"),
            InternalError::Timeout => defmt::write!(f, "Timeout"),
            InternalError::InvalidResponse => defmt::write!(f, "InvalidResponse"),
            InternalError::Aborted => defmt::write!(f, "Aborted"),
            InternalError::Overflow => defmt::write!(f, "Overflow"),
            InternalError::Parse => defmt::write!(f, "Parse"),
            InternalError::Error(e) => defmt::write!(f, "Error({=[u8]:a})", &e),
        }
    }
}

/// Errors returned by the crate
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E = GenericError> {
    /// Serial read error
    Read,
    /// Serial write error
    Write,
    /// Timed out while waiting for a response
    Timeout,
    /// Invalid response from module
    InvalidResponse,
    /// Command was aborted
    Aborted,
    /// Buffer overflow
    Overflow,
    /// Failed to parse received response
    Parse,
    /// Error response containing any error message
    Error(E),
}

impl<E> From<InternalError> for Error<E>
where
    E: core::str::FromStr,
{
    fn from(ie: InternalError) -> Self {
        match ie {
            InternalError::Read => Self::Read,
            InternalError::Write => Self::Write,
            InternalError::Timeout => Self::Timeout,
            InternalError::InvalidResponse => Self::InvalidResponse,
            InternalError::Aborted => Self::Aborted,
            InternalError::Overflow => Self::Overflow,
            InternalError::Parse => Self::Parse,
            InternalError::Error(ref e) => {
                if let Ok(s) = core::str::from_utf8(e) {
                    if let Ok(e) = core::str::FromStr::from_str(s) {
                        return Self::Error(e);
                    }
                }
                Self::Parse
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GenericError;

impl core::str::FromStr for GenericError {
    type Err = core::convert::Infallible;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(GenericError)
    }
}
