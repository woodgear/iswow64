//! Determines whether the current process is running under WOW64.
//!
//! # Example:
//!
//! ```rust
//! extern crate iswow64;
//!
//! let result = iswow64::iswow64();
//!
//! println!("{:?}", result);
//!
//! #[cfg(target_arch = "x86")]
//! assert_eq!(result.unwrap(), true);
//!
//! #[cfg(target_arch = "x86_64")]
//! assert_eq!(result.unwrap(), false);
//!
//! ```

#[cfg(windows)]
extern crate winapi;

use winapi::{
    shared::minwindef::BOOL,
    um::{
        errhandlingapi::GetLastError, processthreadsapi::GetCurrentProcess,
        wow64apiset::IsWow64Process,
    },
};

#[cfg(windows)]
pub fn iswow64() -> Result<bool, u32> {
    let mut is_wow64: BOOL = 0;
    unsafe {
        match IsWow64Process(GetCurrentProcess(), &mut is_wow64) {
            0 => Err(GetLastError()),
            _ => Ok(is_wow64 == 1),
        }
    }
}

#[cfg(all(test, windows))]
mod test {
    use super::*;

    #[test]
    fn iswow64_test() {
        let result = iswow64();

        #[cfg(target_arch = "x86")]
        assert_eq!(result.unwrap(), true);

        #[cfg(target_arch = "x86_64")]
        assert_eq!(result.unwrap(), false);
    }
}
