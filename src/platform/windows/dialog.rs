use crate::windows::utils::ToWide;
use std::{io, ptr};
use windows::Win32::{
    Foundation::{HWND, PWSTR},
    UI::WindowsAndMessaging::MessageBoxW,
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

impl TryFrom<i32> for MessageBoxResult {
    type Error = io::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
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
                let msg = format!("unexpected message box result {}", value);
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
        let wnd = wnd.unwrap_or(0);
        let text = text.to_wide_null();
        let text = text.as_mut_ptr();
        let caption = caption.map(|v| v.to_wide_null());
        let caption = caption
            .as_ref()
            .map(|v| v.as_mut_ptr())
            .unwrap_or(ptr::null_mut());
        let type_ = type_.unwrap_or(0);
        match MessageBoxW(wnd, PWSTR(text), PWSTR(caption), type_) {
            0 => Err(io::Error::last_os_error()),
            v => Ok(MessageBoxResult::try_from(v)?),
        }
    }
}
