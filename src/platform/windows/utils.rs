use kernel32;
use std::ffi::{OsStr, OsString};
use std::{io, mem};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use user32;
use winapi::{HWND, LONG_PTR};

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}

impl<T> ToWide for T where T: AsRef<OsStr> {
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

pub fn other_error<T>(msg: &str) -> io::Result<T> {
    Err(io::Error::new(io::ErrorKind::Other, msg))
}

pub fn leak<T>(v: T) -> &'static T {
    unsafe {
        println!("Inside utils::leak");
        let b = Box::new(v);
        let p: *const T = &*b;
        mem::forget(b); // leak our reference, so that `b` is never freed
        &*p
    }
}

pub unsafe fn get_window_long_ptr<T>(wnd: HWND, index: i32) -> io::Result<*const T> {
    #[cfg(target_pointer_width="32")]
    use user32::GetWindowLongW as GetWindowLongPtr;

    #[cfg(target_pointer_width="64")]
    use user32::GetWindowLongPtrW as GetWindowLongPtr;

    // Clear so that we can distinguish from "success, and the value was zero" and "failure".
    kernel32::SetLastError(0);

    match GetWindowLongPtr(wnd, index) {
        0 if kernel32::GetLastError() == 0 => {
            println!("Last error: {}", io::Error::last_os_error());
            Ok(0usize as *const T)
        },
        0 => Err(io::Error::last_os_error()),
        v => Ok(v as *const T)
    }
}

pub unsafe fn set_window_long_ptr<T>(wnd: HWND, index: i32, new_long: *const T)
    -> io::Result<*const T> {
    #[cfg(target_pointer_width="32")]
    use user32::SetWindowLongW as SetWindowLongPtr;

    #[cfg(target_pointer_width="64")]
    use user32::SetWindowLongPtrW as SetWindowLongPtr;

    // Clear so that we can distinguish from "success, and the last value was zero" and "failure".
    kernel32::SetLastError(0);

    let new_long = new_long as LONG_PTR;

    match SetWindowLongPtr(wnd, index, new_long) {
        0 if kernel32::GetLastError() == 0 => Ok(0usize as *const T),
        0 => Err(io::Error::last_os_error()),
        v => Ok(v as *const T)
    }
}