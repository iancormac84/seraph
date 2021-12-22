use crate::generic::window::{GenericWindow, WindowMode};
use crate::generic::window_definition::{
    WindowActivationPolicy, WindowDefinition, WindowTransparency, WindowType,
};
use crate::windows::application::WindowsApplication;
use crate::windows::application::WINDOWS_APPLICATION;
use crate::windows::utils::ToWide;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    cmp, fmt, io, mem,
    os::raw::c_void,
    ptr,
    rc::Rc,
    sync::{Arc, Weak},
};
use windows::Win32::{
    Foundation::{BOOL, HINSTANCE, HWND, PWSTR, RECT},
    Graphics::{
        Dwm::{
            DwmExtendFrameIntoClientArea, DwmSetWindowAttribute, DWMNCRP_DISABLED,
            DWMWA_ALLOW_NCPAINT, DWMWA_NCRENDERING_POLICY,
        },
        Gdi::{
            CreateRectRgn, CreateRoundRectRgn, DeleteObject, GetMonitorInfoW, MonitorFromWindow,
            PtInRegion, SetWindowRgn, HRGN, MONITORINFO, MONITOR_DEFAULTTONEAREST,
            MONITOR_DEFAULTTOPRIMARY,
        },
    },
    System::Ole::RevokeDragDrop,
    UI::{
        Controls::MARGINS,
        Input::KeyboardAndMouse::{
            EnableWindow, GetFocus, IsWindowEnabled, SetActiveWindow, SetFocus,
        },
        WindowsAndMessaging::{
            AdjustWindowRectEx, CreateWindowExW, GetClientRect, GetForegroundWindow,
            GetSystemMetrics, GetWindowInfo, GetWindowLongW, GetWindowPlacement, GetWindowRect,
            IsIconic, IsWindow, IsZoomed, SetLayeredWindowAttributes, SetWindowLongW,
            SetWindowPlacement, SetWindowPos, SetWindowTextW, ShowWindow, GWL_EXSTYLE, GWL_STYLE,
            HMENU, HWND_TOP, HWND_TOPMOST, LWA_ALPHA, MB_ICONEXCLAMATION, MB_OK, SM_CYCAPTION,
            SM_REMOTESESSION, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER,
            SWP_NOREDRAW, SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_MAXIMIZE,
            SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWMAXIMIZED, SW_SHOWMINNOACTIVE, SW_SHOWNA,
            SW_SHOWNOACTIVATE, WINDOWINFO, WINDOWPLACEMENT, WS_BORDER, WS_CAPTION, WS_CLIPCHILDREN,
            WS_CLIPSIBLINGS, WS_EX_APPWINDOW, WS_EX_COMPOSITED, WS_EX_LAYERED, WS_EX_TOOLWINDOW,
            WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX,
            WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_THICKFRAME,
        },
    },
};

pub const APP_WINDOW_CLASS: &'static str = "CormacWindow";

//TODO can I make this capable of clone? I want to try this so I don't have to do a clone in the WindowsApplication::find_window_by_hwnd method.
pub struct WindowsWindow {
    pub app_window_class: &'static str,
    pub owning_application: Weak<WindowsApplication>,
    hwnd: Cell<HWND>,
    region_height: Cell<i32>,
    region_width: Cell<i32>,
    window_mode: WindowMode,
    ole_reference_count: u32,
    pre_fullscreen_window_placement: WINDOWPLACEMENT,
    pre_parent_minimized_window_placement: WINDOWPLACEMENT,
    virtual_height: Cell<i32>,
    virtual_width: Cell<i32>,
    aspect_ratio: Cell<f32>,
    is_visible: Cell<bool>,
    is_first_time_visible: Cell<bool>,
    initially_minimized: Cell<bool>,
    initially_maximized: Cell<bool>,
    dpi_scale_factor: f32,
    handle_manual_dpi_changes: bool,
    window_definitions: Rc<WindowDefinition>,
}

impl fmt::Debug for WindowsWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pre_fullscreen_window_placement = format!("WINDOWPLACEMENT {{ length: {}, flags: {}, showCmd: {}, ptMinPosition: POINT {{ x: {}, y: {} }}, ptMaxPosition: POINT {{ x: {}, y: {} }}, rcNormalPosition: RECT {{ left: {}, top: {}, right: {}, bottom: {} }} }}", &self.pre_fullscreen_window_placement.length, &self.pre_fullscreen_window_placement.flags, &self.pre_fullscreen_window_placement.showCmd, &self.pre_fullscreen_window_placement.ptMinPosition.x, &self.pre_fullscreen_window_placement.ptMinPosition.y, &self.pre_fullscreen_window_placement.ptMaxPosition.x, &self.pre_fullscreen_window_placement.ptMaxPosition.y, &self.pre_fullscreen_window_placement.rcNormalPosition.left, &self.pre_fullscreen_window_placement.rcNormalPosition.top, &self.pre_fullscreen_window_placement.rcNormalPosition.right, &self.pre_fullscreen_window_placement.rcNormalPosition.bottom);
        let pre_parent_minimized_window_placement = format!("WINDOWPLACEMENT {{ length: {}, flags: {}, showCmd: {}, ptMinPosition: POINT {{ x: {}, y: {} }}, ptMaxPosition: POINT {{ x: {}, y: {} }}, rcNormalPosition: RECT {{ left: {}, top: {}, right: {}, bottom: {} }} }}", &self.pre_parent_minimized_window_placement.length, &self.pre_parent_minimized_window_placement.flags, &self.pre_parent_minimized_window_placement.showCmd, &self.pre_parent_minimized_window_placement.ptMinPosition.x, &self.pre_parent_minimized_window_placement.ptMinPosition.y, &self.pre_parent_minimized_window_placement.ptMaxPosition.x, &self.pre_parent_minimized_window_placement.ptMaxPosition.y, &self.pre_parent_minimized_window_placement.rcNormalPosition.left, &self.pre_parent_minimized_window_placement.rcNormalPosition.top, &self.pre_parent_minimized_window_placement.rcNormalPosition.right, &self.pre_parent_minimized_window_placement.rcNormalPosition.bottom);
        f.debug_struct("WindowsWindow")
            .field("app_window_class", &self.app_window_class)
            .field("owning_application", &self.owning_application)
            .field("hwnd", &self.hwnd)
            .field("region_height", &self.region_height)
            .field("region_width", &self.region_width)
            .field("window_mode", &self.window_mode)
            .field("ole_reference_count", &self.ole_reference_count)
            .field(
                "pre_fullscreen_window_placement",
                &pre_fullscreen_window_placement,
            )
            .field(
                "pre_parent_minimized_window_placement",
                &pre_parent_minimized_window_placement,
            )
            .field("virtual_height", &self.virtual_height)
            .field("virtual_width", &self.virtual_width)
            .field("aspect_ratio", &self.aspect_ratio)
            .field("is_visible", &self.is_visible)
            .field("is_first_time_visible", &self.is_first_time_visible)
            .field("initially_minimized", &self.initially_minimized)
            .field("initially_maximized", &self.initially_maximized)
            .field("dpi_scale_factor", &self.dpi_scale_factor)
            .field("handle_manual_dpi_changes", &self.handle_manual_dpi_changes)
            .field("window_definitions", &self.window_definitions)
            .finish()
    }
}

impl WindowsWindow {
    pub fn new() -> WindowsWindow {
        unsafe {
            let mut wnd_plcment: WINDOWPLACEMENT = mem::zeroed();
            let mut wnd_plcment1: WINDOWPLACEMENT = mem::zeroed();
            wnd_plcment.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
            wnd_plcment1.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
            WindowsWindow {
                app_window_class: APP_WINDOW_CLASS,
                owning_application: Arc::downgrade(WINDOWS_APPLICATION.unwrap()),
                hwnd: Cell::new(0),
                region_height: Cell::new(-1),
                region_width: Cell::new(-1),
                window_mode: WindowMode::Windowed,
                ole_reference_count: 0,
                pre_fullscreen_window_placement: wnd_plcment,
                pre_parent_minimized_window_placement: wnd_plcment1,
                virtual_height: Cell::new(0),
                virtual_width: Cell::new(0),
                aspect_ratio: Cell::new(1.0f32),
                is_visible: Cell::new(false),
                is_first_time_visible: Cell::new(true),
                initially_minimized: Cell::new(false),
                initially_maximized: Cell::new(false),
                dpi_scale_factor: 1.0,
                handle_manual_dpi_changes: false,
                window_definitions: Rc::new(WindowDefinition::default()),
            }
        }
    }
    pub fn make() -> Rc<RefCell<WindowsWindow>> {
        println!("WindowsWindow::make");
        Rc::new(RefCell::new(WindowsWindow::new()))
    }
    pub fn initialize(
        &self,
        definition: &Rc<WindowDefinition>,
        instance: HINSTANCE,
        parent: Option<Rc<WindowsWindow>>,
        show_immediately: bool,
    ) {
        println!("Just reach in initialize");

        self.window_definitions.borrow_mut().clone_from(definition);
        println!(
            "definition strong count is {}",
            Rc::strong_count(&definition)
        );
        println!("definition weak count is {}", Rc::weak_count(&definition));

        let mut window_ex_style: u32 = 0;
        let mut window_style: u32 = 0;

        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);

        let x_initial_rect = windef_borrow.x_desired_position_on_screen;
        let y_initial_rect = windef_borrow.y_desired_position_on_screen;

        let width_initial = windef_borrow.width_desired_on_screen;
        let height_initial = windef_borrow.height_desired_on_screen;

        let mut client_x = x_initial_rect as i32;
        let mut client_y = y_initial_rect as i32;
        let mut client_width = width_initial as i32;
        let mut client_height = height_initial as i32;
        let mut window_x = client_x;
        let mut window_y = client_y;
        let mut window_width = client_width;
        let mut window_height = client_height;

        let application_supports_per_pixel_blending = unsafe {
            WINDOWS_APPLICATION
                .unwrap()
                .get_window_transparency_support()
        } == WindowTransparency::PerPixel;

        if !windef_borrow.has_os_window_border {
            window_ex_style = WS_EX_WINDOWEDGE;

            if windef_borrow.transparency_support == WindowTransparency::PerWindow {
                window_ex_style |= WS_EX_LAYERED;
            } else if windef_borrow.transparency_support == WindowTransparency::PerPixel {
                if application_supports_per_pixel_blending {
                    window_ex_style |= WS_EX_COMPOSITED;
                }
            }
            window_style = WS_POPUP | WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
            if windef_borrow.appears_in_taskbar {
                window_ex_style |= WS_EX_APPWINDOW;
            } else {
                window_ex_style |= WS_EX_TOOLWINDOW;
            }
            if windef_borrow.is_topmost_window {
                // Tool tips are always top most windows
                window_ex_style |= WS_EX_TOPMOST;
            }
            if !windef_borrow.accepts_input {
                // Window should never get input
                window_ex_style |= WS_EX_TRANSPARENT;
            }
        } else {
            // OS Window border setup
            window_ex_style = WS_EX_APPWINDOW;
            window_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
            if windef_borrow.is_regular_window {
                if windef_borrow.supports_maximize {
                    window_style |= WS_MAXIMIZEBOX;
                }

                if windef_borrow.supports_minimize {
                    window_style |= WS_MINIMIZEBOX;
                }

                if windef_borrow.has_sizing_frame {
                    window_style |= WS_THICKFRAME;
                } else {
                    window_style |= WS_BORDER;
                }
            } else {
                window_style |= WS_POPUP | WS_BORDER;
            }

            // X,Y, Width, Height defines the top-left pixel of the client area on the screen
            // This adjusts a zero rect to give us the size of the border
            let mut border_rect = RECT::default();
            unsafe {
                AdjustWindowRectEx(&mut border_rect, window_style, false, window_ex_style);
            }

            // Border rect size is negative - see MoveWindowTo
            window_x += border_rect.left;
            window_y += border_rect.top;

            // Inflate the window size by the OS border
            window_width += border_rect.right - border_rect.left;
            window_height += border_rect.bottom - border_rect.top;
        }

        //TODO: parent window may be null, but I'm using Rc to hold parent window, which I think implies that parent window can't be null. Fix.
        println!("WindowsWindow self is {:p}", self);
        //let len = Weak::upgrade(&self.owning_application).unwrap().borrow().windows.len();
        //println!("The vec len is {}", len);
        println!("self debug is {:#?}", self);
        println!("self.hwnd is {:?}", self.hwnd.get());
        println!(
            "self.owning_application is {:p}",
            &Weak::upgrade(&self.owning_application)
        );
        self.hwnd.set(
            match create_window(
                window_ex_style,
                PWSTR(APP_WINDOW_CLASS.to_wide_null().as_mut_ptr()),
                PWSTR((&windef_borrow.title[..]).to_wide_null().as_mut_ptr()),
                window_style,
                window_x,
                window_y,
                window_width,
                window_height,
                if parent.is_some() {
                    parent.unwrap().hwnd.get()
                } else {
                    0
                },
                0,
                instance,
                ptr::null_mut(),
            ) {
                Ok(hwnd) => hwnd,
                Err(err) => {
                    wui_abort!(
                        "\
                            Window Creation Failed!: {}\r\n\
                        ",
                        err
                    )
                }
            },
        );
        println!("self.hwnd is now {:?}", self.hwnd.get());

        println!("CreateWindowExW called");
        self.virtual_width.set(client_width);
        self.virtual_height.set(client_height);

        // We call reshape window here because we didn't take into account the non-client area
        // in the initial creation of the window. Slate should only pass client area dimensions.
        // Reshape window may resize the window if the non-client area is encroaching on our
        // desired client area space.
        self.reshape_window(
            &mut client_x,
            &mut client_y,
            &mut client_width,
            &mut client_height,
        );

        /*if window.hwnd.is_null() {
            unsafe {
                user32::MessageBoxW(ptr::null_mut(),
                    format!("Window Creation Failed! {}", io::Error::last_os_error()).to_wide_null().as_ptr(),
                    "Error!".to_wide_null().as_ptr(),
                    MB_ICONEXCLAMATION | MB_OK
                );
            }
            return;
        }*/
        if windef_borrow.transparency_support == WindowTransparency::PerWindow {
            let opacity = windef_borrow.opacity;
            self.set_opacity(opacity);
        }
        if !windef_borrow.has_os_window_border {
            let rendering_policy = DWMNCRP_DISABLED;

            unsafe {
                let res = DwmSetWindowAttribute(
                    self.hwnd.get(),
                    mem::transmute(DWMWA_NCRENDERING_POLICY),
                    mem::transmute(&rendering_policy),
                    mem::size_of::<u32>() as u32,
                );
                if let Err(error) = res {
                    println!("Warning: {}", error);
                }
                let enable_nc_paint = false;
                let res = DwmSetWindowAttribute(
                    self.hwnd.get(),
                    mem::transmute(DWMWA_ALLOW_NCPAINT),
                    mem::transmute(&enable_nc_paint),
                    mem::size_of::<BOOL>() as u32,
                );
                if let Err(error) = res {
                    println!("Warning: {}", error);
                }
                if application_supports_per_pixel_blending
                    && windef_borrow.transparency_support == WindowTransparency::PerPixel
                {
                    let mut margins = MARGINS {
                        cxLeftWidth: -1,
                        cxRightWidth: -1,
                        cyTopHeight: -1,
                        cyBottomHeight: -1,
                    };
                    let res = DwmExtendFrameIntoClientArea(self.hwnd.get(), &margins);
                    if let Err(error) = res {
                        println!("Warning: {}", error);
                    }
                }
            }
        }

        if windef_borrow.is_regular_window && !windef_borrow.has_os_window_border {
            window_style |= WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU;

            if windef_borrow.supports_maximize {
                window_style |= WS_MAXIMIZEBOX;
            }
            if windef_borrow.supports_minimize {
                window_style |= WS_MINIMIZEBOX;
            }
            if windef_borrow.has_sizing_frame {
                window_style |= WS_THICKFRAME;
            }

            unsafe {
                if SetWindowLongW(self.hwnd.get(), GWL_STYLE, window_style as i32) == 0 {
                    println!("Warning: {}", io::Error::last_os_error());
                }
                SetWindowPos(
                    self.hwnd.get(),
                    0,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                );
                self.adjust_window_region(client_width, client_height); //Adjusts region{width, height}
            }
        }
    }
    pub fn get_hwnd(&self) -> HWND {
        self.hwnd.get()
    }
    pub fn make_window_region_object(&self) -> HRGN {
        let mut region: HRGN;
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if self.region_width.get() != -1 && self.region_height.get() != -1 {
            if self.is_maximized() {
                if windef_borrow.window_type == WindowType::GameWindow
                    && !windef_borrow.has_os_window_border
                {
                    // Windows caches the cxWindowBorders size at window creation. Even if borders are removed or resized Windows will continue to use this value when evaluating regions
                    // and sizing windows. When maximized this means that our window position will be offset from the screen origin by (-cxWindowBorders,-cxWindowBorders). We want to
                    // display only the region within the maximized screen area, so offset our upper left and lower right by cxWindowBorders.
                    unsafe {
                        let mut window_info: WINDOWINFO = mem::zeroed();
                        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
                        GetWindowInfo(self.hwnd.get(), &mut window_info);

                        region = CreateRectRgn(
                            window_info.cxWindowBorders as i32,
                            window_info.cxWindowBorders as i32,
                            self.region_width.get() + window_info.cxWindowBorders as i32,
                            self.region_height.get() + window_info.cxWindowBorders as i32,
                        );
                    }
                } else {
                    let window_border_size = self.get_window_border_size();
                    unsafe {
                        region = CreateRectRgn(
                            window_border_size as i32,
                            window_border_size as i32,
                            self.region_width.get() - window_border_size as i32,
                            self.region_height.get() - window_border_size as i32,
                        );
                    }
                }
            } else {
                let use_corner_radius = self.window_mode == WindowMode::Windowed
                    && windef_borrow.transparency_support != WindowTransparency::PerPixel
                    && windef_borrow.corner_radius > 0;
                if use_corner_radius {
                    unsafe {
                        region = CreateRoundRectRgn(
                            0,
                            0,
                            self.region_width.get() + 1,
                            self.region_height.get() + 1,
                            windef_borrow.corner_radius,
                            windef_borrow.corner_radius,
                        );
                    }
                } else {
                    unsafe {
                        region =
                            CreateRectRgn(0, 0, self.region_width.get(), self.region_height.get());
                    }
                }
            }
        } else {
            let mut rect_wnd = RECT::default();
            unsafe {
                GetWindowRect(self.hwnd.get(), &mut rect_wnd);
                region = CreateRectRgn(
                    0,
                    0,
                    rect_wnd.right - rect_wnd.left,
                    rect_wnd.bottom - rect_wnd.top,
                );
            }
        }
        region
    }
    pub fn adjust_window_region(&self, width: i32, height: i32) {
        self.region_width.set(width);
        self.region_height.set(height);
        let region = self.make_window_region_object();
        unsafe {
            if SetWindowRgn(self.hwnd.get(), region, false) == 0 {
                println!("Warning: {}", io::Error::last_os_error());
            }
        }
    }
    pub fn on_parent_window_minimized(&mut self) {
        // This function is called from SW_PARENTCLOSING, because there's a bug in Win32 that causes the equivalent SW_PARENTOPENING
        // message to restore in an incorrect state (eg, it will lose the maximized status of the window)
        // To work around this, we cache our window placement here so that we can restore it later (see OnParentWindowRestored)
        unsafe {
            GetWindowPlacement(
                self.hwnd.get(),
                &mut self.pre_parent_minimized_window_placement,
            );
        }
    }
    pub fn on_parent_window_restored(&self) {
        // This function is called from SW_PARENTOPENING so that we can restore the window placement that was cached in OnParentWindowMinimized
        unsafe {
            SetWindowPlacement(self.hwnd.get(), &self.pre_parent_minimized_window_placement);
        }
    }
    pub fn is_enabled(&self) -> bool {
        let res = unsafe { !!IsWindowEnabled(self.hwnd.get()) };
        res.0 != 0
    }
    pub fn is_regular_window(&self) -> bool {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        windef_borrow.is_regular_window
    }
    pub fn on_transparency_support_changed(&mut self, new_transparency: WindowTransparency) {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if windef_borrow.transparency_support == WindowTransparency::PerPixel {
            let style = unsafe { GetWindowLongW(self.hwnd.get(), GWL_EXSTYLE) };

            if new_transparency == WindowTransparency::PerPixel {
                unsafe {
                    SetWindowLongW(
                        self.hwnd.get(),
                        GWL_EXSTYLE,
                        style | WS_EX_COMPOSITED as i32,
                    )
                };
                let margins: MARGINS = MARGINS {
                    cxLeftWidth: -1,
                    cxRightWidth: -1,
                    cyTopHeight: -1,
                    cyBottomHeight: -1,
                };

                let res = unsafe { DwmExtendFrameIntoClientArea(self.hwnd.get(), &margins) };
                if let Err(error) = res {
                    println!("Warning: {}", error);
                }
            } else {
                unsafe {
                    SetWindowLongW(
                        self.hwnd.get(),
                        GWL_EXSTYLE,
                        style & !WS_EX_COMPOSITED as i32,
                    )
                };
            }

            // Must call SWP_FRAMECHANGED when updating the style attribute of a window in order to update internal caches (according to MSDN)
            unsafe {
                SetWindowPos(
                    self.hwnd.get(),
                    0,
                    0,
                    0,
                    0,
                    0,
                    SWP_FRAMECHANGED
                        | SWP_NOACTIVATE
                        | SWP_NOMOVE
                        | SWP_NOOWNERZORDER
                        | SWP_NOREDRAW
                        | SWP_NOSIZE
                        | SWP_NOSENDCHANGING
                        | SWP_NOZORDER,
                )
            };
        }
    }
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio.get()
    }
}

impl GenericWindow for WindowsWindow {
    fn reshape_window(
        &self,
        new_x: &mut i32,
        new_y: &mut i32,
        new_width: &mut i32,
        new_height: &mut i32,
    ) {
        let mut window_info = WINDOWINFO::default();
        unsafe {
            window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
            GetWindowInfo(self.hwnd.get(), &mut window_info);
        }

        self.aspect_ratio
            .set(*new_width as f32 / *new_height as f32);

        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);

        if windef_borrow.has_os_window_border {
            let mut border_rect: RECT = unsafe { mem::zeroed() };
            unsafe {
                AdjustWindowRectEx(
                    &mut border_rect,
                    window_info.dwStyle,
                    false,
                    window_info.dwExStyle,
                )
            };

            *new_x += border_rect.left;
            *new_y += border_rect.top;

            *new_width += border_rect.right - border_rect.left;
            *new_height += border_rect.bottom - border_rect.top;
        }
        let window_x = new_x;
        let window_y = new_y;

        let virtual_size_changed =
            *new_width != self.virtual_width.get() || *new_height != self.virtual_height.get();
        self.virtual_width.set(*new_width);
        self.virtual_height.set(*new_height);

        if windef_borrow.size_will_change_often {
            let old_window_rect = window_info.rcWindow;
            let old_width = old_window_rect.right - old_window_rect.left;
            let old_height = old_window_rect.bottom - old_window_rect.top;

            let min_retained_width = if windef_borrow.expected_max_width != -1 {
                windef_borrow.expected_max_width
            } else {
                old_width
            };
            let min_retained_height = if windef_borrow.expected_max_height != -1 {
                windef_borrow.expected_max_height
            } else {
                old_height
            };

            *new_width = cmp::max(*new_width, cmp::min(old_width, min_retained_width));
            *new_height = cmp::max(*new_height, cmp::min(old_height, min_retained_height));
        }

        if self.is_maximized() {
            self.restore();
        }

        unsafe {
            SetWindowPos(
                self.hwnd.get(),
                0,
                *window_x,
                *window_y,
                *new_width,
                *new_height,
                SWP_NOZORDER
                    | SWP_NOACTIVATE
                    | if self.window_mode == WindowMode::Fullscreen {
                        SWP_NOSENDCHANGING
                    } else {
                        0
                    },
            );
        }

        if windef_borrow.size_will_change_often && virtual_size_changed {
            let vwidth = self.virtual_width.clone();
            let vheight = self.virtual_height.clone();
            self.adjust_window_region(vwidth.get(), vheight.get());
        }
    }
    fn get_fullscreen_info(
        &self,
        x: &mut i32,
        y: &mut i32,
        width: &mut i32,
        height: &mut i32,
    ) -> bool {
        let true_fullscreen = self.window_mode == WindowMode::Fullscreen;

        unsafe {
            let monitor = MonitorFromWindow(
                self.hwnd.get(),
                if true_fullscreen {
                    MONITOR_DEFAULTTOPRIMARY
                } else {
                    MONITOR_DEFAULTTONEAREST
                },
            );
            let mut monitor_info = MONITORINFO::default();
            monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
            GetMonitorInfoW(monitor, &mut monitor_info);

            *x = monitor_info.rcMonitor.left;
            *y = monitor_info.rcMonitor.top;
            *width = monitor_info.rcMonitor.right - *x;
            *height = monitor_info.rcMonitor.bottom - *y;
        }
        true
    }
    fn move_window_to(&self, x: &mut i32, y: &mut i32) {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if windef_borrow.has_os_window_border {
            unsafe {
                let window_style = GetWindowLongW(self.hwnd.get(), GWL_STYLE);
                let window_ex_style = GetWindowLongW(self.hwnd.get(), GWL_EXSTYLE);

                // This adjusts a zero rect to give us the size of the border
                let mut border_rect: RECT = mem::zeroed();
                AdjustWindowRectEx(
                    &mut border_rect,
                    window_style as u32,
                    false,
                    window_ex_style as u32,
                );

                // Border rect size is negative
                *x += border_rect.left;
                *y += border_rect.top;

                SetWindowPos(
                    self.hwnd.get(),
                    0,
                    *x,
                    *y,
                    0,
                    0,
                    SWP_NOACTIVATE | SWP_NOSIZE | SWP_NOZORDER,
                );
            }
        }
    }
    fn bring_to_front(&self, force: bool) {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if self.is_regular_window() {
            unsafe {
                if IsIconic(self.hwnd.get()).0 != 0 {
                    ShowWindow(self.hwnd.get(), SW_RESTORE);
                } else {
                    SetActiveWindow(self.hwnd.get());
                }
            }
        } else {
            let mut hwnd_insert_after = HWND_TOP;
            // By default we activate the window or it isn't actually brought to the front
            let mut flags: u32 = SWP_NOMOVE | SWP_NOSIZE | SWP_NOOWNERZORDER;

            if !force {
                flags |= SWP_NOACTIVATE;
            }

            if windef_borrow.is_topmost_window {
                hwnd_insert_after = HWND_TOPMOST;
            }

            unsafe {
                SetWindowPos(self.hwnd.get(), hwnd_insert_after, 0, 0, 0, 0, flags);
            }
        }
    }
    fn minimize(&self) {
        unsafe { ShowWindow(self.hwnd.get(), SW_MINIMIZE) };
    }
    fn maximize(&self) {
        unsafe { ShowWindow(self.hwnd.get(), SW_MAXIMIZE) };
    }
    fn restore(&self) {
        unsafe { ShowWindow(self.hwnd.get(), SW_RESTORE) };
    }
    fn show(&self) {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if !self.is_visible.get() {
            self.is_visible.set(true);

            // Should the show command include activation?
            // Do not activate windows that do not take input; e.g. tool-tips and cursor decorators
            let mut should_activate = false;
            if windef_borrow.accepts_input {
                should_activate = windef_borrow.activation_policy == WindowActivationPolicy::Always;
                if self.is_first_time_visible.get()
                    && windef_borrow.activation_policy == WindowActivationPolicy::FirstShown
                {
                    should_activate = true;
                }
            }
            let mut show_window_cmd = if should_activate {
                SW_SHOW
            } else {
                SW_SHOWNOACTIVATE
            };
            if self.is_first_time_visible.get() {
                self.is_first_time_visible.set(false);
                if self.initially_minimized.get() {
                    show_window_cmd = if should_activate {
                        SW_MINIMIZE
                    } else {
                        SW_SHOWMINNOACTIVE
                    };
                } else if self.initially_maximized.get() {
                    show_window_cmd = if should_activate {
                        SW_SHOWMAXIMIZED
                    } else {
                        SW_MAXIMIZE
                    };
                }
            }
            unsafe {
                ShowWindow(self.hwnd.get(), show_window_cmd);
            }
            // Turns out SW_SHOWNA doesn't work correctly if the window has never been shown before.  If the window
            // was already maximized, (and hidden) and we're showing it again, SW_SHOWNA would be right.  But it's not right
            // to use SW_SHOWNA when the window has never been shown before!
            //
            // TODO Add in a more complicated path that involves SW_SHOWNA if we hide windows in their maximized/minimized state.
            //::ShowWindow(HWnd, bShouldActivate ? SW_SHOW : SW_SHOWNA);
        }
    }
    fn hide(&self) {
        if self.is_visible.get() {
            self.is_visible.set(false);
            unsafe { ShowWindow(self.hwnd.get(), SW_HIDE) };
        }
    }
    fn get_dpi_scale_factor(&self) -> f32 {
        self.dpi_scale_factor
    }
    fn set_window_mode(&mut self, new_window_mode: WindowMode) {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if new_window_mode != self.window_mode {
            self.window_mode = new_window_mode;

            let true_fullscreen = new_window_mode == WindowMode::Fullscreen;

            let mut window_style = unsafe { GetWindowLongW(self.hwnd.get(), GWL_STYLE) };
            let fullscreen_mode_style = WS_POPUP;
            let mut windowed_mode_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
            if self.is_regular_window() {
                if windef_borrow.supports_maximize {
                    windowed_mode_style |= WS_MAXIMIZEBOX;
                }
                if windef_borrow.supports_minimize {
                    windowed_mode_style |= WS_MINIMIZEBOX;
                }
                if windef_borrow.has_sizing_frame {
                    windowed_mode_style |= WS_THICKFRAME;
                } else {
                    windowed_mode_style |= WS_BORDER;
                }
            } else {
                windowed_mode_style |= WS_POPUP | WS_BORDER;
            }

            if new_window_mode == WindowMode::WindowedFullscreen
                || new_window_mode == WindowMode::Fullscreen
            {
                let is_borderless_game_window = windef_borrow.window_type == WindowType::GameWindow
                    && !windef_borrow.has_os_window_border;
                unsafe {
                    GetWindowPlacement(self.hwnd.get(), &mut self.pre_fullscreen_window_placement);
                }

                // Setup Win32 flags for fullscreen window
                if is_borderless_game_window && !true_fullscreen {
                    window_style &= !fullscreen_mode_style as i32;
                    window_style |= windowed_mode_style as i32;
                } else {
                    window_style &= !windowed_mode_style as i32;
                    window_style |= fullscreen_mode_style as i32;
                }

                unsafe {
                    SetWindowLongW(self.hwnd.get(), GWL_STYLE, window_style);
                    SetWindowPos(
                        self.hwnd.get(),
                        0,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                    );
                }

                if !true_fullscreen {
                    // Ensure the window is restored if we are going for WindowedFullscreen
                    unsafe {
                        ShowWindow(self.hwnd.get(), SW_RESTORE);
                    }
                }

                unsafe {
                    // Get the current window position.
                    let mut client_rect = RECT::default();

                    GetClientRect(self.hwnd.get(), &mut client_rect);

                    // Grab current monitor data for sizing
                    let monitor = MonitorFromWindow(
                        self.hwnd.get(),
                        if true_fullscreen {
                            MONITOR_DEFAULTTOPRIMARY
                        } else {
                            MONITOR_DEFAULTTONEAREST
                        },
                    );
                    let mut monitor_info = MONITORINFO::default();
                    monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
                    GetMonitorInfoW(monitor, &mut monitor_info);

                    // Get the target client width to send to ReshapeWindow.
                    // Preserve the current res if going to true fullscreen and the monitor supports it and allow the calling code
                    // to resize if required.
                    // Else, use the monitor's res for windowed fullscreen.
                    let monitor_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
                    let mut target_client_width = if true_fullscreen {
                        cmp::min(monitor_width, client_rect.right - client_rect.left)
                    } else {
                        monitor_width
                    };

                    let monitor_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;
                    let mut target_client_height = if true_fullscreen {
                        cmp::min(monitor_height, client_rect.bottom - client_rect.top)
                    } else {
                        monitor_height
                    };

                    // Resize and position fullscreen window
                    self.reshape_window(
                        &mut monitor_info.rcMonitor.left,
                        &mut monitor_info.rcMonitor.top,
                        &mut target_client_width,
                        &mut target_client_height,
                    );
                }
            } else {
                // Windowed:

                // Setup Win32 flags for restored window
                window_style &= !fullscreen_mode_style as i32;
                window_style |= windowed_mode_style as i32;
                unsafe {
                    SetWindowLongW(self.hwnd.get(), GWL_STYLE, window_style);
                    SetWindowPos(
                        self.hwnd.get(),
                        0,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                    );

                    SetWindowPlacement(self.hwnd.get(), &self.pre_fullscreen_window_placement);
                }
            }
        }
    }
    fn get_window_mode(&self) -> WindowMode {
        self.window_mode
    }
    fn is_maximized(&self) -> bool {
        let zoomed = unsafe { !!IsZoomed(self.hwnd.get()) };
        zoomed.0 != 0
    }
    fn is_minimized(&self) -> bool {
        let iconic = unsafe { !!IsIconic(self.hwnd.get()) };
        iconic.0 != 0
    }
    fn is_visible(&self) -> bool {
        self.is_visible.get()
    }
    fn get_restored_dimensions(
        &self,
        x: &mut i32,
        y: &mut i32,
        width: &mut i32,
        height: &mut i32,
    ) -> bool {
        let mut window_placement = WINDOWPLACEMENT::default();
        window_placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;

        let res = unsafe { GetWindowPlacement(self.hwnd.get(), &mut window_placement) };
        if res.0 != 0 {
            let restored = window_placement.rcNormalPosition;

            *x = restored.left;
            *y = restored.top;

            *width = restored.right - restored.left;
            *height = restored.bottom - restored.top;
            true
        } else {
            false
        }
    }
    /** Sets focus on the native window */
    fn set_window_focus(&mut self) {
        unsafe {
            if GetFocus() != self.hwnd.get() {
                SetFocus(self.hwnd.get());
            }
        }
    }
    fn set_opacity(&self, opacity: f32) {
        unsafe {
            SetLayeredWindowAttributes(self.hwnd.get(), 0, (opacity * 255.0f32) as u8, LWA_ALPHA);
        }
    }
    fn enable(&self, enable: bool) {
        unsafe { EnableWindow(self.hwnd.get(), if enable { BOOL(1) } else { BOOL(0) }) };
    }
    fn is_point_in_window(&self, x: i32, y: i32) -> bool {
        let mut result = false;
        let region = self.make_window_region_object();
        let res = unsafe { !!PtInRegion(region, x, y).0 == 1 };
        unsafe {
            DeleteObject(region);
        }
        result == res
    }
    fn get_window_border_size(&self) -> u32 {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        if windef_borrow.window_type == WindowType::GameWindow
            && !windef_borrow.has_os_window_border
        {
            // Our borderless game windows actually have a thick border to allow sizing, which we draw over to simulate
            // a borderless window. We return zero here so that the game will correctly behave as if this is truly a
            // borderless window.
            return 0;
        }
        unsafe {
            let mut window_info = WINDOWINFO::default();
            window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
            GetWindowInfo(self.hwnd.get(), &mut window_info);

            window_info.cxWindowBorders
        }
    }
    fn get_os_window_handle(&self) -> *const c_void {
        self.hwnd.get() as *const c_void
    }
    fn get_window_title_bar_size(&self) -> i32 {
        unsafe { GetSystemMetrics(SM_CYCAPTION) }
    }
    fn is_foreground_window(&self) -> bool {
        unsafe { GetForegroundWindow() == self.hwnd.get() }
    }
    fn is_fullscreen_supported(&self) -> bool {
        unsafe { GetSystemMetrics(SM_REMOTESESSION) != 0 }
    }
    fn set_text(&self, text: Vec<u16>) {
        //TODO: genericize the text variable
        unsafe {
            SetWindowTextW(self.hwnd.get(), PWSTR(text.as_mut_ptr()));
        }
    }
    fn get_definition(&self) -> Rc<WindowDefinition> {
        self.window_definitions
    }
    fn adjust_cached_size(&self, size: &mut (i32, i32)) {
        let windef_borrow: &WindowDefinition = Rc::borrow(&self.window_definitions);
        //Unreal Engine 4's check for if the FGenericWindowDefinition is valid is necessary because this is a pointer. Is it necessary in my code?
        if
        /* self.window_definitions.is_valid() && */
        windef_borrow.size_will_change_often {
            *size = (self.virtual_width.get(), self.virtual_height.get());
        } else if self.hwnd.get() != 0 {
            unsafe {
                let mut client_rect = RECT::default();
                GetClientRect(self.hwnd.get(), &mut client_rect);
                size.0 = client_rect.right - client_rect.left;
                size.1 = client_rect.bottom - client_rect.top;
            }
        }
    }
    fn is_manual_manage_dpi_change(&self) -> bool {
        self.handle_manual_dpi_changes
    }
    fn set_manual_manage_dpi_change(&mut self, manual_dpi_changes: bool) {
        self.handle_manual_dpi_changes = manual_dpi_changes;
    }
    fn destroy(&mut self) {
        unsafe {
            if self.ole_reference_count > 0 && IsWindow(self.hwnd.get()).0 != 0 {
                let res = RevokeDragDrop(self.hwnd.get());
                if let Ok(()) = res {}
            }
            DestroyWindow(self.hwnd.get());
        }
    }

    fn is_definition_valid(&self) -> bool {
        todo!()
    }

    fn set_dpi_scale_factor(&mut self, factor: f32) {
        todo!()
    }

    fn draw_attention(&self, parameters: crate::generic::window::WindowDrawAttentionRequestType) {
        todo!()
    }

    fn set_native_window_buttons_visibility(&mut self, visible: bool) {
        todo!()
    }
}

fn create_window(
    ex_style: u32,
    class_name: PWSTR,
    window_name: PWSTR,
    style: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    wnd_parent: HWND,
    menu: HMENU,
    instance: HINSTANCE,
    param: *const c_void,
) -> io::Result<HWND> {
    match unsafe {
        CreateWindowExW(
            ex_style,
            class_name,
            window_name,
            style,
            x,
            y,
            width,
            height,
            wnd_parent,
            menu,
            instance,
            param,
        )
    } {
        v if v == 0 => Err(io::Error::last_os_error()),
        v => Ok(v),
    }
}
