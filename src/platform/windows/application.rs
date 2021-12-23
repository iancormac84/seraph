use crate::core::math::int_point::IntPoint;
use crate::generic::application::{
    GenericApplication, MonitorInfo, PlatformRect, DEBUG_ACTION_ZONE_RATIO, DEBUG_SAFE_ZONE_RATIO,
};
use crate::generic::application_message_handler::{
    ApplicationMessageHandler, WindowAction, WindowSizeLimits, WindowZone,
};
use cgmath::Point2;
use windows::Win32::Devices::HumanInterfaceDevice::MOUSE_VIRTUAL_DESKTOP;
//use crate::generic::cursor::ICursor;
use crate::generic::window::GenericWindow;
use crate::generic::window_definition::{WindowDefinition, WindowTransparency, WindowType};
use crate::windows::cursor::WindowsCursor;
use crate::windows::utils::ToWide;
use crate::windows::utils;
use crate::windows::window::{WindowsWindow, APP_WINDOW_CLASS};
use crate::windows::xinputinterface::XInputInterface;
use lazy_static::lazy_static;
use std::{cell::RefCell, collections::BTreeMap, io::Error, mem, os::raw::c_void, ptr, rc::Rc, sync::{Arc, Once}};

use windows::Win32::{
    Devices::{
        DeviceAndDriverInstallation::{
            CM_Get_Device_IDW, SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo,
            SetupDiGetClassDevsExW, SetupDiOpenDevRegKey, CR_SUCCESS, DICS_FLAG_GLOBAL,
            DIGCF_PRESENT, DIREG_DEV, GUID_DEVCLASS_MONITOR, MAX_DEVICE_ID_LEN, SP_DEVINFO_DATA,
        },
        HumanInterfaceDevice::{MOUSE_MOVE_ABSOLUTE, MOUSE_MOVE_RELATIVE},
    },
    Foundation::{
        ERROR_NO_MORE_ITEMS, HINSTANCE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, POINT, POINTL,
        PWSTR, RECT, WPARAM,
    },
    Graphics::{
        Dwm::DwmIsCompositionEnabled,
        Gdi::{
            EnumDisplayDevicesW, GetMonitorInfoW, MonitorFromRect, DISPLAY_DEVICEW,
            DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP,
            DISPLAY_DEVICE_MIRRORING_DRIVER, DISPLAY_DEVICE_PRIMARY_DEVICE, HMONITOR, MONITORINFO,
            MONITOR_DEFAULTTONEAREST,
        },
    },
    System::Registry::{HKEY, KEY_READ},
    UI::{
        Accessibility::{
            FILTERKEYS, SKF_CONFIRMHOTKEY, SKF_HOTKEYACTIVE, SKF_STICKYKEYSON, STICKYKEYS,
            TOGGLEKEYS,
        },
        Controls::{WM_MOUSEHOVER, WM_MOUSELEAVE},
        Input::{
            GetRawInputData, GetRawInputDeviceInfoA, GetRawInputDeviceList,
            Ime::{
                IMN_CHANGECANDIDATE, IMN_CLOSECANDIDATE, IMN_CLOSESTATUSWINDOW, IMN_GUIDELINE,
                IMN_OPENCANDIDATE, IMN_OPENSTATUSWINDOW, IMN_PRIVATE, IMN_SETCANDIDATEPOS,
                IMN_SETCOMPOSITIONFONT, IMN_SETCOMPOSITIONWINDOW, IMN_SETCONVERSIONMODE,
                IMN_SETOPENSTATUS, IMN_SETSENTENCEMODE, IMN_SETSTATUSWINDOWPOS,
                IMR_CANDIDATEWINDOW, IMR_COMPOSITIONFONT, IMR_COMPOSITIONWINDOW,
                IMR_CONFIRMRECONVERTSTRING, IMR_DOCUMENTFEED, IMR_QUERYCHARPOSITION,
                IMR_RECONVERTSTRING,
            },
            KeyboardAndMouse::{
                GetAsyncKeyState, GetCapture, GetKeyState, SetCapture, VK_CAPITAL, VK_F4,
                VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_SPACE,
            },
            RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE, RAWINPUTDEVICELIST,
            RAWINPUTHEADER, RIDEV_REMOVE, RIDI_DEVICENAME, RID_INPUT, RIM_TYPEMOUSE,
        },
        WindowsAndMessaging::{
            AdjustWindowRectEx, DefWindowProcW, DisableProcessWindowsGhosting, DispatchMessageW,
            GetCursorPos, GetSystemMetrics, GetWindowInfo, GetWindowLongW, MessageBoxW,
            PeekMessageW, RegisterClassW, SetCursorPos, SystemParametersInfoW, TranslateMessage,
            WindowFromPoint, CREATESTRUCTW, CS_DBLCLKS, DLGC_WANTALLKEYS, FKF_CONFIRMHOTKEY,
            FKF_FILTERKEYSON, FKF_HOTKEYACTIVE, GWLP_USERDATA, GWL_EXSTYLE, GWL_STYLE, HICON,
            HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION, HTCLIENT, HTCLOSE, HTLEFT,
            HTMAXBUTTON, HTMINBUTTON, HTNOWHERE, HTRIGHT, HTSYSMENU, HTTOP, HTTOPLEFT, HTTOPRIGHT,
            MB_ICONEXCLAMATION, MB_OK, MINMAXINFO, MSG, NCCALCSIZE_PARAMS, PM_REMOVE, SC_MAXIMIZE,
            SC_RESTORE, SM_CXSCREEN, SM_CXVIRTUALSCREEN, SM_CYSCREEN, SM_CYVIRTUALSCREEN,
            SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SPI_GETWORKAREA, SPI_SETFILTERKEYS,
            SPI_SETSTICKYKEYS, SPI_SETTOGGLEKEYS, SW_RESTORE, TKF_CONFIRMHOTKEY, TKF_HOTKEYACTIVE,
            TKF_TOGGLEKEYSON, WINDOWINFO, WMSZ_BOTTOM, WMSZ_BOTTOMLEFT, WMSZ_BOTTOMRIGHT,
            WMSZ_LEFT, WMSZ_RIGHT, WMSZ_TOP, WMSZ_TOPLEFT, WMSZ_TOPRIGHT, WM_ACTIVATE,
            WM_ACTIVATEAPP, WM_CHAR, WM_CLOSE, WM_CREATE, WM_DESTROY, WM_DEVICECHANGE,
            WM_DISPLAYCHANGE, WM_DWMCOMPOSITIONCHANGED, WM_ENTERSIZEMOVE, WM_ERASEBKGND,
            WM_EXITSIZEMOVE, WM_GETDLGCODE, WM_GETMINMAXINFO, WM_IME_CHAR, WM_IME_COMPOSITION,
            WM_IME_ENDCOMPOSITION, WM_IME_NOTIFY, WM_IME_REQUEST, WM_IME_SETCONTEXT,
            WM_IME_STARTCOMPOSITION, WM_INPUT, WM_INPUTLANGCHANGE, WM_INPUTLANGCHANGEREQUEST,
            WM_INPUT_DEVICE_CHANGE, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN,
            WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEACTIVATE,
            WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_MOVE, WM_NCACTIVATE, WM_NCCALCSIZE,
            WM_NCCREATE, WM_NCHITTEST, WM_NCLBUTTONDOWN, WM_NCMBUTTONDBLCLK, WM_NCMBUTTONDOWN,
            WM_NCMBUTTONUP, WM_NCMOUSEHOVER, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE, WM_NCPAINT,
            WM_NCRBUTTONDBLCLK, WM_NCRBUTTONDOWN, WM_NCRBUTTONUP, WM_NCXBUTTONDBLCLK,
            WM_NCXBUTTONDOWN, WM_NCXBUTTONUP, WM_PAINT, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN,
            WM_RBUTTONUP, WM_SETCURSOR, WM_SETTINGCHANGE, WM_SHOWWINDOW, WM_SIZE, WM_SIZING,
            WM_SYSCHAR, WM_SYSCOMMAND, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_TOUCH, WM_XBUTTONDBLCLK,
            WM_XBUTTONDOWN, WM_XBUTTONUP, WNDCLASSW, WVR_VALIDRECTS,
        },
    },
};
use winreg::RegKey;

type IntPoint2 = Point2<i32>;

pub static mut WINDOWS_APPLICATION: Option<&'static Arc<WindowsApplication>> = None;
pub static INIT_APPLICATION: Once = Once::new();

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

static HIT_RESULTS: [LRESULT; 15] = [
    HTNOWHERE as isize,
    HTTOPLEFT as isize,
    HTTOP as isize,
    HTTOPRIGHT as isize,
    HTLEFT as isize,
    HTCLIENT as isize,
    HTRIGHT as isize,
    HTBOTTOMLEFT as isize,
    HTBOTTOM as isize,
    HTBOTTOMRIGHT as isize,
    HTCAPTION as isize,
    HTMINBUTTON as isize,
    HTMAXBUTTON as isize,
    HTCLOSE as isize,
    HTSYSMENU as isize,
];

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
}

pub enum WindowsDragDropOperationType {
    DragEnter,
    DragOver,
    DragLeave,
    Drop,
}

//This is internal to the DragDropOLEData struct in the Unreal Engine 4 code.
pub enum WindowsOLEDataType {
    None = 0,
    Text = 1 << 0,
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
    key_state: u32,
    cursor_position: POINTL,
}

//I just got an idea about making it a struct that deals with a type that implements a trait that contains the process_message method, but probably not.
//Actually, this will most likely remain a trait whose method wraps the real ProcessMessage method, since the real method needs an ABI signature that is not compatible with Rust's method
//call signatures.
pub trait IWindowsMessageHandler {
    fn process_message(
        &mut self,
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        out_result: &mut i32,
    ) -> bool;
}

//This is internal to the WindowsApplication class in the Unreal Engine 4 code.
pub enum ModifierKey {
    LeftShift = 0,    // VK_LSHIFT
    RightShift = 1,   // VK_RSHIFT
    LeftControl = 2,  // VK_LCONTROL
    RightControl = 3, // VK_RCONTROL
    LeftAlt = 4,      // VK_LMENU
    RightAlt = 5,     // VK_RMENU
    CapsLock = 6,     // VK_CAPITAL
    Count = 7,
}

pub fn create_windows_application(
    hinstance: HINSTANCE,
    hicon: HICON,
) -> &'static Arc<WindowsApplication> {
    INIT_APPLICATION.call_once(|| unsafe { init_windows_application(hinstance, hicon) });
    unsafe { WINDOWS_APPLICATION.unwrap() }
}

unsafe fn init_windows_application(hinstance: HINSTANCE, hicon: HICON) {
    let windows_application = WindowsApplication::new(hinstance, hicon);
    let app = utils::leak(Arc::new(windows_application));
    println!("app address is {:p}", &app);
    WINDOWS_APPLICATION = Some(app);
}

fn HIWORD(l: isize) -> isize {
    (l >> 16) & 0xffff
}

fn LOWORD(l: isize) -> isize {
    l & 0xffff
}

//TODO implement GenericApplication trait. Also most likely trait based on IForceFeedbackSystem.
pub struct WindowsApplication {
    cursor: Rc<WindowsCursor>,
    minimized: bool,
    minimized_window_position: IntPoint2,
    instance_handle: HINSTANCE,
    using_high_precision_mouse_input: bool,
    is_mouse_attached: bool,
    force_activate_by_mouse: bool,
    force_no_gamepads: bool,
    consume_alt_space: bool,
    //XInput( XInputInterface::Create( MessageHandler ) )
    has_loaded_input_plugins: bool,
    allowed_to_defer_message_processing: bool,
    pub windows: RefCell<Vec<RefCell<WindowsWindow>>>,
    modifier_key_state: [bool; ModifierKey::Count as usize],
    in_modal_size_loop: bool,
    pub display_metrics: DisplayMetrics,
    message_handlers: Vec<Box<dyn IWindowsMessageHandler>>,
    startup_sticky_keys: STICKYKEYS,
    startup_toggle_keys: TOGGLEKEYS,
    startup_filter_keys: FILTERKEYS,
    simulating_high_precision_mouse_input_for_rdp: bool,
    num_pre_wrap_msgs_to_respect: i32,
    last_cursor_point: IntPoint2,
    last_cursor_point_pre_wrap: IntPoint2,
    clip_cursor_rect: RECT,
}

impl WindowsApplication {
    fn allow_accessibility_shortcut_keys(&mut self, allow_keys: bool) {
        unsafe {
            if allow_keys {
                // Restore StickyKeys/etc to original state and enable Windows key
                SystemParametersInfoW(
                    SPI_SETSTICKYKEYS,
                    mem::size_of::<STICKYKEYS>() as u32,
                    &mut self.startup_sticky_keys as *mut _ as *mut c_void,
                    0,
                );
                SystemParametersInfoW(
                    SPI_SETTOGGLEKEYS,
                    mem::size_of::<TOGGLEKEYS>() as u32,
                    &mut self.startup_toggle_keys as *mut _ as *mut c_void,
                    0,
                );
                SystemParametersInfoW(
                    SPI_SETFILTERKEYS,
                    mem::size_of::<FILTERKEYS>() as u32,
                    &mut self.startup_filter_keys as *mut _ as *mut c_void,
                    0,
                );
            } else {
                // Disable StickyKeys/etc shortcuts but if the accessibility feature is on,
                // then leave the settings alone as its probably being usefully used

                let mut sk_off = self.startup_sticky_keys;
                if (sk_off.dwFlags & SKF_STICKYKEYSON) == 0 {
                    // Disable the hotkey and the confirmation
                    sk_off.dwFlags &= !SKF_HOTKEYACTIVE;
                    sk_off.dwFlags &= !SKF_CONFIRMHOTKEY;

                    SystemParametersInfoW(
                        SPI_SETSTICKYKEYS,
                        mem::size_of::<STICKYKEYS>() as u32,
                        &mut sk_off as *mut _ as *mut c_void,
                        0,
                    );
                }

                let mut tk_off = self.startup_toggle_keys;
                if (tk_off.dwFlags & TKF_TOGGLEKEYSON) == 0 {
                    // Disable the hotkey and the confirmation
                    tk_off.dwFlags &= !TKF_HOTKEYACTIVE;
                    tk_off.dwFlags &= !TKF_CONFIRMHOTKEY;

                    SystemParametersInfoW(
                        SPI_SETTOGGLEKEYS,
                        mem::size_of::<TOGGLEKEYS>() as u32,
                        &mut tk_off as *mut _ as *mut c_void,
                        0,
                    );
                }

                let mut fk_off = self.startup_filter_keys;
                if (fk_off.dwFlags & FKF_FILTERKEYSON) == 0 {
                    // Disable the hotkey and the confirmation
                    fk_off.dwFlags &= !FKF_HOTKEYACTIVE;
                    fk_off.dwFlags &= !FKF_CONFIRMHOTKEY;

                    SystemParametersInfoW(
                        SPI_SETFILTERKEYS,
                        mem::size_of::<FILTERKEYS>() as u32,
                        &mut fk_off as *mut _ as *mut c_void,
                        0,
                    );
                }
            }
        }
    }
    pub fn new(hinstance: HINSTANCE, hicon: HICON) -> WindowsApplication {
        unsafe { DisableProcessWindowsGhosting() };
        let display_metrics = DisplayMetrics::new();
        let mut winapp = WindowsApplication {
            cursor: Rc::new(WindowsCursor::new()),
            instance_handle: hinstance,
            minimized: false,
            using_high_precision_mouse_input: false,
            is_mouse_attached: false,
            force_activate_by_mouse: false,
            force_no_gamepads: false,
            consume_alt_space: false,
            //XInput( XInputInterface::Create( MessageHandler ) )
            has_loaded_input_plugins: false,
            allowed_to_defer_message_processing: true,
            /* CVarDeferMessageProcessing(
            TEXT( "Slate.DeferWindowsMessageProcessing" ),
            bAllowedToDeferMessageProcessing,
            TEXT( "Whether windows message processing is deferred until tick or if they are processed immediately" ) ) */
            in_modal_size_loop: false,
            minimized_window_position: IntPoint2::new(-32000, -32000),
            windows: RefCell::new(vec![]),
            modifier_key_state: [false; ModifierKey::Count as usize],
            display_metrics: display_metrics,
            message_handlers: vec![],
            startup_sticky_keys: STICKYKEYS::default(),
            startup_toggle_keys: TOGGLEKEYS::default(),
            startup_filter_keys: FILTERKEYS::default(),
            simulating_high_precision_mouse_input_for_rdp: false,
            num_pre_wrap_msgs_to_respect: 0,
            last_cursor_point: IntPoint2 {
                x: 0,
                y: 0,
            },
            last_cursor_point_pre_wrap: IntPoint2 {
                x: 0,
                y: 0,
            },
            clip_cursor_rect: RECT::default(),
        };
        println!("Successfully created winapp");
        let class_registered = winapp.register_class(hinstance, hicon);
        println!("Have we registered class? {}", class_registered);
        winapp.query_connected_mice();
        //println!("winapp debug is {:#?}", winapp);
        winapp
    }
    pub fn make_window(&self) -> RefCell<WindowsWindow> {
        WindowsWindow::make()
    }
    pub fn initialize_window(
        &self,
        in_window: &RefCell<WindowsWindow>,
        in_definition: WindowDefinition,
        in_parent_window: &Option<Rc<WindowsWindow>>,
        show_immediately: bool,
    ) {
        println!("about to push on dat windows. Mutable borrow here.");
        let window = in_window.clone();
        let parent_window = if let Some(parent) = in_parent_window {
            Some(parent.clone())
        } else {
            None
        };
        window.borrow_mut().initialize(
            &self,
            in_definition,
            self.instance_handle,
            parent_window,
            show_immediately,
        );
        let mut windows_mut_borrow = self.windows.borrow_mut();
        windows_mut_borrow.push(window);
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
                hCursor: 0,       // We manage the cursor ourselves
                hbrBackground: 0, // Transparent
                lpszMenuName: PWSTR("".to_wide_null().as_mut_ptr()),
                lpszClassName: PWSTR(APP_WINDOW_CLASS.to_wide_null().as_mut_ptr()),
            };

            if RegisterClassW(&wc) == 0 {
                //ShowLastError();

                // @todo Slate: Error message should be localized!
                //FSlowHeartBeatScope SuspendHeartBeat;
                MessageBoxW(
                    0,
                    PWSTR("Window Registration Failed!".to_wide_null().as_mut_ptr()),
                    PWSTR("Error!".to_wide_null().as_mut_ptr()),
                    MB_ICONEXCLAMATION | MB_OK,
                );

                return false;
            }

            return true;
        }
    }
    pub fn get_window_transparency_support(&self) -> WindowTransparency {
        let res = unsafe { DwmIsCompositionEnabled() };

        match res {
            Err(error) => {
                println!("WARNING: {}", error);
                WindowTransparency::PerWindow
            }
            Ok(composition_enabled) => {
                if composition_enabled.0 != 0 {
                    WindowTransparency::PerPixel
                } else {
                    WindowTransparency::PerWindow
                }
            }
        }
    }
    //TODO the return signature for this method feels wrong.
    pub fn find_window_by_hwnd(
        &self,
        windows_to_search: &Vec<RefCell<WindowsWindow>>,
        handle_to_find: HWND,
    ) -> Option<RefCell<WindowsWindow>> {
        println!(
            "Inside find_window_by_hwnd, handle_to_find is {:?}",
            handle_to_find
        );
        let len = windows_to_search.len();
        let mut n = 0;
        loop {
            if n == len {
                break;
            }
            let window = &windows_to_search[n];
            if window.borrow().get_hwnd() == handle_to_find {
                return Some(window.clone());
            }
            n += 1;
        }
        None
    }
    pub fn is_cursor_directly_over_seraph_window(&self) -> bool {
        unsafe {
            let mut cursor_pos = POINT::default();
            let got_point = GetCursorPos(&mut cursor_pos);
            if got_point.0 != 0 {
                let hwnd: HWND = WindowFromPoint(cursor_pos);
                let window_under_cursor = self.find_window_by_hwnd(&self.windows.borrow(), hwnd);
                return window_under_cursor.is_some();
            }
        }
        false
    }
    pub fn set_capture(&mut self, window: Rc<dyn GenericWindow>) {
        //if ( InWindow.IsValid() )
        unsafe {
            SetCapture(window.get_os_window_handle() as HWND);
        }
        /*else
        {
            ::ReleaseCapture();
        }
        */
    }
    pub fn get_capture(&self) -> HWND {
        unsafe { GetCapture() }
    }
    pub fn set_high_precision_mouse_mode(&mut self, enable: bool, window: Rc<dyn GenericWindow>) {
        unsafe {
            let mut hwnd: HWND = 0;
            let mut flags: u32 = RIDEV_REMOVE;
            self.using_high_precision_mouse_input = enable;

            if enable {
                flags = 0;
                //if ( InWindow.IsValid() )
                hwnd = window.get_os_window_handle() as HWND;
            }
            let mut raw_input_device = RAWINPUTDEVICE::default();
            //The HID standard for mouse
            let standard_mouse: u16 = 0x02;

            raw_input_device.usUsagePage = 0x01;
            raw_input_device.usUsage = standard_mouse;
            raw_input_device.dwFlags = flags;

            // Process input for just the window that requested it.  NOTE: If we pass NULL here events are routed to the window with keyboard focus
            // which is not always known at the HWND level with Slate
            raw_input_device.hwndTarget = hwnd;

            // Register the raw input device
            RegisterRawInputDevices(
                &raw_input_device,
                1,
                mem::size_of::<RAWINPUTDEVICE>() as u32,
            );
        }
    }
    pub fn get_work_area(&self, current_window: &PlatformRect) -> PlatformRect {
        let mut windows_window_dim = RECT::default();
        windows_window_dim.left = current_window.left;
        windows_window_dim.top = current_window.top;
        windows_window_dim.right = current_window.right;
        windows_window_dim.bottom = current_window.bottom;

        unsafe {
            let best_monitor: HMONITOR =
                MonitorFromRect(&mut windows_window_dim, MONITOR_DEFAULTTONEAREST);

            let mut monitor_info = MONITORINFO::default();
            monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
            GetMonitorInfoW(best_monitor, &mut monitor_info);

            let mut work_area = PlatformRect::default();
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
            println!("hwnd is {:?}", hwnd);
            println!("WindowsApplication self address is {:p}", self);
            let len = self.windows.borrow().len();
            println!(
                "self.windows.len() is {}. about to find window by hwnd.",
                len
            );
            let current_native_event_window_opt = self.find_window_by_hwnd(&self.windows.borrow(), hwnd);

            if len != 0 && current_native_event_window_opt.is_some() {
                println!(
                    "current_native_event_window_opt is some. Don't believe me? Look! {:#?}",
                    current_native_event_window_opt
                );
                let current_native_event_window = current_native_event_window_opt.unwrap();

                let mut message_externally_handled = false;
                let mut external_message_handler_result = 0;

                for handler in self.message_handlers {
                    let mut handler_result = 0;
                    let res = handler.process_message(hwnd, msg, wparam, lparam, &mut handler_result);
                    if !message_externally_handled {
                        message_externally_handled = true;
                        external_message_handler_result = handler_result;
                    }
                }

                match msg {
                    WM_INPUTLANGCHANGEREQUEST | WM_INPUTLANGCHANGE | WM_IME_SETCONTEXT | WM_IME_COMPOSITION | WM_IME_ENDCOMPOSITION | WM_IME_CHAR => {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        return 0;
                    }
                    WM_IME_NOTIFY => {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        return 0;
                    }
                    WM_IME_REQUEST => {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        return 0;
                    }
                    WM_CHAR => {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        return 0;
                    }
                    WM_SYSCHAR => {
                        if !self.consume_alt_space && (HIWORD(lparam) & 0x2000) != 0 && wparam == VK_SPACE as usize {

                        } else {
                            return 0;
                        }
                    }
                    WM_SYSKEYDOWN => {
                        // Alt-F4 or Alt+Space was pressed.
				        if wparam == VK_F4 as usize {
                            // Allow alt+f4 to close the window, but write a log warning
                            println!("Alt-F4 pressed!");
                        } // If we're consuming alt+space, pass it along
				        else if self.consume_alt_space || wparam != VK_SPACE as usize {
                            self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        }
                    }
                    WM_KEYDOWN | WM_SYSKEYUP | WM_KEYUP | WM_LBUTTONDBLCLK | WM_LBUTTONDOWN | WM_MBUTTONDBLCLK | WM_MBUTTONDOWN | WM_RBUTTONDBLCLK | WM_RBUTTONDOWN | WM_XBUTTONDBLCLK | WM_XBUTTONDOWN | WM_XBUTTONUP | WM_LBUTTONUP | WM_MBUTTONUP | WM_RBUTTONUP | WM_NCMOUSEMOVE | WM_MOUSEMOVE | WM_MOUSEWHEEL | WM_TOUCH => {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);
                        return 0;
                    }
                    WM_SETCURSOR => {
                        self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, 0);

                        // If we're rendering our own window border, we'll "handle" this event so that Windows doesn't try to manage the cursor
			            // appearance for us in the non-client area.  However, for OS window borders we need to fall through to DefWindowProc to
			            // allow Windows to draw the resize cursor
                        if current_native_event_window.borrow().get_definition().has_os_window_border {
                            // Handled
				            return 0;
                        }
                    }
                    WM_INPUT => {
                        let mut size: u32 = 0;
                        GetRawInputData(
                            lparam as HRAWINPUT,
                            RID_INPUT,
                            ptr::null_mut(),
                            &mut size,
                            mem::size_of::<RAWINPUTHEADER>() as u32,
                        );

                        let (raw, raw_input_num) = {
                            let mut raw = RAWINPUT::default();
                            let raw_input_num = GetRawInputData(
                                    lparam as HRAWINPUT,
                                    RID_INPUT,
                                    &mut raw as *mut RAWINPUT as *mut c_void,
                                    &mut size,
                                    mem::size_of::<RAWINPUTHEADER>() as u32
                                );
                            (raw, raw_input_num)
                        };
                        if raw_input_num == size {
                            let raw_mouse = raw.data.mouse;

                            if raw.header.dwType == RIM_TYPEMOUSE {
                                let is_absolute_input = (raw_mouse.usFlags as u32
                                    & MOUSE_MOVE_ABSOLUTE)
                                    == MOUSE_MOVE_ABSOLUTE;
                                if is_absolute_input {
                                    if self.simulating_high_precision_mouse_input_for_rdp {	
								        let is_virtual_screen = (raw_mouse.usFlags as u32 & MOUSE_VIRTUAL_DESKTOP) == MOUSE_VIRTUAL_DESKTOP;

								        // Get the new cursor position
								        let mut cursor_point = POINT::default();
								        let left = if is_virtual_screen { GetSystemMetrics(SM_XVIRTUALSCREEN) } else { 0 };
								let top = if is_virtual_screen { GetSystemMetrics(SM_YVIRTUALSCREEN) } else { 0 };
								let width = GetSystemMetrics(if is_virtual_screen { SM_CXVIRTUALSCREEN } else { SM_CXSCREEN });
								let height = GetSystemMetrics(if is_virtual_screen { SM_CYVIRTUALSCREEN } else { SM_CYSCREEN });

								cursor_point.x = ((raw_mouse.lLastX as f32 / 65535.0) * width as f32) as i32 + left;
								cursor_point.y = ((raw_mouse.lLastY as f32 / 65535.0) * height as f32) as i32 + top;

								let clip_width = self.clip_cursor_rect.right - self.clip_cursor_rect.left;
								let clip_height = self.clip_cursor_rect.bottom - self.clip_cursor_rect.top;

								let delta_width_max = (clip_width as f32 * 0.4) as i32;
								let delta_height_max = (clip_height as f32 * 0.4) as i32;

								let cursor_point_no_wrap = cursor_point;

								let can_accept_pre_wrap_msg = self.num_pre_wrap_msgs_to_respect > 0;
								self.num_pre_wrap_msgs_to_respect -= 1;

								// Calculate the cursor delta from the last position
								// We do this prior to and wrapping for continuous input as that would mean an incorrect delta we'd just ignore
								let delta_x = cursor_point.x - self.last_cursor_point.x;
								let delta_y = cursor_point.y - self.last_cursor_point.y;

								// Always skip wrapping if no movement has occured.
								let accepting_pre_wrap_delta = delta_x == 0 && delta_y == 0;

								if can_accept_pre_wrap_msg && !accepting_pre_wrap_delta {
									let delta_x_pre_wrap = cursor_point.x - self.last_cursor_point_pre_wrap.x;
									let delta_y_pre_wrap = cursor_point.y - self.last_cursor_point_pre_wrap.y;

									let delta_len = (delta_x * delta_x) + (delta_y * delta_y);
									let delta_pre_wrap_len = (delta_x_pre_wrap * delta_x_pre_wrap) + (delta_y_pre_wrap * delta_y_pre_wrap);

									if delta_pre_wrap_len < delta_len {
										accepting_pre_wrap_delta = true;

										delta_x = delta_x_pre_wrap;
										delta_y = delta_y_pre_wrap;
										self.last_cursor_point_pre_wrap.x = cursor_point.x;
										self.last_cursor_point_pre_wrap.y = cursor_point.y;

										//UE_LOG(LogWindowsDesktop, Log, TEXT("Accept PreWrap Delta X: %d Y: %d  ---- Last X: %d Y: %d ---- Cur X: %d Y: %d ---- Width: %d Height: %d"), DeltaX, DeltaY, LastCursorPointPreWrap.X, LastCursorPointPreWrap.Y, CursorPoint.x, CursorPoint.y, (int32)Width, (int32)Height);
									}
								}

								if !accepting_pre_wrap_delta {
									// Wrap and set cursor position in necessary
									let wrap_leeway = 50; // We add some leeway on the wrap so that if the user is doing small movements hear the border we don't wrap back and fourth constantly
									let top_edge = self.clip_cursor_rect.top + (0.1 * clip_height as f32) as i32;
									let bottom_edge = self.clip_cursor_rect.top + (0.9 * clip_height as f32) as i32;
									let left_edge = self.clip_cursor_rect.left + (0.1 * clip_width as f32) as i32;
									let right_edge = self.clip_cursor_rect.left + (0.9 * clip_width as f32) as i32;

									let mut set = false;
									if (cursor_point.y < top_edge) { cursor_point.y = bottom_edge - wrap_leeway;	set = true; }
									else if (cursor_point.y > bottom_edge) { cursor_point.y = top_edge + wrap_leeway; set = true; }

									if (cursor_point.x < left_edge) { cursor_point.x = right_edge - wrap_leeway;	set = true; }
									else if (cursor_point.x > right_edge) { cursor_point.x = left_edge + wrap_leeway; set = true; }

									if set {
										//UE_LOG(LogWindowsDesktop, Log, TEXT("Wrapping Cursor to X: %d Y: %d ---- TopEdge: %d BottomEdge: %d left_edge: %d RightEdge: %d "), CursorPoint.x, CursorPoint.y, TopEdge, BottomEdge, LeftEdge, RightEdge);

										MessageHandler.SetCursorPos(FVector2D(cursor_point.x,cursor_point.y));
										self.last_cursor_point.x = cursor_point.x;
										self.last_cursor_point.y = cursor_point.y;

										self.num_pre_wrap_msgs_to_respect = 10;
										last_cursor_point_pre_wrap.X = cursor_pointNoWrap.x;
										last_cursor_point_pre_wrap.Y = cursor_pointNoWrap.y;
									}
									
									/*
									if (DeltaX != 0 || DeltaY != 0)
									{
										if (FMath::Abs(DeltaX) < DeltaWidthMax && FMath::Abs(DeltaY) < DeltaHeightMax)
										{
											UE_LOG(LogWindowsDesktop, Log, TEXT("Accept Delta X: %d Y: %d  ---- Last X: %d Y: %d ---- Cur X: %d Y: %d ---- Width: %d Height: %d"), DeltaX, DeltaY, LastCursorPoint.X, LastCursorPoint.Y, CursorPoint.x, CursorPoint.y, (int32)Width, (int32)Height);
										}
										else
										{
											UE_LOG(LogWindowsDesktop, Log, TEXT("IGNORE Delta X: %d Y: %d  ---- Last X: %d Y: %d ---- Cur X: %d Y: %d ---- Width: %d Height: %d"), DeltaX, DeltaY, LastCursorPoint.X, LastCursorPoint.Y, CursorPoint.x, CursorPoint.y, (int32)Width, (int32)Height);
										}
									}*/

									self.last_cursor_point.x = cursor_point.x;
									self.last_cursor_point.y = cursor_point.y;
								}

								// Send a delta assuming it's not zero or beyond our max delta 
								if ((delta_x != 0 || delta_y != 0) && delta_x.abs() < delta_width_max && delta_y.abs() < delta_height_max)
								{
									self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam, delta_x, delta_y, MOUSE_MOVE_RELATIVE);
								}
								return 1;
                                } else {
                                    // Since the raw input is coming in as absolute it is likely the user is using a tablet
                                    // or perhaps is interacting through a virtual desktop
                                    self.defer_message(
                                        &current_native_event_window,
                                        hwnd,
                                        msg,
                                        wparam,
                                        lparam,
                                        0,
                                        0,
                                        MOUSE_MOVE_ABSOLUTE as u32,
                                    );
                                    return 1;
                                }
    
                                // Since raw input is coming in as relative it is likely a traditional mouse device
                                let x_pos_relative = raw_mouse.lLastX;
                                let y_pos_relative = raw_mouse.lLastY;
    
                                self.defer_message(
                                    &current_native_event_window,
                                    hwnd,
                                    msg,
                                    wparam,
                                    lparam,
                                    x_pos_relative,
                                    y_pos_relative,
                                    MOUSE_MOVE_RELATIVE as u32,
                                );
                                return 1;
                            }
                        }
                    }
                    WM_NCCALCSIZE => {
                        let borrowed_wnd = current_native_event_window.borrow();
                        let windef = borrowed_wnd.get_definition();
                        // Let windows absorb this message if using the standard border
                        if wparam != 0 && !windef.has_os_window_border {
                            // Borderless game windows are not actually borderless, they have a thick border that we simply draw game content over (client
                            // rect contains the window border). When maximized Windows will bleed our border over the edges of the monitor. So that we
                            // don't draw content we are going to later discard, we change a maximized window's size and position so that the entire
                            // window rect (including the border) sits inside the monitor. The size adjustments here will be sent to WM_MOVE and
                            // WM_SIZE and the window will still be considered maximized.
                            if windef.window_type == WindowType::GameWindow
                                && current_native_event_window.borrow().is_maximized()
                            {
                                // Ask the system for the window border size as this is the amount that Windows will bleed our window over the edge
                                // of our desired space. The value returned by current_native_event_window will be incorrect for our usage here as it
                                // refers to the border of the window that Slate should consider.
                                let mut window_info: WINDOWINFO = mem::zeroed();
                                window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                                GetWindowInfo(hwnd, &mut window_info);

                                // A pointer to the window size data that Windows will use is passed to us in lparam
                                let resizing_rects: &mut NCCALCSIZE_PARAMS =
                                    &mut *(lparam as usize as *mut NCCALCSIZE_PARAMS) as &mut _;

                                // The first rectangle contains the client rectangle of the resized window.
                                // Decrease window size on all sides by the border size.
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
                                (&mut *resizing_rects.lppos).x +=
                                    window_info.cxWindowBorders as i32;
                                (&mut *resizing_rects.lppos).y +=
                                    window_info.cxWindowBorders as i32;
                                (&mut *resizing_rects.lppos).cx -=
                                    2 * window_info.cxWindowBorders as i32;
                                (&mut *resizing_rects.lppos).cy -=
                                    2 * window_info.cxWindowBorders as i32;

                                // Informs Windows to use the values as we altered them.
                                return WVR_VALIDRECTS as i32;
                            }
                            return 0;
                        }
                    }
                    WM_SHOWWINDOW => {
                        self.defer_message(
                            &current_native_event_window,
                            hwnd,
                            msg,
                            wparam,
                            lparam,
                            0,
                            0,
                            0,
                        );
                    }
                    case WM_SIZE => {
				self.defer_message(&current_native_event_window, hwnd, msg, wparam, lparam );

				let was_maximized = (wparam == SIZE_MAXIMIZED);
				let was_restored = (wparam == SIZE_RESTORED);

				if was_maximized || was_restored {
					//MessageHandler->OnWindowAction(CurrentNativeEventWindow, was_maximized ? EWindowAction::Maximize : EWindowAction::Restore);
				}

				return 0;
			}
            WM_SIZING => {
                let borrowed_wnd = current_native_event_window.borrow();
                let windef = borrowed_wnd.get_definition();
                if windef.should_preserve_aspect_ratio {
                    // The rect we get in lparam is window rect, but we need to preserve client's aspect ratio,
                    // so we need to find what the border and title bar sizes are, if window has them and adjust the rect.
                    let mut window_info: WINDOWINFO = mem::zeroed();
                    window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                    GetWindowInfo(hwnd, &mut window_info);

                    let mut test_rect: RECT = mem::zeroed();
                    AdjustWindowRectEx(
                        &mut test_rect,
                        window_info.dwStyle,
                        false,
                        window_info.dwExStyle,
                    );

                    let mut rect: RECT = {
                        let lprect = mem::transmute::<LPARAM, *const RECT>(lparam);
                        *lprect
                    };

                    rect.left -= test_rect.left;
                    rect.right -= test_rect.right;
                    rect.top -= test_rect.top;
                    rect.bottom -= test_rect.bottom;

                    let aspect_ratio =
                        current_native_event_window.borrow().get_aspect_ratio();
                    let new_width = rect.right - rect.left;
                    let new_height = rect.bottom - rect.top;

                    if wparam == WMSZ_LEFT as usize || wparam == WMSZ_RIGHT as usize {
                        let adjusted_height: i32 = new_width / aspect_ratio as i32;
                        rect.top -= (adjusted_height - new_height) / 2;
                        rect.bottom += (adjusted_height - new_height) / 2;
                        //break;
                    } else if wparam == WMSZ_TOP as usize || wparam == WMSZ_BOTTOM as usize
                    {
                        let adjusted_width: i32 = new_height * aspect_ratio as i32;
                        rect.left -= (adjusted_width - new_width) / 2;
                        rect.right += (adjusted_width - new_width) / 2;
                        //break;
                    } else if wparam == WMSZ_TOPLEFT as usize {
                        let adjusted_height: i32 = new_width / aspect_ratio as i32;
                        rect.top -= adjusted_height - new_height;
                        //break;
                    } else if wparam == WMSZ_TOPRIGHT as usize {
                        let adjusted_height: i32 = new_width / aspect_ratio as i32;
                        rect.top -= adjusted_height - new_height;
                        //break;
                    } else if wparam == WMSZ_BOTTOMLEFT as usize {
                        let adjusted_height: i32 = new_width / aspect_ratio as i32;
                        rect.bottom += adjusted_height - new_height;
                        //break;
                    } else if wparam == WMSZ_BOTTOMRIGHT as usize {
                        let adjusted_height: i32 = new_width / aspect_ratio as i32;
                        rect.bottom += adjusted_height - new_height;
                        //break;
                    }

                    AdjustWindowRectEx(
                        &mut rect,
                        window_info.dwStyle,
                        false,
                        window_info.dwExStyle,
                    );

                    return 1;
                }
            }
                    WM_GETMINMAXINFO => {
                        let mut min_max_info: MINMAXINFO = {
                            let mmi = mem::transmute::<LPARAM, *const MINMAXINFO>(lparam);
                            *mmi
                        };
                        let borrowed_wnd = current_native_event_window.borrow();
                        let windef = borrowed_wnd.get_definition();
                        let ref size_limits: WindowSizeLimits = windef.size_limits;

                        // We need to inflate the max values if using an OS window border
                        let mut border_width: i32 = 0;
                        let mut border_height: i32 = 0;
                        if windef.has_os_window_border {
                            let window_style = GetWindowLongW(hwnd, GWL_STYLE);
                            let window_ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

                            // This adjusts a zero rect to give us the size of the border
                            let mut border_rect: RECT = mem::zeroed();
                            AdjustWindowRectEx(
                                &mut border_rect,
                                window_style as u32,
                                false,
                                window_ex_style as u32,
                            );

                            border_width = border_rect.right - border_rect.left;
                            border_height = border_rect.bottom - border_rect.top;
                        }

                        // We always apply BorderWidth and BorderHeight since Slate always works with client area window sizes
                        min_max_info.ptMinTrackSize.x = size_limits
                            .get_min_width()
                            .unwrap_or(min_max_info.ptMinTrackSize.x as f32)
                            as i32;
                        min_max_info.ptMinTrackSize.y = size_limits
                            .get_min_height()
                            .unwrap_or(min_max_info.ptMinTrackSize.y as f32)
                            as i32;
                        min_max_info.ptMaxTrackSize.x = size_limits
                            .get_max_width()
                            .unwrap_or(min_max_info.ptMaxTrackSize.x as f32)
                            as i32
                            + border_width;
                        min_max_info.ptMaxTrackSize.y = size_limits
                            .get_max_height()
                            .unwrap_or(min_max_info.ptMaxTrackSize.y as f32)
                            as i32
                            + border_height;
                        return 0;
                    }
                    WM_ERASEBKGND => {
                        // Intercept background erasing to eliminate flicker.
                        // Return non-zero to indicate that we'll handle the erasing ourselves
                        return 1;
                    }
                    WM_NCACTIVATE => {
                        if !current_native_event_window
                            .borrow()
                            .get_definition()
                            .has_os_window_border
                        {
                            // Unless using the OS window border, intercept calls to prevent non-client area drawing a border upon activation or deactivation
                            // Return true to ensure standard activation happens
                            return 1;
                        }
                    }
                    WM_NCPAINT => {
                        if !current_native_event_window
                            .borrow()
                            .get_definition()
                            .has_os_window_border
                        {
                            // Unless using the OS window border, intercept calls to draw the non-client area - we do this ourselves
                            return 0;
                        }
                    }
                    WM_DESTROY => {
                        println!("about to delete references to windows after WM_DESTROY. Mutable borrow here.");
                        self.windows.borrow_mut()
                            .retain(|ref x| x != &&current_native_event_window);
                        return 0;
                    }
                    _ => {}
                }
            }
            DefWindowProcW(hwnd, msg, wparam, lparam) as i32
        }
    }
    /*void FWindowsApplication::GetInitialDisplayMetrics( FDisplayMetrics& OutDisplayMetrics ) const {
        OutDisplayMetrics = InitialDisplayMetrics;
    }*/
    fn is_keyboard_input_message(&self, msg: u32) -> bool {
        match msg {
            // Keyboard input notification messages...
            WM_CHAR | WM_SYSCHAR | WM_SYSKEYDOWN | WM_KEYDOWN | WM_SYSKEYUP | WM_KEYUP
            | WM_SYSCOMMAND => true,
            _ => false,
        }
    }
    fn is_mouse_input_message(&self, msg: u32) -> bool {
        match msg {
            // Mouse input notification messages...
            WM_MOUSEHWHEEL | WM_MOUSEWHEEL | WM_MOUSEHOVER | WM_MOUSELEAVE | WM_MOUSEMOVE
            | WM_NCMOUSEHOVER | WM_NCMOUSELEAVE | WM_NCMOUSEMOVE | WM_NCMBUTTONDBLCLK
            | WM_NCMBUTTONDOWN | WM_NCMBUTTONUP | WM_NCRBUTTONDBLCLK | WM_NCRBUTTONDOWN
            | WM_NCRBUTTONUP | WM_NCXBUTTONDBLCLK | WM_NCXBUTTONDOWN | WM_NCXBUTTONUP
            | WM_LBUTTONDBLCLK | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONDBLCLK
            | WM_MBUTTONDOWN | WM_MBUTTONUP | WM_RBUTTONDBLCLK | WM_RBUTTONDOWN | WM_RBUTTONUP
            | WM_XBUTTONDBLCLK | WM_XBUTTONDOWN | WM_XBUTTONUP => true,
            _ => false,
        }
    }
    fn is_input_message(&self, msg: u32) -> bool {
        if self.is_keyboard_input_message(msg) || self.is_mouse_input_message(msg) {
            return true;
        }

        match msg {
            // Raw input notification messages...
            WM_INPUT | WM_INPUT_DEVICE_CHANGE => true,
            _ => false,
        }
    }
    fn query_connected_mice(&mut self) {
        unsafe {
            let mut device_count: u32 = 0;

            GetRawInputDeviceList(
                ptr::null_mut(),
                &mut device_count,
                mem::size_of::<RAWINPUTDEVICELIST>() as u32,
            );
            if device_count == 0 {
                self.is_mouse_attached = false;
                return;
            }

            let mut device_list: Vec<RAWINPUTDEVICELIST> =
                Vec::with_capacity(device_count as usize);
            GetRawInputDeviceList(
                device_list.as_mut_ptr() as *mut RAWINPUTDEVICELIST,
                &mut device_count,
                mem::size_of::<RAWINPUTDEVICELIST>() as u32,
            );

            let mut mouse_count: i32 = 0;
            for device in device_list {
                let mut name_len: u32 = 0;
                let mut name: Vec<u8> = vec![];
                if device.dwType != RIM_TYPEMOUSE {
                    continue;
                }
                //Force the use of ANSI versions of these calls
                let ret = GetRawInputDeviceInfoA(
                    device.hDevice,
                    RIDI_DEVICENAME,
                    ptr::null_mut(),
                    &mut name_len,
                );
                if ret as i32 == -1 {
                    continue;
                }

                name.resize(name_len as usize + 1, 0);
                let ret = GetRawInputDeviceInfoA(
                    device.hDevice,
                    RIDI_DEVICENAME,
                    name.as_mut_ptr() as *mut c_void,
                    &mut name_len,
                );
                if ret as i32 == -1 {
                    continue;
                }

                let name_str = String::from_utf8(name).unwrap();
                let replacement = name_str.replace("#", "\\");
                /*
                 * Name XP starts with \??\, vista+ starts \\?\
                 * In the device list exists a fake Mouse device with the device name of RDP_MOU
                 * This is used for Remote Desktop so ignore it.
                 */
                if replacement.starts_with("\\??\\ROOT\\RDP_MOU\\")
                    || replacement.starts_with("\\\\?\\ROOT\\RDP_MOU\\")
                {
                    continue;
                }

                mouse_count += 1;
            }

            self.is_mouse_attached = mouse_count > 0;
        }
    }
    fn update_all_modifier_key_states(&mut self) {
        unsafe {
            self.modifier_key_state[ModifierKey::LeftShift as usize] =
                (GetAsyncKeyState(VK_LSHIFT.into()) as u16 & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::RightShift as usize] =
                (GetAsyncKeyState(VK_RSHIFT.into()) as u16 & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::LeftControl as usize] =
                (GetAsyncKeyState(VK_LCONTROL.into()) as u16 & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::RightControl as usize] =
                (GetAsyncKeyState(VK_RCONTROL.into()) as u16 & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::LeftAlt as usize] =
                (GetAsyncKeyState(VK_LMENU.into()) as u16 & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::RightAlt as usize] =
                (GetAsyncKeyState(VK_RMENU.into()) as u16 & 0x8000) != 0;
            self.modifier_key_state[ModifierKey::CapsLock as usize] =
                (GetKeyState(VK_CAPITAL.into()) & 0x0001) != 0;
        }
    }
    // Defined as a global so that it can be extern'd by UELibrary
    fn windows_application_wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        println!("Got in here with the globals");
        (unsafe {
            WINDOWS_APPLICATION
                .unwrap()
                .process_message(hwnd, msg, wparam, lparam)
        }) as isize
    }
    pub fn pump_messages(&self, time_delta: f32) {
        unsafe {
            let mut message: MSG = mem::zeroed();

            // standard Windows message handling
            while PeekMessageW(&mut message, 0, 0, 0, PM_REMOVE).0 != 0 {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            println!("In pump_messages, return value was 0");
        }
    }
    unsafe extern "system" fn app_wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        println!(
            "in app_wnd_proc, message is {0}, {:#06x} ({})",
            msg,
            if let Some(msg_str) = WINDOWS_MESSAGE_STRINGS.get(&msg) {
                msg_str
            } else {
                "Not found"
            }
        );
        Self::windows_application_wnd_proc(hwnd, msg, wparam, lparam)
    }
    fn defer_message(
        &self,
        native_window: &RefCell<WindowsWindow>,
        in_hwnd: HWND,
        in_message: u32,
        in_wparam: WPARAM,
        in_lparam: LPARAM,
        mouse_x: i32,
        mouse_y: i32,
        raw_input_flags: u32,
    ) {
        if self.allowed_to_defer_message_processing {
            self.deferred_messages.push(DeferredMessage);
        }
    }
    /*fn add_message_handler(&mut self, in_message_handler: &dyn IWindowsMessageHandler) {
        if !self.message_handlers.contains(&in_message_handler)
        self.message_handlers.push(in_message_handler);
    }*/
}

fn get_monitor_size_from_edid(
    h_dev_reg_key: RegKey,
    out_width: &mut i32,
    out_height: &mut i32,
) -> bool {
    for (name, value) in h_dev_reg_key.enum_values().map(|x| x.unwrap()) {
        if &name[..] != "EDID" {
            continue;
        } else {
            // EDID data format documented here:
            // http://en.wikipedia.org/wiki/EDID

            let detail_timing_descriptor_start_index: usize = 54;

            *out_width = ((value.bytes[detail_timing_descriptor_start_index + 4] as i32 >> 4) << 8)
                | value.bytes[detail_timing_descriptor_start_index + 2] as i32;
            *out_height = ((value.bytes[detail_timing_descriptor_start_index + 7] as i32 >> 4)
                << 8)
                | value.bytes[detail_timing_descriptor_start_index + 5] as i32;

            return true; // valid EDID found
        }
    }

    false
}

fn get_size_for_dev_id(target_dev_id: &String, width: &mut i32, height: &mut i32) -> bool {
    unsafe {
        let dev_info = SetupDiGetClassDevsExW(
            &GUID_DEVCLASS_MONITOR, //class GUID
            PWSTR("".to_wide_null().as_mut_ptr()),
            0,
            DIGCF_PRESENT,
            ptr::null_mut(),
            PWSTR("".to_wide_null().as_mut_ptr()),
            ptr::null_mut(),
        );

        if dev_info.is_null() {
            return false;
        }

        let mut res = false;
        let mut monitor_index = 0;
        loop {
            let err = Error::last_os_error();
            if err.raw_os_error().unwrap() == ERROR_NO_MORE_ITEMS as i32 {
                break;
            }
            let mut dev_info_data = SP_DEVINFO_DATA::default();
            dev_info_data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

            if SetupDiEnumDeviceInfo(dev_info, monitor_index, &mut dev_info_data).0 != 0 {
                let mut buffer = [0u16; MAX_DEVICE_ID_LEN as usize];
                if CM_Get_Device_IDW(
                    dev_info_data.DevInst,
                    PWSTR(buffer.as_mut_ptr()),
                    MAX_DEVICE_ID_LEN as u32,
                    0,
                ) == CR_SUCCESS
                {
                    let mut dev_id = String::from_utf16_lossy(&buffer[..]);
                    let idx = &dev_id[9..].find("\\").unwrap();
                    dev_id = dev_id[8..9 + *idx].to_string();
                    if &dev_id[..] == &target_dev_id[..] {
                        let h_dev_reg_key: HKEY = SetupDiOpenDevRegKey(
                            dev_info,
                            &mut dev_info_data,
                            DICS_FLAG_GLOBAL,
                            0,
                            DIREG_DEV,
                            KEY_READ,
                        );

                        if h_dev_reg_key != 0 && h_dev_reg_key != INVALID_HANDLE_VALUE.0 {
                            res = get_monitor_size_from_edid(
                                RegKey::predef(h_dev_reg_key as *mut _),
                                width,
                                height,
                            );
                            //advapi32::RegCloseKey(h_dev_reg_key);
                            break;
                        }
                    }
                }
            }
            monitor_index += 1;
        }

        if SetupDiDestroyDeviceInfoList(dev_info).0 == 0 {
            res = false;
        }

        res
    }
}

fn get_monitor_info(out_monitor_info: &mut Vec<MonitorInfo>) {
    unsafe {
        let mut display_device = DISPLAY_DEVICEW::default();
        display_device.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
        let mut device_index = 0; // device index

        let mut primary_device: *mut MonitorInfo = ptr::null_mut();
        out_monitor_info.reserve(2); // Reserve two slots, as that will be the most common maximum

        while EnumDisplayDevicesW(
            PWSTR("".to_wide_null().as_mut_ptr()),
            device_index,
            &mut display_device,
            0,
        )
        .0 != 0
        {
            if display_device.StateFlags & DISPLAY_DEVICE_ATTACHED_TO_DESKTOP > 0 {
                let mut monitor = DISPLAY_DEVICEW::default();
                monitor.cb = mem::size_of::<DISPLAY_DEVICEW>() as u32;
                let mut monitor_index = 0;

                while EnumDisplayDevicesW(
                    PWSTR(display_device.DeviceName.as_mut_ptr()),
                    monitor_index,
                    &mut monitor,
                    0,
                )
                .0 != 0
                {
                    if (monitor.StateFlags & DISPLAY_DEVICE_ACTIVE != 0)
                        && (monitor.StateFlags & DISPLAY_DEVICE_MIRRORING_DRIVER == 0)
                    {
                        let mut info = MonitorInfo::default();

                        let temp_str = String::from_utf16_lossy(&monitor.DeviceID[..]);
                        let idx = &temp_str[9..].find("\\").unwrap();
                        info.name += &temp_str[8..9 + *idx];
                        info.id = temp_str;
                        //info.name = info.id.Mid (8, Info.ID.Find (TEXT("\\"), ESearchCase::CaseSensitive, ESearchDir::FromStart, 9) - 8);

                        if get_size_for_dev_id(
                            &info.name,
                            &mut info.native_width,
                            &mut info.native_height,
                        ) {
                            let temp_str = String::from_utf16_lossy(&monitor.DeviceID[..]);
                            info.id = temp_str.trim_end_matches('\0').to_string();
                            info.is_primary =
                                (display_device.StateFlags & DISPLAY_DEVICE_PRIMARY_DEVICE) > 0;
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
#[derive(PartialEq, Debug, Clone, Default)]
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
            let mut out_display_metrics = DisplayMetrics::default();
            // Total screen size of the primary monitor
            out_display_metrics.primary_display_width = GetSystemMetrics(SM_CXSCREEN);
            out_display_metrics.primary_display_height = GetSystemMetrics(SM_CYSCREEN);

            // Get the screen rect of the primary monitor, excluding taskbar etc.
            let mut work_area_rect: RECT = mem::zeroed();
            if SystemParametersInfoW(SPI_GETWORKAREA, 0, mem::transmute(&mut work_area_rect), 0).0
                == 0
            {
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
            out_display_metrics.virtual_display_rect.left = GetSystemMetrics(SM_XVIRTUALSCREEN);
            out_display_metrics.virtual_display_rect.top = GetSystemMetrics(SM_YVIRTUALSCREEN);
            out_display_metrics.virtual_display_rect.right =
                out_display_metrics.virtual_display_rect.left
                    + GetSystemMetrics(SM_CXVIRTUALSCREEN);
            out_display_metrics.virtual_display_rect.bottom =
                out_display_metrics.virtual_display_rect.top + GetSystemMetrics(SM_CYVIRTUALSCREEN);

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
            self.title_safe_padding_size = (
                self.primary_display_width as f32 * half_unsafe_ratio,
                self.primary_display_height as f32 * half_unsafe_ratio,
            );
        }

        let action_safe_zone_ratio = self.get_debug_action_safe_zone_ratio();
        if action_safe_zone_ratio < 1.0 {
            let half_unsafe_ratio = (1.0 - action_safe_zone_ratio) * 0.5;
            self.action_safe_padding_size = (
                self.primary_display_width as f32 * half_unsafe_ratio,
                self.primary_display_height as f32 * half_unsafe_ratio,
            );
        }
    }
}
