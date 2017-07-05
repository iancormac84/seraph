use cgmath::Point2;
//use dwmapi;
use platform::generic::application::{GenericApplication, MonitorInfo, PlatformRect, DEBUG_ACTION_ZONE_RATIO, DEBUG_SAFE_ZONE_RATIO};
use platform::generic::application_message_handler::{ApplicationMessageHandler, WindowAction, WindowSizeLimits, WindowZone};
//use platform::generic::cursor::ICursor;
use platform::generic::window::GenericWindow;
use platform::generic::window_definition::{WindowDefinition, WindowTransparency, WindowType};
use platform::windows::cursor::WindowsCursor;
use platform::windows::utils;
use platform::windows::utils::ToWide;
use platform::windows::window::{APP_WINDOW_CLASS, WindowsWindow};
use platform::windows::xinputinterface::XInputInterface;
use setupapi;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Error;
use std::os::raw::c_void;
use std::{mem, ptr};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Once, ONCE_INIT};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use super::{
  DLGC_WANTALLKEYS, FILTERKEYS, FKF_CONFIRMHOTKEY, FKF_FILTERKEYSON, FKF_HOTKEYACTIVE, IMN_CHANGECANDIDATE, IMN_CLOSECANDIDATE, IMN_CLOSESTATUSWINDOW, IMN_GUIDELINE, IMN_OPENCANDIDATE, IMN_OPENSTATUSWINDOW, IMN_PRIVATE, IMN_SETCANDIDATEPOS, IMN_SETCOMPOSITIONFONT, IMN_SETCOMPOSITIONWINDOW, IMN_SETCONVERSIONMODE, IMN_SETOPENSTATUS, IMN_SETSENTENCEMODE, IMN_SETSTATUSWINDOWPOS, IMR_CANDIDATEWINDOW, IMR_COMPOSITIONFONT, IMR_COMPOSITIONWINDOW, IMR_CONFIRMRECONVERTSTRING, IMR_DOCUMENTFEED, IMR_QUERYCHARPOSITION, IMR_RECONVERTSTRING,
  SKF_CONFIRMHOTKEY, SKF_HOTKEYACTIVE, SKF_STICKYKEYSON, STICKYKEYS, TKF_CONFIRMHOTKEY, TKF_HOTKEYACTIVE, TKF_TOGGLEKEYSON, TOGGLEKEYS, WINDOWINFO
};
use user32;
use winapi::{
    c_short, CREATESTRUCTW, CR_SUCCESS, CS_DBLCLKS, DICS_FLAG_GLOBAL, DIGCF_PRESENT, DIREG_DEV, DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP, DISPLAY_DEVICE_MIRRORING_DRIVER,
    DISPLAY_DEVICE_PRIMARY_DEVICE, DISPLAY_DEVICEW, DWORD, ERROR_NO_MORE_ITEMS, FALSE, GWLP_USERDATA, GWL_STYLE,
    GWL_EXSTYLE, HDEVINFO, HICON, HINSTANCE, HIWORD, HKEY, HMONITOR,
    HRAWINPUT, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION, HTCLIENT, HTCLOSE, HTLEFT, HTMINBUTTON, HTMAXBUTTON, HTNOWHERE, HTRIGHT, HTSYSMENU, HTTOP, HTTOPLEFT, HTTOPRIGHT, HWND,
    INVALID_HANDLE_VALUE, KEY_READ, LOWORD, LPARAM, LPNCCALCSIZE_PARAMS,
    LPVOID, LPWSTR, LRESULT,
    MAX_DEVICE_ID_LEN, MB_ICONEXCLAMATION, MB_OK, MINMAXINFO, MONITORINFO, MONITOR_DEFAULTTONEAREST, MOUSE_MOVE_ABSOLUTE, MOUSE_MOVE_RELATIVE, MSG, NCCALCSIZE_PARAMS, PM_REMOVE, POINT,
    POINTL, RAWINPUT, RAWINPUTDEVICE, RAWINPUTDEVICELIST, RAWINPUTHEADER,
    RECT, RIDEV_REMOVE, RIDI_DEVICENAME, RID_INPUT, RIM_TYPEMOUSE, SC_MAXIMIZE, SC_RESTORE, SM_CXSCREEN,
    SM_CYSCREEN, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SP_DEVINFO_DATA, SPI_GETWORKAREA, SPI_SETFILTERKEYS, SPI_SETSTICKYKEYS, SPI_SETTOGGLEKEYS,
    SW_RESTORE, TRUE, VK_CAPITAL, VK_F4, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_SPACE, WM_ACTIVATE, WM_ACTIVATEAPP, WM_CHAR, WM_CLOSE, WM_CREATE, WM_DESTROY,
    WM_DEVICECHANGE, WM_DISPLAYCHANGE, WM_GETDLGCODE, WM_DWMCOMPOSITIONCHANGED, WM_ERASEBKGND, WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_GETMINMAXINFO, WM_INPUT,
    WM_INPUT_DEVICE_CHANGE, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEHOVER,
    WM_MOUSEHWHEEL,
    WM_MOUSELEAVE, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_NCACTIVATE, WM_NCCALCSIZE, WM_NCCREATE, WM_NCMOUSEHOVER, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE, WM_NCMBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCMBUTTONDOWN, WM_NCMBUTTONUP, WM_NCRBUTTONDBLCLK, WM_NCRBUTTONDOWN,
    WM_NCRBUTTONUP, WM_NCXBUTTONDBLCLK, WM_NCXBUTTONDOWN, WM_NCXBUTTONUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP, WM_SYSCHAR,
    WM_SYSKEYDOWN, WM_SYSCOMMAND, WM_IME_CHAR,
    WM_IME_COMPOSITION, WM_IME_ENDCOMPOSITION, WM_IME_NOTIFY, WM_IME_REQUEST, WM_IME_SETCONTEXT, WM_IME_STARTCOMPOSITION, WM_INPUTLANGCHANGE, WM_INPUTLANGCHANGEREQUEST, WM_MOVE,
    WM_NCHITTEST, WM_NCPAINT, WM_PAINT,
    WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SETCURSOR, WM_SETTINGCHANGE, WM_SHOWWINDOW, WM_SIZE, WM_SIZING, WM_SYSKEYUP, WM_TOUCH, WMSZ_BOTTOM, WMSZ_BOTTOMLEFT,
    WMSZ_BOTTOMRIGHT, WMSZ_LEFT,
    WMSZ_RIGHT, WMSZ_TOP, WMSZ_TOPLEFT, WMSZ_TOPRIGHT, WNDCLASSW, WPARAM, WVR_VALIDRECTS
};
use winreg::RegKey;

type IntPoint2 = Point2<i32>;

pub static mut WINDOWS_APPLICATION: Option<&'static Arc<WindowsApplication>> = None;
static INIT_APPLICATION: Once = ONCE_INIT;

lazy_static! {
    static ref WINDOWS_MESSAGE_STRINGS: BTreeMap<u32, &'static str> = {
        let mut result: BTreeMap<u32, &'static str> = BTreeMap::new();
        result.insert(WM_INPUTLANGCHANGEREQUEST, "WM_INPUTLANGCHANGEREQUEST");
        result.insert(WM_INPUTLANGCHANGE, "WM_INPUTLANGCHANGE");
        result.insert(WM_IME_SETCONTEXT, "WM_IME_SETCONTEXT");
        result.insert(WM_IME_NOTIFY, "WM_IME_NOTIFY");
        result.insert(WM_IME_REQUEST, "WM_IME_REQUEST");
        result.insert(WM_IME_STARTCOMPOSITION, "WM_IME_STARTCOMPOSITION");
        result.insert(WM_IME_COMPOSITION, "WM_IME_COMPOSITION");
        result.insert(WM_IME_ENDCOMPOSITION, "WM_IME_ENDCOMPOSITION");
        result.insert(WM_IME_CHAR, "WM_IME_CHAR");
        result
    };

    static ref IMN_STRINGS: BTreeMap<u32, &'static str> = {
        let mut result: BTreeMap<u32, &'static str> = BTreeMap::new();
        result.insert(IMN_CLOSESTATUSWINDOW, "IMN_CLOSESTATUSWINDOW");
        result.insert(IMN_OPENSTATUSWINDOW, "IMN_OPENSTATUSWINDOW");
        result.insert(IMN_CHANGECANDIDATE, "IMN_CHANGECANDIDATE");
        result.insert(IMN_CLOSECANDIDATE, "IMN_CLOSECANDIDATE");
        result.insert(IMN_OPENCANDIDATE, "IMN_OPENCANDIDATE");
        result.insert(IMN_SETCONVERSIONMODE, "IMN_SETCONVERSIONMODE");
        result.insert(IMN_SETSENTENCEMODE, "IMN_SETSENTENCEMODE");
        result.insert(IMN_SETOPENSTATUS, "IMN_SETOPENSTATUS");
        result.insert(IMN_SETCANDIDATEPOS, "IMN_SETCANDIDATEPOS");
        result.insert(IMN_SETCOMPOSITIONFONT, "IMN_SETCOMPOSITIONFONT");
        result.insert(IMN_SETCOMPOSITIONWINDOW, "IMN_SETCOMPOSITIONWINDOW");
        result.insert(IMN_SETSTATUSWINDOWPOS, "IMN_SETSTATUSWINDOWPOS");
        result.insert(IMN_GUIDELINE, "IMN_GUIDELINE");
        result.insert(IMN_PRIVATE, "IMN_PRIVATE");
        result
    };

    static ref IMR_STRINGS: BTreeMap<u32, &'static str> = {
        let mut result: BTreeMap<u32, &'static str> = BTreeMap::new();
        result.insert(IMR_CANDIDATEWINDOW, "IMR_CANDIDATEWINDOW");
        result.insert(IMR_COMPOSITIONFONT, "IMR_COMPOSITIONFONT");
        result.insert(IMR_COMPOSITIONWINDOW, "IMR_COMPOSITIONWINDOW");
        result.insert(IMR_CONFIRMRECONVERTSTRING, "IMR_CONFIRMRECONVERTSTRING");
        result.insert(IMR_DOCUMENTFEED, "IMR_DOCUMENTFEED");
        result.insert(IMR_QUERYCHARPOSITION, "IMR_QUERYCHARPOSITION");
        result.insert(IMR_RECONVERTSTRING, "IMR_RECONVERTSTRING");
        result
    };
    static ref MINIMIZED_WINDOW_POSITION: IntPoint2 = IntPoint2::new(-32000, -32000);
}

static HIT_RESULTS: [LRESULT; 15] = [HTNOWHERE as i64, HTTOPLEFT as i64, HTTOP as i64, HTTOPRIGHT as i64, HTLEFT as i64, HTCLIENT as i64,
                                     HTRIGHT as i64, HTBOTTOMLEFT as i64, HTBOTTOM as i64, HTBOTTOMRIGHT as i64, HTCAPTION as i64,
                                     HTMINBUTTON as i64, HTMAXBUTTON as i64, HTCLOSE as i64, HTSYSMENU as i64];

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

/*pub struct DeferredWindowsMessage {
    pub native_window: Weak<WindowsWindow>,
    pub hwnd: HWND,
    pub message: u32,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
    pub mouse_coord_x: i32,
    pub mouse_coord_y: i32,
    pub raw_input_flags: u32,
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
}*/

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
//Actually, this will most likely remain a trait whose method wraps the real ProcessMessage method, since the real method needs an ABI signature that is not compatible with Rust's method
//call signatures.
pub trait IWindowsMessageHandler {
    fn process_message(&mut self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM, out_result: &mut i32) -> bool;
}

//This is internal to the WindowsApplication class in the Unreal Engine 4 code.
pub enum ModifierKey {
    LeftShift = 0,      // VK_LSHIFT
    RightShift = 1,     // VK_RSHIFT
    LeftControl = 2,    // VK_LCONTROL
    RightControl = 3,   // VK_RCONTROL
    LeftAlt = 4,        // VK_LMENU
    RightAlt = 5,       // VK_RMENU
    CapsLock = 6,       // VK_CAPITAL
    Count = 7,
}

pub fn create_windows_application(hinstance: HINSTANCE, hicon: HICON) -> &'static Arc<WindowsApplication> {
    INIT_APPLICATION.call_once(|| unsafe {
        init_windows_application(hinstance, hicon)
    });
    unsafe { WINDOWS_APPLICATION.unwrap() }
}

unsafe fn init_windows_application(hinstance: HINSTANCE, hicon: HICON) {
    let mut app = utils::leak(Arc::new(WindowsApplication::new(hinstance, hicon)));
    println!("app address is {:p}", &app);
    WINDOWS_APPLICATION = Some(app);
}

//TODO implement GenericApplication trait. Also most likely trait based on IForceFeedbackSystem.
#[derive(Debug)]
pub struct WindowsApplication {
    cursor: Rc<WindowsCursor>,
	minimized_window_position: IntPoint2,
	instance_handle: HINSTANCE,
    using_high_precision_mouse_input: bool,
    is_mouse_attached: bool,
    force_activate_by_mouse: bool,
    pub windows: RefCell<Vec<Rc<RefCell<WindowsWindow>>>>,
    //modifier_key_state: [bool; ModifierKey::Count as usize],
    in_modal_size_loop: bool,
    pub display_metrics: DisplayMetrics,
    //startup_sticky_keys: STICKYKEYS,
    //startup_toggle_keys: TOGGLEKEYS,
    //startup_filter_keys: FILTERKEYS,
}

impl WindowsApplication {
    /*fn allow_accessibility_shortcut_keys(&mut self, allow_keys: bool) {
        unsafe {
            if allow_keys {
                // Restore StickyKeys/etc to original state and enable Windows key      
                user32::SystemParametersInfoW(SPI_SETSTICKYKEYS, mem::size_of::<STICKYKEYS>() as u32, &mut self.startup_sticky_keys as *mut _ as *mut c_void, 0);
                user32::SystemParametersInfoW(SPI_SETTOGGLEKEYS, mem::size_of::<TOGGLEKEYS>() as u32, &mut self.startup_toggle_keys as *mut _ as *mut c_void, 0);
                user32::SystemParametersInfoW(SPI_SETFILTERKEYS, mem::size_of::<FILTERKEYS>() as u32, &mut self.startup_filter_keys as *mut _ as *mut c_void, 0);
            } else {
                // Disable StickyKeys/etc shortcuts but if the accessibility feature is on, 
                // then leave the settings alone as its probably being usefully used

                let mut sk_off: STICKYKEYS = mem::uninitialized();
                if (self.startup_sticky_keys.dwFlags & SKF_STICKYKEYSON) == 0 {
                    // Disable the hotkey and the confirmation
                    sk_off.dwFlags &= !SKF_HOTKEYACTIVE;
                    sk_off.dwFlags &= !SKF_CONFIRMHOTKEY;

                    user32::SystemParametersInfoW(SPI_SETSTICKYKEYS, mem::size_of::<STICKYKEYS>() as u32, &mut sk_off as *mut _ as *mut c_void, 0);
                }

                let mut tk_off: TOGGLEKEYS = mem::uninitialized();
                if (self.startup_toggle_keys.dwFlags & TKF_TOGGLEKEYSON) == 0 {
                    // Disable the hotkey and the confirmation
                    tk_off.dwFlags &= !TKF_HOTKEYACTIVE;
                    tk_off.dwFlags &= !TKF_CONFIRMHOTKEY;

                    user32::SystemParametersInfoW(SPI_SETTOGGLEKEYS, mem::size_of::<TOGGLEKEYS>() as u32, &mut tk_off as *mut _ as *mut c_void, 0);
                }

                let mut fk_off: FILTERKEYS = mem::uninitialized();
                if (self.startup_filter_keys.dwFlags & FKF_FILTERKEYSON) == 0 {
                    // Disable the hotkey and the confirmation
                    fk_off.dwFlags &= !FKF_HOTKEYACTIVE;
                    fk_off.dwFlags &= !FKF_CONFIRMHOTKEY;

                    user32::SystemParametersInfoW(SPI_SETFILTERKEYS, mem::size_of::<FILTERKEYS>() as u32, &mut fk_off as *mut _ as *mut c_void, 0);
                }
            }
        }
    }*/
    pub fn new(hinstance: HINSTANCE, hicon: HICON) -> WindowsApplication {
        let display_metrics = DisplayMetrics::new();
        let mut winapp = WindowsApplication {
            cursor: Rc::new(WindowsCursor::new()),
            minimized_window_position: IntPoint2::new(-32000, -32000),
            instance_handle: hinstance,
            using_high_precision_mouse_input: false,
            is_mouse_attached: false,
            force_activate_by_mouse: false,
            windows: RefCell::new(vec![]),
            //modifier_key_state: unsafe { mem::zeroed() },
            in_modal_size_loop: false,
            display_metrics: display_metrics,
            //startup_sticky_keys: STICKYKEYS,
            //startup_toggle_keys: TOGGLEKEYS,
            //startup_filter_keys: FILTERKEYS,
        };
        println!("Successed created winapp");
        let class_registered = winapp.register_class(hinstance, hicon);
        println!("Have we registered class? {}", class_registered);
        winapp.query_connected_mice();
        println!("winapp debug is {:#?}", winapp);
        winapp
    }
    pub fn make_window(&self) -> Rc<RefCell<WindowsWindow>> {
        WindowsWindow::make()
    }
    pub fn initialize_window(&self, window: &Rc<RefCell<WindowsWindow>>, definition: &Rc<RefCell<WindowDefinition>>, parent: Option<Rc<WindowsWindow>>, show_immediately: bool) {
        println!("about to push on dat windows. Mutable borrow here.");
        self.windows.borrow_mut().push(window.clone());
        println!("window strong count is {}", Rc::strong_count(&window));
		println!("window weak count is {}", Rc::weak_count(&window));
        println!("definition strong count is {}", Rc::strong_count(&definition));
		println!("definition weak count is {}", Rc::weak_count(&definition));
        println!("about to initialize the window. Immutable borrow here.");
        window.borrow().initialize(definition, self.instance_handle, parent, show_immediately);
    } 
    fn register_class(&self, hinstance: HINSTANCE, hicon: HICON) -> bool {
        unsafe {
            let wc = WNDCLASSW {
                style: CS_DBLCLKS, // We want to receive double clicks
                lpfnWndProc: Some(Self::app_wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: hicon,
                hCursor: ptr::null_mut(),  // We manage the cursor ourselves
                hbrBackground: ptr::null_mut(), // Transparent
                lpszMenuName: ptr::null_mut(),
                lpszClassName: APP_WINDOW_CLASS.to_wide_null().as_ptr(),
            };

            if user32::RegisterClassW(&wc) == 0 {
                //ShowLastError();

                // @todo Slate: Error message should be localized!
                //FSlowHeartBeatScope SuspendHeartBeat;
                user32::MessageBoxW(
                    ptr::null_mut(),
                    "Window Registration Failed!".to_wide_null().as_ptr(),
                    "Error!".to_wide_null().as_ptr(),
                    MB_ICONEXCLAMATION | MB_OK
                );

                return false;
            }

            return true;
        }
    }
	pub fn get_window_transparency_support(&self) -> WindowTransparency {
        let mut is_composition_enabled = FALSE;
	    unsafe { super::DwmIsCompositionEnabled(&mut is_composition_enabled); }

	    if is_composition_enabled != 0 { WindowTransparency::PerPixel } else { WindowTransparency::PerWindow }
    }
     //TODO the return signature for this method feels wrong.
    pub fn find_window_by_hwnd(&self, /*windows_to_search: &Vec<Rc<RefCell<WindowsWindow>>>,*/ handle_to_find: HWND) -> Option<Rc<RefCell<WindowsWindow>>> {
        println!("Inside find_window_by_hwnd, handle_to_find is {:p}", handle_to_find);
        let len = self.windows.borrow().len();
        let mut n = 0;
        loop {
            if n == len { break; }
            let borrowed_window = &self.windows.borrow()[n];
            //println!("self.windows.borrow()[{}] is {:p} and {:#?}", n, borrowed_window, borrowed_window);
            //println!("borrowed_window.get_hwnd() is {:p}", borrowed_window.borrow().get_hwnd());
            if borrowed_window.borrow().get_hwnd() == handle_to_find {
                return Some(borrowed_window.clone());
            }
            n += 1;
        }
        None
    }
    pub fn is_cursor_directly_over_window(&self) -> bool {
        unsafe {
            let mut cursor_pos: POINT = mem::uninitialized();
            let got_point = user32::GetCursorPos(&mut cursor_pos);
            if got_point != 0 {
                let hwnd: HWND = user32::WindowFromPoint(cursor_pos);
                let window_under_cursor = self.find_window_by_hwnd(hwnd);
                return window_under_cursor.is_some();
            }
        }
        false
    }
    pub fn set_capture(&mut self, window: Rc<GenericWindow>) {
        //if ( InWindow.IsValid() )
        unsafe {
            user32::SetCapture(window.get_os_window_handle() as HWND);
        }
        /*else
        {
            ::ReleaseCapture();
        }
        */
    }
    pub fn get_capture(&self) -> HWND {
        unsafe { user32::GetCapture() }
    }
    pub fn set_high_precision_mouse_mode(&mut self, enable: bool, window: Rc<GenericWindow>) {
        unsafe {
            let mut hwnd: HWND = ptr::null_mut();
            let mut flags: DWORD = RIDEV_REMOVE;
            self.using_high_precision_mouse_input = enable;

            if enable {
                flags = 0;
                //if ( InWindow.IsValid() )
                hwnd = window.get_os_window_handle() as HWND;
            }
            let mut raw_input_device: RAWINPUTDEVICE = mem::uninitialized();
            //The HID standard for mouse
            let standard_mouse: u16 = 0x02;

            raw_input_device.usUsagePage = 0x01; 
            raw_input_device.usUsage = standard_mouse;
            raw_input_device.dwFlags = flags;

            // Process input for just the window that requested it.  NOTE: If we pass NULL here events are routed to the window with keyboard focus
            // which is not always known at the HWND level with Slate
            raw_input_device.hwndTarget = hwnd;

            // Register the raw input device
            user32::RegisterRawInputDevices(&raw_input_device, 1, mem::size_of::<RAWINPUTDEVICE>() as u32);
        }
    }
    pub fn get_work_area(&self, current_window: &PlatformRect) -> PlatformRect {
        let mut windows_window_dim: RECT = unsafe { mem::uninitialized() };
        windows_window_dim.left = current_window.left;
        windows_window_dim.top = current_window.top;
        windows_window_dim.right = current_window.right;
        windows_window_dim.bottom = current_window.bottom;

        unsafe {
            let best_monitor: HMONITOR = user32::MonitorFromRect(&mut windows_window_dim, MONITOR_DEFAULTTONEAREST);

            let mut monitor_info: MONITORINFO = mem::uninitialized();
            monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
            user32::GetMonitorInfoW(best_monitor, &mut monitor_info);

            let mut work_area: PlatformRect = mem::uninitialized();
            work_area.left = monitor_info.rcWork.left;
            work_area.top = monitor_info.rcWork.top;
            work_area.right = monitor_info.rcWork.right;
            work_area.bottom = monitor_info.rcWork.bottom;
            work_area
        }
    }
    pub fn process_message(&self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> i32 {
        println!("Reached inside process_message");
        unsafe {
            println!("hwnd is {:p}", hwnd);
            println!("WindowsApplication self address is {:p}", self);
            println!("self.windows.borrow().len() is {}. about to find window by hwnd.", self.windows.borrow().len());
            let mut current_native_event_window_opt = self.find_window_by_hwnd(hwnd);
            
            if self.windows.borrow().len() != 0 && current_native_event_window_opt.is_some() {
                println!("current_native_event_window_opt is some. Don't believe me? Look! {:#?}", current_native_event_window_opt);
                let mut current_native_event_window = current_native_event_window_opt.unwrap();

                match msg {
                    WM_GETMINMAXINFO => {
                        let mut min_max_info: MINMAXINFO = {
                            let mmi = mem::transmute::<LPARAM, *const MINMAXINFO>(lparam);
                            *mmi
                        };
                        let windef = current_native_event_window.borrow().get_definition();
                        let ref size_limits: WindowSizeLimits = windef.borrow().size_limits;

                        // We need to inflate the max values if using an OS window border
                        let mut border_width: i32 = 0;
                        let mut border_height: i32 = 0;
                        if windef.borrow().has_os_window_border {
                            let window_style = user32::GetWindowLongW(hwnd, GWL_STYLE);
                            let window_ex_style = user32::GetWindowLongW(hwnd, GWL_EXSTYLE);

                            // This adjusts a zero rect to give us the size of the border
                            let mut border_rect: RECT = mem::zeroed();
                            user32::AdjustWindowRectEx(&mut border_rect, window_style as u32, FALSE, window_ex_style as u32);

                            border_width = border_rect.right - border_rect.left;
                            border_height = border_rect.bottom - border_rect.top;
                        }

                        // We always apply BorderWidth and BorderHeight since Slate always works with client area window sizes
                        min_max_info.ptMinTrackSize.x = size_limits.get_min_width().unwrap_or(min_max_info.ptMinTrackSize.x as f32) as i32;
                        min_max_info.ptMinTrackSize.y = size_limits.get_min_height().unwrap_or(min_max_info.ptMinTrackSize.y as f32) as i32;
                        min_max_info.ptMaxTrackSize.x = size_limits.get_max_width().unwrap_or(min_max_info.ptMaxTrackSize.x as f32) as i32 + border_width;
                        min_max_info.ptMaxTrackSize.y = size_limits.get_max_height().unwrap_or(min_max_info.ptMaxTrackSize.y as f32) as i32 + border_height;
                        return 0;
                    },
                    WM_NCCALCSIZE => {
                        let windef = current_native_event_window.borrow().get_definition();
                        // Let windows absorb this message if using the standard border
                        if wparam != 0 && !windef.borrow().has_os_window_border {
                            // Borderless game windows are not actually borderless, they have a thick border that we simply draw game content over (client
                            // rect contains the window border). When maximized Windows will bleed our border over the edges of the monitor. So that we
                            // don't draw content we are going to later discard, we change a maximized window's size and position so that the entire
                            // window rect (including the border) sits inside the monitor. The size adjustments here will be sent to WM_MOVE and
                            // WM_SIZE and the window will still be considered maximized.
                            if windef.borrow().window_type == WindowType::GameWindow && current_native_event_window.borrow().is_maximized() {
                                // Ask the system for the window border size as this is the amount that Windows will bleed our window over the edge
                                // of our desired space. The value returned by current_native_event_window will be incorrect for our usage here as it
                                // refers to the border of the window that Slate should consider.
                                let mut window_info: WINDOWINFO = mem::zeroed();
                                window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                                super::GetWindowInfo(hwnd, &mut window_info);

                                // A pointer to the window size data that Windows will use is passed to us in lparam
                                let mut resizing_rects: NCCALCSIZE_PARAMS = {
                                    let calcparams = mem::transmute::<LPARAM, LPNCCALCSIZE_PARAMS>(lparam);
                                    *calcparams
                                };
                                // The first rectangle contains the client rectangle of the resized window. Decrease window size on all sides by
                                // the border size.
                                resizing_rects.rgrc[0].left += window_info.cxWindowBorders as i32;
                                resizing_rects.rgrc[0].top += window_info.cxWindowBorders as i32;
                                resizing_rects.rgrc[0].right -= window_info.cxWindowBorders as i32;
                                resizing_rects.rgrc[0].bottom -= window_info.cxWindowBorders as i32;
                                // The second rectangle contains the destination rectangle for the content currently displayed in the window's
                                // client rect. Windows will blit the previous client content into this new location to simulate the move of
                                // the window until the window can repaint itself. This should also be adjusted to our new window size.
                                resizing_rects.rgrc[1].left = resizing_rects.rgrc[0].left;
                                resizing_rects.rgrc[1].top = resizing_rects.rgrc[0].top;
                                resizing_rects.rgrc[1].right = resizing_rects.rgrc[0].right;
                                resizing_rects.rgrc[1].bottom = resizing_rects.rgrc[0].bottom;
                                // A third rectangle is passed in that contains the source rectangle (client area from window pre-maximize).
                                // It's value should not be changed.

                                // The new window position. Pull in the window on all sides by the width of the window border so that the
                                // window fits entirely on screen. We'll draw over these borders with game content.
                                (&mut *resizing_rects.lppos).x += window_info.cxWindowBorders as i32;
                                (&mut *resizing_rects.lppos).y += window_info.cxWindowBorders as i32;
                                (&mut *resizing_rects.lppos).cx -= 2 * window_info.cxWindowBorders as i32;
                                (&mut *resizing_rects.lppos).cy -= 2 * window_info.cxWindowBorders as i32;

                                // Informs Windows to use the values as we altered them.
                                return WVR_VALIDRECTS as i32;
                            }
                            return 0;
                        }
                    },
                    WM_SIZING => {
                        let windef = current_native_event_window.borrow().get_definition();
                        if windef.borrow().should_preserve_aspect_ratio {
                            // The rect we get in lparam is window rect, but we need to preserve client's aspect ratio,
                            // so we need to find what the border and title bar sizes are, if window has them and adjust the rect.
                            let mut window_info: WINDOWINFO = mem::zeroed();
                            window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                            super::GetWindowInfo(hwnd, &mut window_info);

                            let mut test_rect: RECT = mem::zeroed();
                            user32::AdjustWindowRectEx(&mut test_rect, window_info.dwStyle, FALSE, window_info.dwExStyle);

                            let mut rect: RECT = {
                                let lprect = mem::transmute::<LPARAM, *const RECT>(lparam);
                                *lprect
                            };

                            rect.left -= test_rect.left;
                            rect.right -= test_rect.right;
                            rect.top -= test_rect.top;
                            rect.bottom -= test_rect.bottom;

                            let aspect_ratio = current_native_event_window.borrow().get_aspect_ratio();
                            let new_width = rect.right - rect.left;
                            let new_height = rect.bottom - rect.top;

                            if wparam == WMSZ_LEFT as u64 || wparam == WMSZ_RIGHT as u64 {
                                let adjusted_height: i32 = new_width / aspect_ratio as i32;
                                rect.top -= (adjusted_height - new_height) / 2;
                                rect.bottom += (adjusted_height - new_height) / 2;
                                //break;
                            } else if wparam == WMSZ_TOP as u64 || wparam == WMSZ_BOTTOM as u64 {
                                let adjusted_width: i32 = new_height * aspect_ratio as i32;
                                rect.left -= (adjusted_width - new_width) / 2;
                                rect.right += (adjusted_width - new_width) / 2;
                                //break;
                            } else if wparam == WMSZ_TOPLEFT as u64 {
                                let adjusted_height: i32 = new_width / aspect_ratio as i32;
                                rect.top -= adjusted_height - new_height;
                                //break;
                            } else if wparam == WMSZ_TOPRIGHT as u64 {
                                let adjusted_height: i32 = new_width / aspect_ratio as i32;
                                rect.top -= adjusted_height - new_height;
                                //break;
                            } else if wparam == WMSZ_BOTTOMLEFT as u64 {
                                let adjusted_height: i32 = new_width / aspect_ratio as i32;
                                rect.bottom += adjusted_height - new_height;
                                //break;
                            } else if wparam == WMSZ_BOTTOMRIGHT as u64 {
                                let adjusted_height: i32 = new_width / aspect_ratio as i32;
                                rect.bottom += adjusted_height - new_height;
                                //break;
                            }

                            user32::AdjustWindowRectEx(&mut rect, window_info.dwStyle, FALSE, window_info.dwExStyle);

                            return TRUE;
                        }
                    },
                    WM_DESTROY => {
                        println!("about to delete references to windows after WM_DESTROY. Mutable borrow here.");
                        self.windows.borrow_mut().retain(|ref x| Rc::ptr_eq(*x, &current_native_event_window) == false);
                        return 0;
                    },
                    _ => {}
                }
            }
            user32::DefWindowProcW(hwnd, msg, wparam, lparam) as i32
        }
    }
    /*void FWindowsApplication::GetInitialDisplayMetrics( FDisplayMetrics& OutDisplayMetrics ) const {
        OutDisplayMetrics = InitialDisplayMetrics;
    }*/
    fn is_keyboard_input_message(&self, msg: u32) -> bool {
        match msg {
            // Keyboard input notification messages...
            WM_CHAR | WM_SYSCHAR | WM_SYSKEYDOWN | WM_KEYDOWN | WM_SYSKEYUP | WM_KEYUP | WM_SYSCOMMAND => { true },
            _ => { false }
        }
    }
    fn is_mouse_input_message(&self, msg: u32) -> bool {
        match msg {
            // Mouse input notification messages...
            WM_MOUSEHWHEEL | WM_MOUSEWHEEL | WM_MOUSEHOVER | WM_MOUSELEAVE | WM_MOUSEMOVE | WM_NCMOUSEHOVER | WM_NCMOUSELEAVE | WM_NCMOUSEMOVE |
            WM_NCMBUTTONDBLCLK | WM_NCMBUTTONDOWN | WM_NCMBUTTONUP | WM_NCRBUTTONDBLCLK | WM_NCRBUTTONDOWN | WM_NCRBUTTONUP | WM_NCXBUTTONDBLCLK |
            WM_NCXBUTTONDOWN | WM_NCXBUTTONUP | WM_LBUTTONDBLCLK | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONDBLCLK | WM_MBUTTONDOWN | WM_MBUTTONUP |
            WM_RBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP | WM_XBUTTONDBLCLK | WM_XBUTTONDOWN | WM_XBUTTONUP => { true },
            _ => { false }
        }
    }
    fn is_input_message(&self, msg: u32) -> bool {
        if self.is_keyboard_input_message(msg) || self.is_mouse_input_message(msg) {
            return true;
        }

        match msg {
            // Raw input notification messages...
            WM_INPUT | WM_INPUT_DEVICE_CHANGE => { true },
            _ => { false }
        }
    }
    fn query_connected_mice(&mut self) {
        unsafe {
            let mut device_count: u32 = 0;

            user32::GetRawInputDeviceList(ptr::null_mut(), &mut device_count, mem::size_of::<RAWINPUTDEVICELIST>() as u32);
            if device_count == 0 {
                self.is_mouse_attached = false;
                return;
            }

            let mut device_list: Vec<RAWINPUTDEVICELIST> = Vec::with_capacity(device_count as usize);
            user32::GetRawInputDeviceList(device_list.as_mut_ptr() as *mut RAWINPUTDEVICELIST, &mut device_count, mem::size_of::<RAWINPUTDEVICELIST>() as u32);
    
            let mut mouse_count: i32 = 0;
            for device in device_list {
                let mut name_len: u32 = 0;
                let mut name: Vec<u8> = vec![];
                if device.dwType != RIM_TYPEMOUSE {
                    continue;
                }
                //Force the use of ANSI versions of these calls
                let ret = user32::GetRawInputDeviceInfoA(device.hDevice, RIDI_DEVICENAME, ptr::null_mut(), &mut name_len);
                if ret as i32 == -1 {
                    continue;
                }

                name.resize(name_len as usize + 1, 0);
                let ret = user32::GetRawInputDeviceInfoA(device.hDevice, RIDI_DEVICENAME, name.as_mut_ptr() as *mut c_void, &mut name_len);
                if ret as i32 == -1 {
                    continue;
                }

                let mut name_str = String::from_utf8(name).unwrap();
                let replacement = name_str.replace("#", "\\");
                /*
                * Name XP starts with \??\, vista+ starts \\?\ 
                * In the device list exists a fake Mouse device with the device name of RDP_MOU
                * This is used for Remote Desktop so ignore it.
                */
                if replacement.starts_with("\\??\\ROOT\\RDP_MOU\\") || replacement.starts_with("\\\\?\\ROOT\\RDP_MOU\\") {
                    continue;
                }

                mouse_count += 1;
            }

            self.is_mouse_attached = mouse_count > 0;
        }
    }
    /*fn update_all_modifier_key_states(&mut self) {
        unsafe { 
            self.modifier_key_state[ModifierKey::LeftShift as usize]       = (user32::GetAsyncKeyState(VK_LSHIFT) & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::RightShift as usize]      = (user32::GetAsyncKeyState(VK_RSHIFT) & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::LeftControl as usize]     = (user32::GetAsyncKeyState(VK_LCONTROL) & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::RightControl as usize]    = (user32::GetAsyncKeyState(VK_RCONTROL) & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::LeftAlt as usize]         = (user32::GetAsyncKeyState(VK_LMENU) & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::RightAlt as usize]        = (user32::GetAsyncKeyState(VK_RMENU) & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::CapsLock as usize]        = (user32::GetKeyState(VK_CAPITAL) & 0x0001) != 0;
        }
    }*/
    pub fn pump_messages(&self, time_delta: f32) {
        unsafe {
            let mut message: MSG = mem::zeroed();

            // standard Windows message handling
            while user32::PeekMessageW(&mut message, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 { 
                user32::TranslateMessage(&message);
                user32::DispatchMessageW(&message); 
            }
            println!("In pump_messages, return value was 0");
        }
    }
    unsafe extern "system" fn app_wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        println!("in app_wnd_proc, message is {0}, {:#06x} ({})", msg, if let Some(msg_str) = super::WINDOWS_MESSAGE_STRINGS.get(&msg) { msg_str } else { "Not found" });
        WINDOWS_APPLICATION.unwrap().process_message(hwnd, msg, wparam, lparam) as i64
    }
}

fn get_monitor_size_from_edid(h_dev_reg_key: RegKey, out_width: &mut i32, out_height: &mut i32) -> bool {
    for (name, value) in h_dev_reg_key.enum_values().map(|x| x.unwrap()) {
        if &name[..] != "EDID" {
            continue;
        } else {
            // EDID data format documented here:
            // http://en.wikipedia.org/wiki/EDID

            let detail_timing_descriptor_start_index: usize = 54;
            
            *out_width = ((value.bytes[detail_timing_descriptor_start_index + 4] as i32 >> 4) << 8) | value.bytes[detail_timing_descriptor_start_index + 2] as i32;
            *out_height = ((value.bytes[detail_timing_descriptor_start_index + 7] as i32 >> 4) << 8) | value.bytes[detail_timing_descriptor_start_index + 5] as i32;

            return true; // valid EDID found
        }
    }

    false
}

fn get_size_for_dev_id(target_dev_id: &String, width: &mut i32, height: &mut i32) -> bool {
    unsafe {
        DEFINE_GUID!(GUID_DEVCLASS_MONITOR, 0x4d36e96e, 0xe325, 0x11ce, 0xbf, 0xc1, 0x08, 0x00, 0x2b, 0xe1, 0x03, 0x18);
        let dev_info: HDEVINFO = setupapi::SetupDiGetClassDevsExW(
            &GUID_DEVCLASS_MONITOR, //class GUID
            ptr::null_mut(),
            ptr::null_mut(),
            DIGCF_PRESENT,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut()
        );

        if dev_info.is_null() {
            return false;
        }

        let mut res: bool = false;
        let mut monitor_index = 0;
        loop {
            let err = Error::last_os_error();
            if err.raw_os_error().unwrap() == ERROR_NO_MORE_ITEMS as i32 { break; }
            let mut dev_info_data: SP_DEVINFO_DATA = mem::uninitialized();
            dev_info_data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

            if setupapi::SetupDiEnumDeviceInfo(dev_info, monitor_index, &mut dev_info_data) == TRUE {
                let mut buffer = [0u16; MAX_DEVICE_ID_LEN];
                if setupapi::CM_Get_Device_IDW(dev_info_data.DevInst, buffer.as_mut_ptr(), MAX_DEVICE_ID_LEN as u32, 0) == CR_SUCCESS {

                    let mut dev_id = String::from_utf16_lossy(&buffer[..]);
                    let idx = &dev_id[9..].find("\\").unwrap();
                    dev_id = dev_id[8..9 + *idx].to_string();
                    if &dev_id[..] == &target_dev_id[..] {
                        let h_dev_reg_key: HKEY = setupapi::SetupDiOpenDevRegKey(dev_info, &mut dev_info_data, DICS_FLAG_GLOBAL, 0, DIREG_DEV, KEY_READ);

                        if !h_dev_reg_key.is_null() && h_dev_reg_key != INVALID_HANDLE_VALUE as HKEY {
                            res = get_monitor_size_from_edid(RegKey::predef(h_dev_reg_key), width, height);
                            //advapi32::RegCloseKey(h_dev_reg_key);
                            break;
                        }
                    }
                }
            }
            monitor_index += 1;
        }

        if setupapi::SetupDiDestroyDeviceInfoList(dev_info) == FALSE {
            res = false;
        }

        res
    }
}

fn get_monitor_info(out_monitor_info: &mut Vec<MonitorInfo>) {
    unsafe {
        let mut display_device: DISPLAY_DEVICEW = mem::uninitialized();
        display_device.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
        let mut device_index: DWORD = 0; // device index

        let mut primary_device: *mut MonitorInfo = ptr::null_mut();
        out_monitor_info.reserve(2); // Reserve two slots, as that will be the most common maximum
        
        while user32::EnumDisplayDevicesW(0 as *const u16, device_index, &mut display_device, 0) != 0 {
            if display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP > 0 {
                let mut monitor: DISPLAY_DEVICEW = mem::uninitialized();
                monitor.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
                let mut monitor_index: DWORD = 0;

                while user32::EnumDisplayDevicesW(display_device.DeviceName.as_ptr(), monitor_index, &mut monitor, 0) != 0 {
                    if (monitor.StateFlags & DISPLAY_DEVICE_ACTIVE != 0) && (monitor.StateFlags & DISPLAY_DEVICE_MIRRORING_DRIVER == 0) {
                        let mut info: MonitorInfo = mem::zeroed();

                        let temp_str = String::from_utf16_lossy(&monitor.DeviceID[..]);
                        let idx = &temp_str[9..].find("\\").unwrap();
                        info.name += &temp_str[8..9 + *idx];
                        info.id = temp_str;
                        //info.name = info.id.Mid (8, Info.ID.Find (TEXT("\\"), ESearchCase::CaseSensitive, ESearchDir::FromStart, 9) - 8);

                        if get_size_for_dev_id(&info.name, &mut info.native_width, &mut info.native_height) {
                            let temp_str = String::from_utf16_lossy(&monitor.DeviceID[..]);
                            info.id = temp_str.trim_right_matches('\0').to_string();
                            info.is_primary = (display_device.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE) > 0;
                            out_monitor_info.push(info);

                            let len = out_monitor_info.len();
                            if primary_device.is_null() && out_monitor_info[len - 1].is_primary {
                                primary_device = &mut out_monitor_info[len - 1];
                            }
                        }
                    }
                    monitor_index += 1;

                    monitor = mem::zeroed();
                    monitor.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
                }
            }

            display_device = mem::zeroed();
            display_device.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
            device_index += 1;
        }
    }
}

//TODO this struct has cross-platform applications, so it shouldn't be implemented within the Windows-specific files.
#[derive(PartialEq, Debug, Clone)]
pub struct DisplayMetrics {
    primary_display_width: i32,
    primary_display_height: i32,
    monitor_info: Vec<MonitorInfo>,
    pub primary_display_work_area_rect: PlatformRect,
    pub virtual_display_rect: PlatformRect,
    //TODO: The following should be a Vector2D
    title_safe_padding_size: (f32, f32),
    //TODO: The following should be a Vector2D
    action_safe_padding_size: (f32, f32),
}

impl DisplayMetrics {
    pub fn new() -> DisplayMetrics {
        unsafe {
            let mut out_display_metrics: DisplayMetrics = mem::zeroed();
            // Total screen size of the primary monitor
            out_display_metrics.primary_display_width = user32::GetSystemMetrics(SM_CXSCREEN);
            out_display_metrics.primary_display_height = user32::GetSystemMetrics(SM_CYSCREEN);

            // Get the screen rect of the primary monitor, excluding taskbar etc.
            let mut work_area_rect: RECT = mem::zeroed();
            if user32::SystemParametersInfoW(SPI_GETWORKAREA, 0, mem::transmute(&mut work_area_rect), 0) == 0 {
                work_area_rect.top = 0;
                work_area_rect.bottom = 0;
                work_area_rect.left = 0;
                work_area_rect.right = 0;
            }

            out_display_metrics.primary_display_work_area_rect.left = work_area_rect.left;
            out_display_metrics.primary_display_work_area_rect.top = work_area_rect.top;
            out_display_metrics.primary_display_work_area_rect.right = work_area_rect.right;
            out_display_metrics.primary_display_work_area_rect.bottom = work_area_rect.bottom;
    
            // Virtual desktop area
            out_display_metrics.virtual_display_rect.left = user32::GetSystemMetrics(SM_XVIRTUALSCREEN);
            out_display_metrics.virtual_display_rect.top = user32::GetSystemMetrics(SM_YVIRTUALSCREEN);
            out_display_metrics.virtual_display_rect.right = out_display_metrics.virtual_display_rect.left + user32::GetSystemMetrics(SM_CXVIRTUALSCREEN);
            out_display_metrics.virtual_display_rect.bottom = out_display_metrics.virtual_display_rect.top + user32::GetSystemMetrics(SM_CYVIRTUALSCREEN);

            // Get connected monitor information
            get_monitor_info(&mut out_display_metrics.monitor_info);

            // Apply the debug safe zones
            out_display_metrics.apply_default_safe_zones();
            out_display_metrics
        }
    }
    pub fn get_debug_title_safe_zone_ratio(&self) -> f32 {
        unsafe { DEBUG_SAFE_ZONE_RATIO }
    }
    pub fn get_debug_action_safe_zone_ratio(&self) -> f32 {
        unsafe { DEBUG_ACTION_ZONE_RATIO }
    }
    pub fn apply_default_safe_zones(&mut self) {
        let safe_zone_ratio = self.get_debug_title_safe_zone_ratio();
        if safe_zone_ratio < 1.0 {
            let half_unsafe_ratio = (1.0 - safe_zone_ratio) * 0.5;
            self.title_safe_padding_size = (self.primary_display_width as f32 * half_unsafe_ratio, self.primary_display_height as f32 * half_unsafe_ratio);
        }

        let action_safe_zone_ratio = self.get_debug_action_safe_zone_ratio();
        if action_safe_zone_ratio < 1.0 {
            let half_unsafe_ratio = (1.0 - action_safe_zone_ratio) * 0.5;
            self.action_safe_padding_size = (self.primary_display_width as f32 * half_unsafe_ratio, self.primary_display_height as f32 * half_unsafe_ratio);
        }
    }
}