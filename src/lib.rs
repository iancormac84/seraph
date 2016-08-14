#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate dwmapi;
extern crate gdi32;
extern crate ole32;
extern crate user32;
extern crate uuid;
extern crate winapi;

pub mod platform;

pub use platform::{generic, windows};

//use uuid::{IID_IDropTarget, IID_IUnknown};