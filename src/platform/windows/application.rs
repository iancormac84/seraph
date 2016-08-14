use dwmapi;
use platform::generic::application::PlatformRect;
use platform::generic::window::GenericWindow;
use platform::generic::window_definition::{WindowDefinition, WindowTransparency, WindowType};
use platform::windows::window::WindowsWindow;
use std::collections::BTreeMap;
use std::{mem, ptr};
use std::rc::{Rc, Weak};
use user32;
use winapi::{
    CS_DBLCLKS, DLGC_WANTALLKEYS, DWORD, FALSE, GWL_STYLE, GWL_EXSTYLE, HICON, HINSTANCE, HIWORD, HMONITOR, HWND, HRAWINPUT, IMN_CHANGECANDIDATE, IMN_CLOSECANDIDATE, IMN_CLOSESTATUSWINDOW,
    IMN_GUIDELINE, IMN_OPENCANDIDATE, IMN_OPENSTATUSWINDOW, IMN_PRIVATE, IMN_SETCANDIDATEPOS, IMN_SETCOMPOSITIONFONT, IMN_SETCOMPOSITIONWINDOW, IMN_SETCONVERSIONMODE, IMN_SETOPENSTATUS,
    IMN_SETSENTENCEMODE, IMN_SETSTATUSWINDOWPOS, IMR_CANDIDATEWINDOW, IMR_COMPOSITIONFONT, IMR_COMPOSITIONWINDOW, IMR_CONFIRMRECONVERTSTRING, IMR_DOCUMENTFEED, IMR_QUERYCHARPOSITION,
    IMR_RECONVERTSTRING, LPARAM, LPNCCALCSIZE_PARAMS, MINMAXINFO, MONITORINFO, MONITOR_DEFAULTTONEAREST, NCCALCSIZE_PARAMS, POINT, POINTL, RAWINPUTDEVICE, RAWINPUTHEADER, RECT, RIDEV_REMOVE,
    RID_INPUT, SM_CXSCREEN, SM_CYSCREEN, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SPI_GETWORKAREA, SW_RESTORE, TRUE, VK_SPACE, WINDOWINFO, WM_IME_CHAR,
    WM_IME_COMPOSITION, WM_IME_ENDCOMPOSITION, WM_IME_NOTIFY, WM_IME_REQUEST, WM_IME_SETCONTEXT, WM_IME_STARTCOMPOSITION, WM_INPUTLANGCHANGE, WM_INPUTLANGCHANGEREQUEST, WM_KEYDOWN, WM_KEYUP,
    WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_NCLBUTTONDOWN, WM_NCMBUTTONDOWN, WM_NCMOUSEMOVE,
    WM_NCRBUTTONDOWN, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN, WM_XBUTTONUP, WNDCLASSW, WPARAM
};

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
}

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
    pub fn register_class(&self, hinstance: HINSTANCE, hicon: HICON) -> bool {
        unsafe {
            let mut wc: WNDCLASSW = mem::zeroed();
            wc.style = CS_DBLCLKS; // We want to receive double clicks
            wc.lpfnWndProc = AppWndProc;
            wc.cbClsExtra = 0;
            wc.cbWndExtra = 0;
            wc.hInstance = hinstance;
            wc.hIcon = hicon;
            wc.hCursor = ptr::null_mut();  // We manage the cursor ourselves
            wc.hbrBackground = ptr::null_mut(); // Transparent
            wc.lpszMenuName = ptr::null_mut();
            wc.lpszClassName = FWindowsWindow::AppWindowClass;
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
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    return 0;
                },
                WM_IME_NOTIFY => {
                    //UE_LOG(LogWindowsDesktop, Verbose, TEXT("WM_IME_NOTIFY - %s"), IMNStrings.Find(wparam) ? *(IMNStrings[wparam]) : nullptr);
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    return 0;
                },
                WM_IME_REQUEST => {
                    //UE_LOG(LogWindowsDesktop, Verbose, TEXT("WM_IME_REQUEST - %s"), IMRStrings.Find(wparam) ? *(IMRStrings[wparam]) : nullptr);
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    return 0;
                },
                // Character
                WM_CHAR => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    return 0;
                },
                WM_SYSCHAR => {
                    if HIWORD(lparam) & 0x2000 != 0 && wparam == VK_SPACE {
                        // Do not handle Alt+Space so that it passes through and opens the window system menu
                        //break;
                    } else {
                        return 0;
                    }
                },
                WM_SYSKEYDOWN => {
                    // Alt-F4 or Alt+Space was pressed. 
                    // Allow alt+f4 to close the window and alt+space to open the window menu
                    if wparam != VK_F4 && wparam != VK_SPACE {
                        self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
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
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    // Handled
                    return 0;
                },
                WM_SETCURSOR => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);

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

                    TScopedPointer<uint8> RawData(new uint8[Size]);

                    if user32::GetRawInputData(lparam as HRAWINPUT, RID_INPUT, RawData.GetOwnedPointer(), &size, mem::size_of::<RAWINPUTHEADER>() as u32) == size {
                        const RAWINPUT* const Raw = (const RAWINPUT* const)RawData.GetOwnedPointer();

                        if (Raw->header.dwType == RIM_TYPEMOUSE) {
                            const bool IsAbsoluteInput = (Raw->data.mouse.usFlags & MOUSE_MOVE_ABSOLUTE) == MOUSE_MOVE_ABSOLUTE;
                            if IsAbsoluteInput {
                                // Since the raw input is coming in as absolute it is likely the user is using a tablet
                                // or perhaps is interacting through a virtual desktop
                                self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam, 0, 0, MOUSE_MOVE_ABSOLUTE);
                                return 1;
                            }

                            // Since raw input is coming in as relative it is likely a traditional mouse device
                            const int xPosRelative = Raw->data.mouse.lLastX;
                            const int yPosRelative = Raw->data.mouse.lLastY;

                            self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam, xPosRelative, yPosRelative, MOUSE_MOVE_RELATIVE);
                            return 1;
                        }
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
                            return WVR_VALIDRECTS;
                        }
                        return 0;
                    }
                },
                //break;
                WM_SHOWWINDOW => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                },
                //break;
                WM_SIZE => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    return 0;
                },
                //break;
                WM_SIZING => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam, 0, 0);
        
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

                        const float AspectRatio = current_native_event_window->GetAspectRatio();
                        int32 NewWidth = rect.right - rect.left;
                        int32 NewHeight = rect.bottom - rect.top;

                        match wparam {
                            WMSZ_LEFT | WMSZ_RIGHT => {
                                int32 AdjustedHeight = NewWidth / AspectRatio;
                                rect.top -= (AdjustedHeight - NewHeight) / 2;
                                rect.bottom += (AdjustedHeight - NewHeight) / 2;
                                //break;
                            },
                            WMSZ_TOP | WMSZ_BOTTOM => {
                                int32 AdjustedWidth = NewHeight * AspectRatio;
                                rect.left -= (AdjustedWidth - NewWidth) / 2;
                                rect.right += (AdjustedWidth - NewWidth) / 2;
                                //break;
                            },
                            WMSZ_TOPLEFT => {
                                int32 AdjustedHeight = NewWidth / AspectRatio;
                                rect.top -= AdjustedHeight - NewHeight;
                                //break;
                            },
                            WMSZ_TOPRIGHT => {
                                int32 AdjustedHeight = NewWidth / AspectRatio;
                                rect.top -= AdjustedHeight - NewHeight;
                                //break;
                            },
                            WMSZ_BOTTOMLEFT => {
                                int32 AdjustedHeight = NewWidth / AspectRatio;
                                rect.bottom += AdjustedHeight - NewHeight;
                                //break;
                            },
                            WMSZ_BOTTOMRIGHT => {
                                int32 AdjustedHeight = NewWidth / AspectRatio;
                                rect.bottom += AdjustedHeight - NewHeight;
                                //break;
                            }
                        }

                        user32::AdjustWindowRectEx(Rect, window_info.dwStyle, false, window_info.dwExStyle);

                        return TRUE;
                    }
                }
                //break;
                WM_ENTERSIZEMOVE => {
                    bInModalSizeLoop = true;
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam, 0, 0);
                },
                //break;
                WM_EXITSIZEMOVE => {
                    bInModalSizeLoop = false;
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam, 0, 0);
                },
                //break;
                WM_MOVE => {
                    // client area position
                    const int32 NewX = (int)(short)(LOWORD(lparam));
                    const int32 NewY = (int)(short)(HIWORD(lparam));
                    FIntPoint NewPosition(NewX,NewY);

                    // Only cache the screen position if its not minimized
                    if FWindowsApplication::MinimizedWindowPosition != NewPosition {
                        MessageHandler->OnMovedWindow( current_native_event_window, NewX, NewY);

                        return 0;
                    }
                },
                //break;
                WM_NCHITTEST => {
                    // Only needed if not using the os window border as this is determined automatically
                    if !current_native_event_window.get_definition().has_os_window_border {
                        let rc_window: RECT = mem::uninitialized();
                        user32::GetWindowRect(hwnd, &mut rc_window);

                        const int32 LocalMouseX = (int)(short)(LOWORD(lparam)) - rcWindow.left;
                        const int32 LocalMouseY = (int)(short)(HIWORD(lparam)) - rcWindow.top;
                        if current_native_event_window.is_regular_window() {
                            EWindowZone::Type Zone;
                    
                            if MessageHandler->ShouldProcessUserInputMessages( current_native_event_window ) {
                                // Assumes this is not allowed to leave Slate or touch rendering
                                Zone = MessageHandler->GetWindowZoneForPoint( current_native_event_window, LocalMouseX, LocalMouseY);
                            } else {
                                // Default to client area so that we are able to see the feedback effect when attempting to click on a non-modal window when a modal window is active
                                // Any other window zones could have side effects and NotInWindow prevents the feedback effect.
                                Zone = EWindowZone::ClientArea;
                            }

                            static const LRESULT Results[] = {HTNOWHERE, HTTOPLEFT, HTTOP, HTTOPRIGHT, HTLEFT, HTCLIENT,
                                HTRIGHT, HTBOTTOMLEFT, HTBOTTOM, HTBOTTOMRIGHT,
                                HTCAPTION, HTMINBUTTON, HTMAXBUTTON, HTCLOSE, HTSYSMENU};

                            return Results[Zone];
                        }
                    }
                },
                //break;
                WM_DWMCOMPOSITIONCHANGED => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                },
                //break;
                // Window focus and activation
                WM_MOUSEACTIVATE => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                },
                //break;
                // Window focus and activation
                WM_ACTIVATE => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                },
                //break;
                WM_ACTIVATEAPP => {
                    // When window activation changes we are not in a modal size loop
                    bInModalSizeLoop = false;
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                },
                //break;
                WM_SETTINGCHANGE => {
                    // Convertible mode change
                    if (!lparam.is_null() && (wcscmp(TEXT("ConvertibleSlateMode"), (TCHAR *)lparam) == 0)) {
                        self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
                    }
                },
                //break;
                WM_PAINT => {
                    if bInModalSizeLoop && IsInGameThread() {
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
                    self.windows.Remove(current_native_event_window);
                    return 0;
                },
                //break;
                WM_CLOSE => {
                    self.defer_message(current_native_event_window, hwnd, msg, wparam, lparam);
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
                    int32 BorderWidth = 0;
                    int32 BorderHeight = 0;
                    if current_native_event_window.get_definition().has_os_window_border {
                        let window_style = user32::GetWindowLongW(hwnd, GWL_STYLE);
                        let window_ex_style = user32::GetWindowLongW(hwnd, GWL_EXSTYLE);

                        // This adjusts a zero rect to give us the size of the border
                        let mut border_rect: RECT = mem::zeroed();
                        user32::AdjustWindowRectEx(&mut border_rect, WindowStyle, false, WindowExStyle);

                        BorderWidth = BorderRect.right - BorderRect.left;
                        BorderHeight = BorderRect.bottom - BorderRect.top;
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
                    match wparam {
                        HTMINBUTTON => {
                            if !MessageHandler->OnWindowAction( current_native_event_window, WindowAction::ClickedNonClientArea) {
                                return 1;
                            }
                        },
                        //break;
                        HTMAXBUTTON => {
                            if !MessageHandler->OnWindowAction( current_native_event_window, WindowAction::ClickedNonClientArea) {
                                return 1;
                            }
                        },
                        //break;
                        HTCLOSE => {
                            if !MessageHandler->OnWindowAction( current_native_event_window, WindowAction::ClickedNonClientArea) {
                                return 1;
                            }
                        },
                        //break;
                        HTCAPTION => {
                            if !MessageHandler->OnWindowAction( current_native_event_window, WindowAction::ClickedNonClientArea) {
                                return 1;
                            }
                        }
                        //break;
                    }
                },
                //break;
                WM_DISPLAYCHANGE => {
                    // Slate needs to know when desktop size changes.
                    let mut display_metrics: DisplayMetrics::new();
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
        user32::DefWindowProcW(hwnd, msg, wparam, lparam)
    }
    /*void FWindowsApplication::GetInitialDisplayMetrics( FDisplayMetrics& OutDisplayMetrics ) const {
        OutDisplayMetrics = InitialDisplayMetrics;
    }*/
}

//TODO this struct has cross-platform applications, so it shouldn't be implemented within the Windows-specific files. 
pub struct DisplayMetrics {
    primary_display_width: i32,
    primary_display_height: i32,
    monitor_info: Vec<MonitorInfo>,
    primary_display_work_area_rect: PlatformRect,
    virtual_display_rect: PlatformRect,
    //TODO: The following should be a Vector2D
    title_safe_padding_size: (i32, i32),
    //TODO: The following should be a Vector2D
    action_safe_padding_size: (i32, i32),
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
            if !user32::SystemParametersInfo(SPI_GETWORKAREA, 0, &mut work_area_rect, 0) {
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
            user32::GetMonitorInfo(out_display_metrics.monitor_info);

            // Apply the debug safe zones
            out_display_metrics.ApplyDefaultSafeZones();
        }
    }
}