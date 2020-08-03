use std::ffi::CStr;
use std::mem;
use std::str;

use libc::c_char;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UtsName(libc::utsname);

#[allow(dead_code)]
impl UtsName {
    pub fn uname() -> UtsName {
        unsafe {
            let mut ret: UtsName = mem::zeroed();
            libc::uname(&mut ret.0);
            ret
        }
    }

    pub fn sysname(&self) -> &str {
        to_str(&(&self.0.sysname as *const c_char ) as *const *const c_char)
    }

    pub fn nodename(&self) -> &str {
        to_str(&(&self.0.nodename as *const c_char ) as *const *const c_char)
    }

    pub fn release(&self) -> &str {
        to_str(&(&self.0.release as *const c_char ) as *const *const c_char)
    }

    pub fn version(&self) -> &str {
        to_str(&(&self.0.version as *const c_char ) as *const *const c_char)
    }

    pub fn machine(&self) -> &str {
        to_str(&(&self.0.machine as *const c_char ) as *const *const c_char)
    }
}

#[inline]
fn to_str<'a>(s: *const *const c_char) -> &'a str {
    unsafe {
        let res = CStr::from_ptr(*s).to_bytes();
        str::from_utf8_unchecked(res)
    }
}
