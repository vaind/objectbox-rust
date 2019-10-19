use crate::c;
use std::{error, fmt, result};

enum Repr {
    Native(c::NativeError),
}

pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn new_native(native_error: c::NativeError) -> Error {
        Error {
            repr: Repr::Native(native_error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.repr {
            Repr::Native(ref err) => write!(fmt, "{}", err),
        }
    }
}

impl fmt::Debug for Repr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Repr::Native(ref err) => fmt::Debug::fmt(&err, fmt),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.repr {
            Repr::Native(ref err) => err.source(),
        }
    }
}

// A specialized result
pub type Result<T> = result::Result<T, Error>;
