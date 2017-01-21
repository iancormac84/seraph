use conv::TryFrom;
use std::{io, ptr};
use user32;
use platform::windows::utils::ToWide;
use platform::windows::utils::other_error;
use winapi::*;

custom_derive! {
    #[derive(Debug, TryFrom(::std::os::raw::c_int))]
    pub enum MessageBoxResult {
        Abort = 3,
        Cancel = 2,
        Continue = 11,
        Ignore = 5,
        No = 7,
        Ok = 1,
        Retry = 4,
        TryAgain = 10,
        Yes = 6,
    }
}

pub fn message_box(wnd: Option<HWND>, text: &str, caption: Option<&str>, type_: Option<UINT>) -> io::Result<MessageBoxResult> {
    unsafe {
        let wnd = wnd.unwrap_or(ptr::null_mut());
        let text = text.to_wide_null();
        let text = text.as_ptr();
        let caption = caption.map(|v| v.to_wide_null());
        let caption = caption.as_ref().map(|v| v.as_ptr()).unwrap_or(ptr::null_mut());
        let type_ = type_.unwrap_or(0);
        match user32::MessageBoxW(wnd, text, caption, type_) {
            0 => Err(io::Error::last_os_error()),
            v => MessageBoxResult::try_from(v)
                .or_else(|_| other_error("unexpected result"))
        }
    }
}