use crate::windows::utils::ToWide;
use std::{io, ptr};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{MessageBoxW, MESSAGEBOX_RESULT, MESSAGEBOX_STYLE},
    },
};

#[derive(Debug)]
pub enum MessageBoxResult {
    Ok = 1,
    Cancel = 2,
    Abort = 3,
    Retry = 4,
    Ignore = 5,
    Yes = 6,
    No = 7,
    TryAgain = 10,
    Continue = 11,
}

impl TryFrom<MESSAGEBOX_RESULT> for MessageBoxResult {
    type Error = io::Error;
    fn try_from(value: MESSAGEBOX_RESULT) -> Result<Self, Self::Error> {
        match value.0 {
            3 => Ok(MessageBoxResult::Abort),
            2 => Ok(MessageBoxResult::Cancel),
            11 => Ok(MessageBoxResult::Continue),
            5 => Ok(MessageBoxResult::Ignore),
            7 => Ok(MessageBoxResult::No),
            1 => Ok(MessageBoxResult::Ok),
            4 => Ok(MessageBoxResult::Retry),
            10 => Ok(MessageBoxResult::TryAgain),
            6 => Ok(MessageBoxResult::Yes),
            _ => {
                let msg = format!("unexpected message box result {}", value.0);
                Err(io::Error::new(io::ErrorKind::Other, &msg[..]))
            }
        }
    }
}

pub fn message_box(
    wnd: Option<HWND>,
    text: &str,
    caption: Option<&str>,
    type_: Option<u32>,
) -> io::Result<MessageBoxResult> {
    unsafe {
        let wnd = wnd.unwrap_or(HWND(ptr::null_mut()));
        let text = text.to_wide_null();
        let text = text.as_ptr();
        let caption = caption.map(|v| v.to_wide_null());
        let caption = caption.as_ref().map(|v| v.as_ptr()).unwrap_or(ptr::null());
        let type_ = type_.unwrap_or(0);
        match MessageBoxW(wnd, PCWSTR(text), PCWSTR(caption), MESSAGEBOX_STYLE(type_)) {
            MESSAGEBOX_RESULT(0) => Err(io::Error::last_os_error()),
            v => Ok(MessageBoxResult::try_from(v)?),
        }
    }
}
