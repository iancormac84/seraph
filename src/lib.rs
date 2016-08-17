#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate cgmath;
extern crate dwmapi;
extern crate gdi32;
extern crate ole32;
extern crate setupapi;
extern crate user32;
extern crate uuid;
extern crate winapi;
extern crate winreg;

pub mod platform;

pub use platform::{generic, windows};

pub static mut PUMPING_MESSAGE_OUTSIDE_OF_MAIN_LOOP: bool = true;

//use uuid::{IID_IDropTarget, IID_IUnknown};