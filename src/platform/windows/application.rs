use dwmapi;
use platform::generic::window_definition::WindowTransparency;
use platform::windows::window::WindowsWindow;
use std::rc::{Rc, Weak};
use winapi::{DWORD, FALSE, HINSTANCE, HWND, LPARAM, POINTL, WPARAM};

pub enum TaskbarProgressState {
    //Stops displaying progress and returns the button to its normal state.
    NoProgress = 0x0,

    //The progress indicator does not grow in size, but cycles repeatedly along the 
    //length of the task bar button. This indicates activity without specifying what 
    //proportion of the progress is complete. Progress is taking place, but there is 
    //no prediction as to how long the operation will take.
    Indeterminate = 0x1,

    //The progress indicator grows in size from left to right in proportion to the 
    //estimated amount of the operation completed. This is a determinate progress 
    //indicator; a prediction is being made as to the duration of the operation.
    Normal = 0x2,

    //The progress indicator turns red to show that an error has occurred in one of 
    //the windows that is broadcasting progress. This is a determinate state. If the 
    //progress indicator is in the indeterminate state, it switches to a red determinate 
    //display of a generic percentage not indicative of actual progress.
    Error = 0x4,

    //The progress indicator turns yellow to show that progress is currently stopped in 
    //one of the windows but can be resumed by the user. No error condition exists and 
    //nothing is preventing the progress from continuing. This is a determinate state. 
    //If the progress indicator is in the indeterminate state, it switches to a yellow 
    //determinate display of a generic percentage not indicative of actual progress.
    Paused = 0x8,
}

//TODO implement a TaskbarList struct that is built around the ITaskbarList3 COM interface

pub struct DeferredWindowsMessage {
    native_window: Weak<WindowsWindow>,
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    mouse_coord_x: i32,
    mouse_coord_y: i32,
    raw_input_flags: u32,
}

impl DeferredWindowsMessage {
    pub fn new(
        native_window: &Rc<WindowsWindow>,
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        x: i32,
        y: i32,
        raw_input_flags: u32) -> DeferredWindowsMessage {
        DeferredWindowsMessage {
            native_window: Rc::downgrade(native_window),
            hwnd: hwnd,
            message: message,
            wparam: wparam,
            lparam: lparam,
            mouse_coord_x: x,
            mouse_coord_y: y,
            raw_input_flags: raw_input_flags,
        }
    }
}

pub enum WindowsDragDropOperationType {
    DragEnter,
    DragOver,
    DragLeave,
    Drop
}

//This is internal to the DragDropOLEData struct in the Unreal Engine 4 code. 
pub enum WindowsOLEDataType {
    None  = 0,
    Text  = 1 << 0,
    Files = 1 << 1,
}

pub struct DragDropOLEData {
    operation_text: String,
    operation_filenames: Vec<String>,
    data_type: u8,
}

pub struct DeferredWindowsDragDropOperation {
    operation_type: WindowsDragDropOperationType,
    hwnd: HWND,
    ole_data: DragDropOLEData,
    key_state: DWORD,
    cursor_position: POINTL,
}

//I just got an idea about making it a struct that deals with a type that implements a trait that contains the process_message method, but probably not.
pub trait IWindowsMessageHandler {
    fn process_message(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM, out_result: &mut i32) -> bool;
}

//This is internal to the WindowsApplication class in the Unreal Engine 4 code.
pub enum ModifierKey {
    LeftShift,      // VK_LSHIFT
    RightShift,     // VK_RSHIFT
    LeftControl,    // VK_LCONTROL
    RightControl,   // VK_RCONTROL
    LeftAlt,        // VK_LMENU
    RightAlt,       // VK_RMENU
    CapsLock,       // VK_CAPITAL
}

//TODO implement GenericApplication trait. Also most likely trait based on IForceFeedbackSystem.
pub struct WindowsApplication {
	minimized_window_position: (i32, i32),
	instance_handle: HINSTANCE,
    using_high_precision_mouse_input: bool,
    is_mouse_attached: bool,
    force_activate_by_mouse: bool,
    deferred_messages: Vec<DeferredWindowsMessage>,
    deferred_drag_drop_operations: Vec<DeferredWindowsDragDropOperation>,
    message_handlers: Vec<Box<IWindowsMessageHandler>>,
    windows: Vec<Rc<WindowsWindow>>,
}

impl WindowsApplication {
	pub fn get_window_transparency_support(&self) -> WindowTransparency {
        let mut is_composition_enabled = FALSE;
	    unsafe { dwmapi::DwmIsCompositionEnabled(&mut is_composition_enabled); }

	    if is_composition_enabled != 0 { WindowTransparency::PerPixel } else { WindowTransparency::PerWindow }
    }
}