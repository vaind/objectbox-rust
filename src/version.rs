pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

use std::fmt;

// To use the `{}` marker, the trait `fmt::Display` must be implemented manually for the type.
impl fmt::Display for Version {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn rust() -> Version {
    Version {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap(),
    }
}

use crate::c;

pub fn lib() -> Version {
    let mut major: i32 = 0;
    let mut minor: i32 = 0;
    let mut patch: i32 = 0;

    unsafe { c::obx_version(&mut major, &mut minor, &mut patch) }

    Version {
        major: major as u32,
        minor: minor as u32,
        patch: patch as u32,
    }
}

pub fn info() -> String {
    format!(
        "ObjectBox Rust version {} using dynamic library version {}",
        rust(),
        lib()
    )
}
