use cgmath::Point2;
use dwmapi;
use platform::generic::application::{MonitorInfo, PlatformRect, DEBUG_ACTION_ZONE_RATIO, DEBUG_SAFE_ZONE_RATIO};
use platform::generic::application_message_handler::WindowZone;
use platform::generic::cursor::ICursor;
use platform::generic::window::GenericWindow;
use platform::generic::window_definition::{WindowDefinition, WindowTransparency, WindowType};
use platform::windows::window::WindowsWindow;
use setupapi;
use std::collections::BTreeMap;
use std::io::Error;
use std::os::raw::c_void;
use std::{mem, ptr};
use std::rc::{Rc, Weak};
use user32;
use uuid::GUID_DEVCLASS_MONITOR;
use winapi::{
    c_short, CR_SUCCESS, CS_DBLCLKS, DICS_FLAG_GLOBAL, DIGCF_PRESENT, DIREG_DEV, DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP, DISPLAY_DEVICE_MIRRORING_DRIVER,
    DISPLAY_DEVICE_PRIMARY_DEVICE, DISPLAY_DEVICEW, DLGC_WANTALLKEYS, DWORD, ERROR_NO_MORE_ITEMS, FALSE, GUID, GWL_STYLE, GWL_EXSTYLE, HDEVINFO, HICON, HINSTANCE, HIWORD, HKEY, HMONITOR,
    HRAWINPUT, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION, HTCLIENT, HTCLOSE, HTLEFT, HTMINBUTTON, HTMAXBUTTON, HTNOWHERE, HTRIGHT, HTSYSMENU, HTTOP, HTTOPLEFT, HTTOPRIGHT, HWND,
    IMN_CHANGECANDIDATE, IMN_CLOSECANDIDATE, IMN_CLOSESTATUSWINDOW, IMN_GUIDELINE, IMN_OPENCANDIDATE, IMN_OPENSTATUSWINDOW, IMN_PRIVATE, IMN_SETCANDIDATEPOS,
    IMN_SETCOMPOSITIONFONT, IMN_SETCOMPOSITIONWINDOW, IMN_SETCONVERSIONMODE, IMN_SETOPENSTATUS, IMN_SETSENTENCEMODE, IMN_SETSTATUSWINDOWPOS, IMR_CANDIDATEWINDOW, IMR_COMPOSITIONFONT,
    IMR_COMPOSITIONWINDOW, IMR_CONFIRMRECONVERTSTRING, IMR_DOCUMENTFEED, IMR_QUERYCHARPOSITION, IMR_RECONVERTSTRING, INVALID_HANDLE_VALUE, KEY_READ, LOWORD, LPARAM, LPNCCALCSIZE_PARAMS,
    LPVOID, LPWSTR, LRESULT,
    MAX_DEVICE_ID_LEN, MINMAXINFO, MONITORINFO, MONITOR_DEFAULTTONEAREST, MOUSE_MOVE_ABSOLUTE, MOUSE_MOVE_RELATIVE, MSG, NCCALCSIZE_PARAMS, PM_REMOVE, POINT, POINTL, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER,
    RECT, RIDEV_REMOVE, RID_INPUT, RIM_TYPEMOUSE, SM_CXSCREEN,
    SM_CYSCREEN, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SP_DEVINFO_DATA, SPI_GETWORKAREA, SW_RESTORE, TRUE, VK_F4, VK_SPACE, WINDOWINFO, WM_IME_CHAR,
    WM_IME_COMPOSITION, WM_IME_ENDCOMPOSITION, WM_IME_NOTIFY, WM_IME_REQUEST, WM_IME_SETCONTEXT, WM_IME_STARTCOMPOSITION, WM_INPUTLANGCHANGE, WM_INPUTLANGCHANGEREQUEST, WM_KEYDOWN, WM_KEYUP,
    WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_NCLBUTTONDOWN, WM_NCMBUTTONDOWN, WM_NCMOUSEMOVE,
    WM_NCRBUTTONDOWN, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP, WMSZ_BOTTOM, WMSZ_BOTTOMLEFT, WMSZ_BOTTOMRIGHT, WMSZ_LEFT,
    WMSZ_RIGHT, WMSZ_TOP, WMSZ_TOPLEFT, WMSZ_TOPRIGHT, WNDCLASSW, WPARAM, WVR_VALIDRECTS
};
use winreg::RegKey;

type IntPoint2 = Point2<i32>;

static mut windows_application: *mut WindowsApplication = ptr::null_mut();

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
//Actually, this will most likely remain a trait whose method wraps the real ProcessMessage method, since the real method needs an ABI signature that is not compatible with Rust's method
//call signatures.
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
	minimized_window_position: IntPoint2,
	instance_handle: HINSTANCE,
    using_high_precision_mouse_input: bool,
    is_mouse_attached: bool,
    force_activate_by_mouse: bool,
    deferred_messages: Vec<DeferredWindowsMessage>,
    deferred_drag_drop_operations: Vec<DeferredWindowsDragDropOperation>,
    message_handlers: Vec<Box<IWindowsMessageHandler>>,
    windows: Vec<Rc<WindowsWindow>>,
    in_modal_size_loop: bool,
    allowed_to_defer_message_processing: bool,
}

impl WindowsApplication {
    pub fn create_windows_application(hinstance: HINSTANCE, hicon: HICON) -> *mut WindowsApplication {
        let app = WindowsApplication::new(hinstance, hicon);
        windows_application = *mut app;
        windows_application
    }
    pub fn new(hinstance: HINSTANCE, hicon: HICON) -> WindowsApplication {
        unsafe {
            let win_app: WindowsApplication = mem::uninitialized();
            win_app
        }
    }
    fn register_class(&self, hinstance: HINSTANCE, hicon: HICON) -> bool {
        unsafe {
            let mut wc: WNDCLASSW = mem::zeroed();
            wc.style = CS_DBLCLKS; // We want to receive double clicks
            wc.lpfnWndProc = Some(app_wnd_proc);
            wc.cbClsExtra = 0;
            wc.cbWndExtra = 0;
            wc.hInstance = hinstance;
            wc.hIcon = hicon;
            wc.hCursor = ptr::null_mut();  // We manage the cursor ourselves
            wc.hbrBackground = ptr::null_mut(); // Transparent
            wc.lpszMenuName = ptr::null_mut();
            wc.lpszClassName = WindowsWindow::app_window_class;
        }
    }
	pub fn get_window_transparency_support(&self) -> WindowTransparency {
        let mut is_composition_enabled = FALSE;
	    unsafe { dwmapi::DwmIsCompositionEnabled(&mut is_composition_enabled); }

	    if is_composition_enabled != 0 { WindowTransparency::PerPixel } else { WindowTransparency::PerWindow }
    }
     //TODO the return signature for this method feels wrong.
    pub fn find_window_by_hwnd(&self, windows_to_search: &Vec<Rc<WindowsWindow>>, handle_to_find: HWND) -> Option<Rc<WindowsWindow>> {
        for window in windows_to_search {
            if window.get_hwnd() == handle_to_find {
                return Some((*window).clone());
            }
        }
        None
    }
    pub fn is_cursor_directly_over_window(&self) -> bool {
        unsafe {
            let mut cursor_pos: POINT = mem::uninitialized();
            let got_point = user32::GetCursorPos(&mut cursor_pos);
            if got_point != 0 {
                let hwnd: HWND = user32::WindowFromPoint(cursor_pos);
                let window_under_cursor = self.find_window_by_hwnd(&self.windows, hwnd);
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
        let mut current_native_event_window_opt = self.find_window_by_hwnd(&self.windows, hwnd);

        if self.windows.len() != 0 && current_native_event_window_opt.is_some() {
            let mut current_native_event_window = current_native_event_window_opt.unwrap();

            match msg {
                WM_INPUTLANGCHANGEREQUEST |
                WM_INPUTLANGCHANGE |
                WM_IME_SETCONTEXT |
                WM_IME_STARTCOMPOSITION |
                WM_IME_COMPOSITION |
                WM_IME_ENDCOMPOSITION |
                WM_IME_CHAR => {
                    //UE_LOG(LogWindowsDesktop, Verbose, TEXT("%s"), *(WindowsMessageStrings[msg]));
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    return 0;
                },
                WM_IME_NOTIFY => {
                    //UE_LOG(LogWindowsDesktop, Verbose, TEXT("WM_IME_NOTIFY - %s"), IMNStrings.Find(wparam) ? *(IMNStrings[wparam]) : nullptr);
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    return 0;
                },
                WM_IME_REQUEST => {
                    //UE_LOG(LogWindowsDesktop, Verbose, TEXT("WM_IME_REQUEST - %s"), IMRStrings.Find(wparam) ? *(IMRStrings[wparam]) : nullptr);
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    return 0;
                },
                // Character
                WM_CHAR => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    return 0;
                },
                WM_SYSCHAR => {
                    if HIWORD(lparam as u32) & 0x2000 != 0 && wparam == VK_SPACE as u64 {
                        // Do not handle Alt+Space so that it passes through and opens the window system menu
                        //break;
                    } else {
                        return 0;
                    }
                },
                WM_SYSKEYDOWN => {
                    // Alt-F4 or Alt+Space was pressed. 
                    // Allow alt+f4 to close the window and alt+space to open the window menu
                    if wparam != VK_F4 as u64 && wparam != VK_SPACE as u64 {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    }
                },
                WM_KEYDOWN |
                WM_SYSKEYUP |
                WM_KEYUP |
                WM_LBUTTONDBLCLK |
                WM_LBUTTONDOWN |
                WM_MBUTTONDBLCLK |
                WM_MBUTTONDOWN |
                WM_RBUTTONDBLCLK |
                WM_RBUTTONDOWN |
                WM_XBUTTONDBLCLK |
                WM_XBUTTONDOWN |
                WM_XBUTTONUP |
                WM_LBUTTONUP |
                WM_MBUTTONUP |
                WM_RBUTTONUP |
                WM_NCMOUSEMOVE |
                WM_MOUSEMOVE |
                WM_MOUSEWHEEL => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    // Handled
                    return 0;
                },
                WM_SETCURSOR => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);

                    // If we're rendering our own window border, we'll "handle" this event so that Windows doesn't try to manage the cursor
                    // appearance for us in the non-client area.  However, for OS window borders we need to fall through to DefWindowProc to
                    // allow Windows to draw the resize cursor
                    if !current_native_event_window.get_definition().has_os_window_border {
                        // Handled
                        return 0;
                    }
                },
                // Mouse Movement
                WM_INPUT => {
                    let mut size: u32 = 0;
                    unsafe {
                        user32::GetRawInputData(lparam as HRAWINPUT, RID_INPUT, ptr::null_mut(), &mut size, mem::size_of::<RAWINPUTHEADER>() as u32);
                    }

                    let raw = unsafe {
                        let mut raw = mem::uninitialized::<RAWINPUT>();
                        assert!(user32::GetRawInputData(lparam as HRAWINPUT, RID_INPUT, ((&mut raw) as *mut RAWINPUT) as LPVOID, &mut size, mem::size_of::<RAWINPUTHEADER>() as u32) == size);
                        raw
                    };
                    let raw_mouse = unsafe { raw.mouse() };

                    if raw.header.dwType == RIM_TYPEMOUSE {
                        let is_absolute_input = (raw_mouse.usFlags & MOUSE_MOVE_ABSOLUTE) == MOUSE_MOVE_ABSOLUTE;
                        if is_absolute_input {
                            // Since the raw input is coming in as absolute it is likely the user is using a tablet
                            // or perhaps is interacting through a virtual desktop
                            self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, MOUSE_MOVE_ABSOLUTE as u32);
                            return 1;
                        }

                        // Since raw input is coming in as relative it is likely a traditional mouse device
                        let x_pos_relative = raw_mouse.lLastX;
                        let y_pos_relative = raw_mouse.lLastY;

                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, x_pos_relative, y_pos_relative, MOUSE_MOVE_RELATIVE as u32);
                        return 1;
                    }
                },
                WM_NCCALCSIZE => {
                    // Let windows absorb this message if using the standard border
                    if wparam != 0 && !current_native_event_window.get_definition().has_os_window_border {
                        // Borderless game windows are not actually borderless, they have a thick border that we simply draw game content over (client
                        // rect contains the window border). When maximized Windows will bleed our border over the edges of the monitor. So that we
                        // don't draw content we are going to later discard, we change a maximized window's size and position so that the entire
                        // window rect (including the border) sits inside the monitor. The size adjustments here will be sent to WM_MOVE and
                        // WM_SIZE and the window will still be considered maximized.
                        if current_native_event_window.get_definition().window_type == WindowType::GameWindow && current_native_event_window.is_maximized() {
                            // Ask the system for the window border size as this is the amount that Windows will bleed our window over the edge
                            // of our desired space. The value returned by current_native_event_window will be incorrect for our usage here as it
                            // refers to the border of the window that Slate should consider.
                            let mut window_info: WINDOWINFO = mem::zeroed();
                            window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                            user32::GetWindowInfo(hwnd, &mut window_info);

                            // A pointer to the window size data that Windows will use is passed to us in lparam
                            let mut resizing_rects: NCCALCSIZE_PARAMS = unsafe {
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
                            (&*resizing_rects.lppos).x += window_info.cxWindowBorders as i32;
                            (&*resizing_rects.lppos).y += window_info.cxWindowBorders as i32;
                            (&*resizing_rects.lppos).cx -= 2 * window_info.cxWindowBorders as i32;
                            (&*resizing_rects.lppos).cy -= 2 * window_info.cxWindowBorders as i32;

                            // Informs Windows to use the values as we altered them.
                            return WVR_VALIDRECTS as i32;
                        }
                        return 0;
                    }
                },
                //break;
                WM_SHOWWINDOW => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                WM_SIZE => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    return 0;
                },
                //break;
                WM_SIZING => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
        
                    if current_native_event_window.get_definition().should_preserve_aspect_ratio {
                        // The rect we get in lparam is window rect, but we need to preserve client's aspect ratio,
                        // so we need to find what the border and title bar sizes are, if window has them and adjust the rect.
                        let mut window_info: WINDOWINFO = mem::zeroed();
                        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                        user32::GetWindowInfo(hwnd, &mut window_info);

                        let mut test_rect: RECT = mem::zeroed();
                        user32::AdjustWindowRectEx(&mut test_rect, window_info.dwStyle, FALSE, window_info.dwExStyle);

                        let rect: RECT = unsafe {
                            let lprect = mem::transmute::<LPARAM, *const RECT>(lparam);
                            *lprect
                        };

                        rect.left -= test_rect.left;
                        rect.right -= test_rect.right;
                        rect.top -= test_rect.top;
                        rect.bottom -= test_rect.bottom;

                        let aspect_ratio = current_native_event_window.get_aspect_ratio();
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
                }
                //break;
                WM_ENTERSIZEMOVE => {
                    self.in_modal_size_loop = true;
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                WM_EXITSIZEMOVE => {
                    self.in_modal_size_loop = false;
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                WM_MOVE => {
                    // client area position
                    let new_x = LOWORD(lparam as u32) as c_short as i32;
                    let new_y = HIWORD(lparam as u32) as c_short as i32;
                    let new_position: IntPoint2 = IntPoint2::new(new_x, new_y);

                    // Only cache the screen position if its not minimized
                    if FWindowsApplication::MinimizedWindowPosition != new_position {
                        MessageHandler->OnMovedWindow(current_native_event_window, new_x, new_y);

                        return 0;
                    }
                },
                //break;
                WM_NCHITTEST => {
                    // Only needed if not using the os window border as this is determined automatically
                    if !current_native_event_window.get_definition().has_os_window_border {
                        let rc_window: RECT = mem::uninitialized();
                        user32::GetWindowRect(hwnd, &mut rc_window);

                        let local_mouse_x = LOWORD(lparam as u32) as c_short as i32 - rc_window.left;
                        let local_mouse_y = HIWORD(lparam as u32) as c_short as i32 - rc_window.top;
                        if current_native_event_window.is_regular_window() {
                            let mut zone: WindowZone = mem::uninitialized();
                    
                            if MessageHandler->ShouldProcessUserInputMessages(current_native_event_window) {
                                // Assumes this is not allowed to leave Slate or touch rendering
                                zone = MessageHandler->GetWindowZoneForPoint(current_native_event_window, local_mouse_x, local_mouse_y);
                            } else {
                                // Default to client area so that we are able to see the feedback effect when attempting to click on a non-modal window when a modal window is active
                                // Any other window zones could have side effects and NotInWindow prevents the feedback effect.
                                zone = WindowZone::ClientArea;
                            }

                            return HIT_RESULTS[zone];
                        }
                    }
                },
                //break;
                WM_DWMCOMPOSITIONCHANGED => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                // Window focus and activation
                WM_MOUSEACTIVATE => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                // Window focus and activation
                WM_ACTIVATE => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                WM_ACTIVATEAPP => {
                    // When window activation changes we are not in a modal size loop
                    self.in_modal_size_loop = false;
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                },
                //break;
                WM_SETTINGCHANGE => {
                    // Convertible mode change
                    let lparam_str: LPWSTR = unsafe {
                        let lstr = mem::transmute::<LPARAM, LPWSTR>(lparam);
                        lstr
                    };
                    if !lparam_str.is_null() {
                        let mut length_counter = 0;
                        loop {
                            if *lparam_str.offset(length_counter) == 0 {
                                break;
                            }
                            length_counter += 1;
                        }
                        length_counter += 1;
                        let u16_str: Vec<u16> = Vec::from_raw_parts(lparam_str, length_counter as usize, length_counter as usize);
                        let setting_str = String::from_utf16_lossy(&u16_str[..]);
                        if &setting_str[..] == "ConvertibleSlateMode" {
                            self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        }
                    }
                },
                //break;
                WM_PAINT => {
                    if self.in_modal_size_loop && IsInGameThread() {
                        MessageHandler->OnOSPaint(current_native_event_window.ToSharedRef());
                    }
                },
                //break;
                WM_ERASEBKGND => {
                    // Intercept background erasing to eliminate flicker.
                    // Return non-zero to indicate that we'll handle the erasing ourselves
                    return 1;
                },
                //break;
                WM_NCACTIVATE => {
                    if !current_native_event_window.get_definition().has_os_window_border {
                        // Unless using the OS window border, intercept calls to prevent non-client area drawing a border upon activation or deactivation
                        // Return true to ensure standard activation happens
                        return TRUE;
                    }
                },
                //break;
                WM_NCPAINT => {
                    if !current_native_event_window.get_definition().has_os_window_border {
                        // Unless using the OS window border, intercept calls to draw the non-client area - we do this ourselves
                        return 0;
                    }
                },
                //break;
                WM_DESTROY => {
                    self.windows.remove(current_native_event_window);
                    return 0;
                },
                //break;
                WM_CLOSE => {
                    self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                    return 0;
                },
                //break;
                WM_SYSCOMMAND => {
                    match wparam & 0xfff0 {
                        SC_RESTORE => {
                            // Checks to see if the window is minimized.
                            if user32::IsIconic(hwnd) != 0 {
                                // This is required for restoring a minimized fullscreen window
                                user32::ShowWindow(hwnd, SW_RESTORE);
                                return 0;
                            } else {
                                if !MessageHandler->OnWindowAction( current_native_event_window, WindowAction::Restore) {
                                    return 1;
                                }
                            }
                        },
                        //break;
                        SC_MAXIMIZE => {
                            if !MessageHandler->OnWindowAction( current_native_event_window, WindowAction::Maximize) {
                                return 1;
                            }
                        },
                        //break;
                        default => {
                            if !MessageHandler->ShouldProcessUserInputMessages( current_native_event_window ) && IsInputMessage( msg ) {
                                return 0;
                            }
                        }
                        //break;
                    }
                },
                //break;
                WM_GETMINMAXINFO => {
                    let mut min_max_info: MINMAXINFO = unsafe {
                        let mmi = mem::transmute::<LPARAM, *const MINMAXINFO>(lparam);
                        *mmi
                    };
                    FWindowSizeLimits SizeLimits = MessageHandler->GetSizeLimitsForWindow(current_native_event_window);

                    // We need to inflate the max values if using an OS window border
                    let mut border_width: i32 = 0;
                    let mut border_height: i32 = 0;
                    if current_native_event_window.get_definition().has_os_window_border {
                        let window_style = user32::GetWindowLongW(hwnd, GWL_STYLE);
                        let window_ex_style = user32::GetWindowLongW(hwnd, GWL_EXSTYLE);

                        // This adjusts a zero rect to give us the size of the border
                        let mut border_rect: RECT = mem::zeroed();
                        user32::AdjustWindowRectEx(&mut border_rect, window_style, FALSE, window_ex_style);

                        border_width = border_rect.right - border_rect.left;
                        border_height = border_rect.bottom - border_rect.top;
                    }

                    // We always apply BorderWidth and BorderHeight since Slate always works with client area window sizes
                    min_max_info.ptMinTrackSize.x = FMath::RoundToInt( SizeLimits.GetMinWidth().Get(min_max_info.ptMinTrackSize.x));
                    min_max_info.ptMinTrackSize.y = FMath::RoundToInt( SizeLimits.GetMinHeight().Get(min_max_info.ptMinTrackSize.y));
                    min_max_info.ptMaxTrackSize.x = FMath::RoundToInt( SizeLimits.GetMaxWidth().Get(min_max_info.ptMaxTrackSize.x) ) + BorderWidth;
                    min_max_info.ptMaxTrackSize.y = FMath::RoundToInt( SizeLimits.GetMaxHeight().Get(min_max_info.ptMaxTrackSize.y) ) + BorderHeight;
                    return 0;
                },
                //break;
                WM_NCLBUTTONDOWN |
                WM_NCRBUTTONDOWN |
                WM_NCMBUTTONDOWN => {
                    if wparam == HTMINBUTTON as u64 {
                        if !MessageHandler->OnWindowAction(current_native_event_window, WindowAction::ClickedNonClientArea) {
                            return 1;
                        }
                    } else if wparam == HTMAXBUTTON as u64 {
                        if !MessageHandler->OnWindowAction(current_native_event_window, WindowAction::ClickedNonClientArea) {
                            return 1;
                        }
                    } else if wparam == HTCLOSE as u64 {
                        if !MessageHandler->OnWindowAction(current_native_event_window, WindowAction::ClickedNonClientArea) {
                            return 1;
                        }
                    } else if wparam == HTCAPTION as u64 {
                        if !MessageHandler->OnWindowAction(current_native_event_window, WindowAction::ClickedNonClientArea) {
                            return 1;
                        }
                    }
                },
                //break;
                WM_DISPLAYCHANGE => {
                    // Slate needs to know when desktop size changes.
                    let mut display_metrics = DisplayMetrics::new();
                    BroadcastDisplayMetricsChanged(DisplayMetrics);
                },
                //break;
                WM_GETDLGCODE => {
                    // Slate wants all keys and messages.
                    return DLGC_WANTALLKEYS as i32;
                },
                //break;
                WM_CREATE => {
                    return 0;
                },
                WM_DEVICECHANGE => {
                    XInput->SetNeedsControllerStateUpdate(); 
                    QueryConnectedMice();
                },
                default => {
                    let mut handler_result: i32 = 0;

                    // give others a chance to handle unprocessed messages
                    for (auto Handler : MessageHandlers) {
                        if Handler->ProcessMessage(hwnd, msg, wparam, lparam, handler_result) {
                            return HandlerResult;
                        }
                    }
                },
            }
        }
        user32::DefWindowProcW(hwnd, msg, wparam, lparam) as i32
    }
    /*void FWindowsApplication::GetInitialDisplayMetrics( FDisplayMetrics& OutDisplayMetrics ) const {
        OutDisplayMetrics = InitialDisplayMetrics;
    }*/
    fn defer_message(&self, native_window: &Rc<WindowsWindow>, hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM, mouse_x: i32, mouse_y: i32, raw_input_flags: u32) {
        if PUMPING_MESSAGE_OUTSIDE_OF_MAIN_LOOP && self.allowed_to_defer_message_processing {
            self.deferred_messages.push(DeferredWindowsMessage::new(native_window, hwnd, message, wparam, lparam, mouse_x, mouse_y, raw_input_flags));
        } else {
            // When not deferring messages, process them immediately
            self.process_deferred_message(DeferredWindowsMessage::new(native_window, hwnd, message, wparam, lparam, mouse_x, mouse_y, raw_input_flags));
        }
    }
    fn pump_messages(&self, time_delta: f32) {
        unsafe {
            let mut message: MSG = mem::uninitialized();

            // standard Windows message handling
            while user32::PeekMessageW(&mut message, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 { 
                user32::TranslateMessage(&message);
                user32::DispatchMessageW(&message); 
            }
        }
    }
    unsafe extern "system" fn app_wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        //ensure( IsInGameThread() );

        (&*windows_application).process_message(hwnd, msg, wparam, lparam) as i64
    }
}

fn get_monitor_size_from_edid(h_dev_reg_key: RegKey, out_width: &mut i32, out_height: &mut i32) -> bool {
    for (name, value) in h_dev_reg_key.enum_values().map(|x| x.unwrap()) {
        if &name[..] != "EDID" {
            continue;
        } else {
            // EDID data format documented here:
            // http://en.wikipedia.org/wiki/EDID

            let mut detail_timing_descriptor_start_index: usize = 54;
            *out_width = (((value.bytes[detail_timing_descriptor_start_index + 4] >> 4) << 8) | value.bytes[detail_timing_descriptor_start_index + 2]) as i32;
            *out_height = (((value.bytes[detail_timing_descriptor_start_index + 7] >> 4) << 8) | value.bytes[detail_timing_descriptor_start_index + 5]) as i32;

            return true; // valid EDID found
        }
    }

    false
}

fn get_size_for_dev_id(target_dev_id: &String, width: &mut i32, height: &mut i32) -> bool {
    unsafe {
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

        let res: bool = false;
        let mut monitor_index = 0;
        loop {
            let err = Error::last_os_error();
            if err.raw_os_error().unwrap() == ERROR_NO_MORE_ITEMS as i32 { break; }
            let mut dev_info_data: SP_DEVINFO_DATA = mem::uninitialized();
            dev_info_data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

            if setupapi::SetupDiEnumDeviceInfo(dev_info, monitor_index, &mut dev_info_data) == TRUE {
                let mut buffer = [0u16; MAX_DEVICE_ID_LEN];
                if setupapi::CM_Get_Device_IDW(dev_info_data.DevInst, buffer.as_mut_ptr(), MAX_DEVICE_ID_LEN as u32, 0) == CR_SUCCESS {
                    let dev_id = String::from_utf16_lossy(&buffer[..]);
                    let idx = &dev_id[9..].find("\\").unwrap();
                    dev_id = dev_id[8..idx - 8].to_string();
                    if &dev_id[..] == &target_dev_id[..] {
                        let mut h_dev_reg_key: HKEY = setupapi::SetupDiOpenDevRegKey(dev_info, &mut dev_info_data, DICS_FLAG_GLOBAL, 0, DIREG_DEV, KEY_READ);

                        if !h_dev_reg_key.is_null() && h_dev_reg_key != INVALID_HANDLE_VALUE as HKEY {
                            res = get_monitor_size_from_edid(RegKey::predef(h_dev_reg_key), width, height);
                            //advapi32::RegCloseKey(h_dev_reg_key);
                            break;
                        }
                    }
                }
            }
        }

        if setupapi::SetupDiDestroyDeviceInfoList(dev_info) == FALSE {
            res = false;
        }

        res
    }
}

fn get_monitor_info(out_monitor_info: &Vec<MonitorInfo>) {
    unsafe {
        let mut display_device: DISPLAY_DEVICEW = mem::uninitialized();
        display_device.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
        let mut device_index: DWORD = 0; // device index

        let primary_device: *mut MonitorInfo = ptr::null_mut();
        out_monitor_info.reserve(2); // Reserve two slots, as that will be the most common maximum

        while user32::EnumDisplayDevicesW(0 as *const u16, device_index, &mut display_device, 0) != 0 {
            if display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP > 0 {
                let mut monitor: DISPLAY_DEVICEW = mem::uninitialized();
                monitor.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
                let mut monitor_index: DWORD = 0;

                while user32::EnumDisplayDevicesW(display_device.DeviceName.as_ptr(), monitor_index, &mut monitor, 0) != 0 {
                    if (monitor.StateFlags & DISPLAY_DEVICE_ACTIVE != 0) && (monitor.StateFlags & DISPLAY_DEVICE_MIRRORING_DRIVER == 0) {
                        let mut info: MonitorInfo = mem::uninitialized();

                        let temp_str = String::from_utf16_lossy(&monitor.DeviceID[..]);
                        let idx = &temp_str[9..].find("\\").unwrap();
                        info.name.push_str(&temp_str[8..idx - 8]);
                        info.id = temp_str;
                        //info.name = info.id.Mid (8, Info.ID.Find (TEXT("\\"), ESearchCase::CaseSensitive, ESearchDir::FromStart, 9) - 8);

                        if get_size_for_dev_id(&info.name, &mut info.native_width, &mut info.native_height) {
                            let temp_str = String::from_utf16_lossy(&monitor.DeviceID[..]);
                            info.id = temp_str;
                            info.is_primary = (display_device.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE) > 0;
                            out_monitor_info.push(info);

                            if primary_device.is_null() && info.is_primary {
                                primary_device = &mut out_monitor_info[out_monitor_info.len()];
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
pub struct DisplayMetrics {
    primary_display_width: i32,
    primary_display_height: i32,
    monitor_info: Vec<MonitorInfo>,
    primary_display_work_area_rect: PlatformRect,
    virtual_display_rect: PlatformRect,
    //TODO: The following should be a Vector2D
    title_safe_padding_size: (f32, f32),
    //TODO: The following should be a Vector2D
    action_safe_padding_size: (f32, f32),
}

impl DisplayMetrics {
    pub fn new() -> DisplayMetrics {
        unsafe {
            let mut out_display_metrics: DisplayMetrics = mem::uninitialized();
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
        DEBUG_SAFE_ZONE_RATIO
    }
    pub fn get_debug_action_safe_zone_ratio(&self) -> f32 {
        DEBUG_ACTION_ZONE_RATIO
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