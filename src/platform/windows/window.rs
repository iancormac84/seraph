use dwmapi;
use gdi32;
use platform::generic::window::{GenericWindow, WindowMode};
use platform::generic::window_definition::{WindowDefinition, WindowTransparency, WindowType};
use platform::windows::application::WindowsApplication;
use std::{cmp, io, mem, ptr};
use std::ffi::OsStr;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::rc::Rc;
use user32;
use winapi::{
	BOOL, DWMNCRENDERINGPOLICY, DWMNCRP_DISABLED, DWMWA_ALLOW_NCPAINT, DWMWA_NCRENDERING_POLICY, DWORD, FALSE, GWL_EXSTYLE, GWL_STYLE, HINSTANCE, HRESULT, HRGN, HWND, HWND_TOP,
	HWND_TOPMOST, IDataObject, IDropTarget, LWA_ALPHA, MARGINS, MB_ICONEXCLAMATION, MB_OK, MONITORINFO, MONITOR_DEFAULTTOPRIMARY, MONITOR_DEFAULTTONEAREST, POINTL,
	RECT, SM_CYCAPTION, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWNA, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW,
	SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER, S_OK, WINDOWINFO, WINDOWPLACEMENT, WS_BORDER, WS_CAPTION, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_APPWINDOW,
	WS_EX_COMPOSITED, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_THICKFRAME
};

//TODO will these be necessary?
unsafe extern "system" fn drag_enter(This: *mut IDropTarget, pDataObj: *const IDataObject, grfKeyState: DWORD, pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT {
    0
}
unsafe extern "system" fn drag_over(This: *mut IDropTarget, grfKeyState: DWORD, pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT {
    0
}
unsafe extern "system" fn drag_leave(This: *mut IDropTarget) -> HRESULT {
    0
}
unsafe extern "system" fn drop(This: *mut IDropTarget, pDataObj: *const IDataObject, grfKeyState: DWORD, pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT {
    0
}

pub trait DropTarget {
	fn drag_enter(&mut self, pDataObj: *const IDataObject, grfKeyState: DWORD, pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT;
    fn drag_over(&mut self, grfKeyState: DWORD, pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT;
    fn drag_leave(&mut self) -> HRESULT;
    fn drop(&mut self, pDataObj: *const IDataObject, grfKeyState: DWORD, pt: POINTL, pdwEffect: *mut DWORD) -> HRESULT;
}

pub const APP_WINDOW_CLASS: &'static str = "CormacWindow";

//TODO can I make this capable of clone? I want to try this so I don't have to do a clone in the WindowsApplication::find_window_by_hwnd method.
#[derive(PartialEq)]
pub struct WindowsWindow {
	pub app_window_class: &'static str,
	owning_application: *const WindowsApplication,
	hwnd: HWND,
	region_height: i32,
	region_width: i32,
    window_mode: WindowMode,
    ole_reference_count: u32,
    pre_fullscreen_window_placement: WINDOWPLACEMENT,
    pre_parent_minimized_window_placement: WINDOWPLACEMENT,
    virtual_height: i32,
    virtual_width: i32,
    aspect_ratio: f32,
    is_visible: bool,
    window_definitions: Rc<WindowDefinition>,
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
			    owning_application: mem::uninitialized(),
		        hwnd: ptr::null_mut(),
                region_height: mem::uninitialized(),
                region_width: mem::uninitialized(),
                window_mode: WindowMode::Windowed,
                ole_reference_count: 0,
                pre_fullscreen_window_placement: wnd_plcment,
                pre_parent_minimized_window_placement: wnd_plcment1,
                virtual_height: mem::uninitialized(),
                virtual_width: mem::uninitialized(),
                aspect_ratio: 1.0f32,
                is_visible: false,
                window_definitions: mem::uninitialized(),
            }
        }
	}
	pub fn initialize(&mut self, application: *const WindowsApplication, definition: Rc<WindowDefinition>, instance: HINSTANCE, parent: &Rc<WindowsWindow>, show_immediately: bool) {
		self.window_definitions = definition;
        self.owning_application = application;

        let mut window_ex_style: u32 = 0;
        let mut window_style: u32 = 0;
        self.region_width = -1;
        self.region_height = -1;

        let x_initial_rect = self.window_definitions.x_desired_position_on_screen;
	    let y_initial_rect = self.window_definitions.y_desired_position_on_screen;

	    let width_initial = self.window_definitions.width_desired_on_screen;
	    let height_initial = self.window_definitions.height_desired_on_screen;

	    let client_x = x_initial_rect as i32;
	    let client_y = y_initial_rect as i32;
	    let client_width = width_initial as i32;
	    let client_height = height_initial as i32;
	    let mut window_x = client_x;
	    let mut window_y = client_y;
	    let mut window_width = client_width;
	    let mut window_height = client_height;
	    let application_supports_per_pixel_blending = unsafe { (&*application).get_window_transparency_support() == WindowTransparency::PerPixel };

	    if !self.window_definitions.has_os_window_border {
	    	window_ex_style = WS_EX_WINDOWEDGE;

	    	if self.window_definitions.transparency_support == WindowTransparency::PerWindow {
	    		window_ex_style |= WS_EX_LAYERED;
	    	} else if self.window_definitions.transparency_support == WindowTransparency::PerPixel {
	    		if application_supports_per_pixel_blending {
	    			window_ex_style |= WS_EX_COMPOSITED;
	    		}
	    	}
	    	window_style = WS_POPUP | WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
	    	if self.window_definitions.appears_in_taskbar {
	    		window_ex_style |= WS_EX_APPWINDOW;
	    	} else {
	    		window_ex_style |= WS_EX_TOOLWINDOW;
	    	}
	    	if self.window_definitions.is_topmost_window {
	    		// Tool tips are always top most windows
			    window_ex_style |= WS_EX_TOPMOST;
	    	}
	    	if !self.window_definitions.accepts_input {
	    		// Window should never get input
			    window_ex_style |= WS_EX_TRANSPARENT;
	    	}
	    } else {
	    	// OS Window border setup
		    window_ex_style = WS_EX_APPWINDOW;
		    window_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
		    if self.is_regular_window() {
		    	if self.window_definitions.supports_maximize {
				    window_style |= WS_MAXIMIZEBOX;
			    }

			    if self.window_definitions.supports_minimize {
				    window_style |= WS_MINIMIZEBOX;
			    }

			    if self.window_definitions.has_sizing_frame	{
				    window_style |= WS_THICKFRAME;
			    } else {
				    window_style |= WS_BORDER;
			    }
		    } else {
			    window_style |= WS_POPUP | WS_BORDER;
		    }

		    // X,Y, Width, Height defines the top-left pixel of the client area on the screen
		    // This adjusts a zero rect to give us the size of the border
		    let mut border_rect: RECT = unsafe { mem::uninitialized() };
		    unsafe { user32::AdjustWindowRectEx(&mut border_rect, window_style, FALSE, window_ex_style); }

		    // Border rect size is negative - see MoveWindowTo
		    window_x += border_rect.left;
		    window_y += border_rect.top;

		    // Inflate the window size by the OS border
		    window_width += border_rect.right - border_rect.left;
		    window_height += border_rect.bottom - border_rect.top;
	    }

	    //TODO: parent window may be null, but I'm using Rc to hold parent window, which I think implies that parent window can't be null. Fix.
	    self.hwnd = unsafe { user32::CreateWindowExW(
		    window_ex_style,
		    OsStr::new(self.app_window_class).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr(),
		    OsStr::new(&self.window_definitions.title[..]).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr(),
		    window_style,
		    window_x, window_y, 
		    window_width, window_height,
		    parent.hwnd,
		    ptr::null_mut(), instance, ptr::null_mut()) };

	    self.virtual_width = client_width;
	    self.virtual_height = client_height;

	    // We call reshape window here because we didn't take into account the non-client area
	    // in the initial creation of the window. Slate should only pass client area dimensions.
	    // Reshape window may resize the window if the non-client area is encroaching on our
	    // desired client area space.
	    self.reshape_window(client_x, client_y, client_width, client_height);

	    if self.hwnd.is_null() {
	    	unsafe {
	    		user32::MessageBoxW(ptr::null_mut(),
	    		    OsStr::new(&"Window Creation Failed!").encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr(),
	    		    OsStr::new(&"Error!").encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr(),
	    		    MB_ICONEXCLAMATION | MB_OK
	    	    );
	    	}
	    	return;
	    }
	    if self.window_definitions.transparency_support == WindowTransparency::PerWindow {
	    	let opacity = self.window_definitions.opacity;
	    	self.set_opacity(opacity);
	    }
	    if !self.window_definitions.has_os_window_border {
	    	let rendering_policy: DWMNCRENDERINGPOLICY = DWMNCRP_DISABLED;

	    	unsafe {
	    		if dwmapi::DwmSetWindowAttribute(self.hwnd, mem::transmute(DWMWA_NCRENDERING_POLICY), mem::transmute(&rendering_policy), mem::size_of::<DWMNCRENDERINGPOLICY>() as u32) != S_OK {
	    			println!("Warning: {}", io::Error::last_os_error());
	    		}
	    		let enable_nc_paint = FALSE;
	    		if dwmapi::DwmSetWindowAttribute(self.hwnd, mem::transmute(DWMWA_ALLOW_NCPAINT), mem::transmute(&enable_nc_paint), mem::size_of::<BOOL>() as u32) != S_OK {
                    println!("Warning: {}", io::Error::last_os_error());
	    		}
	    		if application_supports_per_pixel_blending && self.window_definitions.transparency_support == WindowTransparency::PerPixel {
	    			let mut margins = MARGINS {cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1};
	    			if dwmapi::DwmExtendFrameIntoClientArea(self.hwnd, &margins) != 0 {
                        println!("Warning: {}", io::Error::last_os_error());
	    			}
	    		}
	    	}
	    }

	    if self.is_regular_window() && !self.window_definitions.has_os_window_border {
	    	window_style |= WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU;

	    	if self.window_definitions.supports_maximize {
			    window_style |= WS_MAXIMIZEBOX;
		    }
		    if self.window_definitions.supports_minimize {
		    	window_style |= WS_MINIMIZEBOX;
		    }
		    if self.window_definitions.has_sizing_frame {
			    window_style |= WS_THICKFRAME;
		    }

		    unsafe {
		    	if user32::SetWindowLongW(self.hwnd, GWL_STYLE, window_style as i32) == 0 {
		    		println!("Warning: {}", io::Error::last_os_error());
		    	}
		    	user32::SetWindowPos(self.hwnd, ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
		    	self.adjust_window_region(client_width, client_height);
		    }
	    }
	}
	pub fn get_hwnd(&self) -> HWND {
		self.hwnd
	}
	pub fn make_window_region_object(&self) -> HRGN {
        let mut region: HRGN;
        if self.region_width != -1 && self.region_height != -1 {
        	if self.is_maximized() {
        		if self.get_definition().window_type == WindowType::GameWindow && !self.get_definition().has_os_window_border {
        			// Windows caches the cxWindowBorders size at window creation. Even if borders are removed or resized Windows will continue to use this value when evaluating regions
				    // and sizing windows. When maximized this means that our window position will be offset from the screen origin by (-cxWindowBorders,-cxWindowBorders). We want to
				    // display only the region within the maximized screen area, so offset our upper left and lower right by cxWindowBorders.
				    unsafe {
				        let mut window_info: WINDOWINFO = mem::zeroed();
				        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
				        user32::GetWindowInfo(self.hwnd, &mut window_info);

				        region = gdi32::CreateRectRgn(window_info.cxWindowBorders as i32, window_info.cxWindowBorders as i32, self.region_width + window_info.cxWindowBorders as i32, self.region_height + window_info.cxWindowBorders as i32);
				    }
        		} else {
                    let window_border_size = self.get_window_border_size();
        		    unsafe {
        			    region = gdi32::CreateRectRgn(window_border_size as i32, window_border_size as i32, self.region_width - window_border_size as i32, self.region_height - window_border_size as i32);
        		    }
        		}
        	} else {
        		let use_corner_radius = self.window_mode == WindowMode::Windowed && self.window_definitions.transparency_support != WindowTransparency::PerPixel && self.window_definitions.corner_radius > 0;
        		if use_corner_radius {
        			unsafe {
        				region = gdi32::CreateRoundRectRgn(0, 0, self.region_width + 1, self.region_height + 1, self.window_definitions.corner_radius, self.window_definitions.corner_radius);
        			}
        		} else {
        			unsafe {
        				region = gdi32::CreateRectRgn(0, 0, self.region_width, self.region_height);
        			}
        		}
        	}
        } else {
            let mut rect_wnd: RECT = unsafe { mem::uninitialized() };
            unsafe { 
            	user32::GetWindowRect(self.hwnd, &mut rect_wnd);
            	region = gdi32::CreateRectRgn(0, 0, rect_wnd.right - rect_wnd.left, rect_wnd.bottom - rect_wnd.top);
            }
        }
        region
	}
	pub fn adjust_window_region(&mut self, width: i32, height: i32) {
		self.region_width = width;
		self.region_height = height;
		let region = self.make_window_region_object();
		unsafe {
			if user32::SetWindowRgn(self.hwnd, region, FALSE) == 0 {
            	println!("Warning: {}", io::Error::last_os_error());
			} 
		}
	}
	pub fn on_parent_window_minimized(&mut self) {
	    // This function is called from SW_PARENTCLOSING, because there's a bug in Win32 that causes the equivalent SW_PARENTOPENING
	    // message to restore in an incorrect state (eg, it will lose the maximized status of the window)
	    // To work around this, we cache our window placement here so that we can restore it later (see OnParentWindowRestored)
	    unsafe { user32::GetWindowPlacement(self.hwnd, &mut self.pre_parent_minimized_window_placement); }
	}
	pub fn on_parent_window_restored(&self) {
	    // This function is called from SW_PARENTOPENING so that we can restore the window placement that was cached in OnParentWindowMinimized
	    unsafe { user32::SetWindowPlacement(self.hwnd, &self.pre_parent_minimized_window_placement); }
	}
	pub fn is_enabled(&self) -> bool {
	    let res = unsafe { !!::user32::IsWindowEnabled(self.hwnd) };
	    res == 1
    }
    pub fn is_regular_window(&self) -> bool {
        self.window_definitions.is_regular_window
	}
	pub fn on_transparency_support_changed(&mut self, new_transparency: WindowTransparency) {
		if self.window_definitions.transparency_support == WindowTransparency::PerPixel {
            let style = unsafe { user32::GetWindowLongW(self.hwnd, GWL_EXSTYLE) };

            if new_transparency == WindowTransparency::PerPixel {
            	unsafe {
            		user32::SetWindowLongW(self.hwnd, GWL_EXSTYLE, style | WS_EX_COMPOSITED as i32)
            	};
            	let margins: MARGINS = MARGINS {cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1};

            	let res = unsafe { dwmapi::DwmExtendFrameIntoClientArea(self.hwnd, &margins) };
            	if res != 0 {
            	    println!("Warning: {}", io::Error::last_os_error());
            	}
            } else {
            	unsafe {
            		user32::SetWindowLongW(self.hwnd, GWL_EXSTYLE, style & !WS_EX_COMPOSITED as i32)
            	};
            }

            // Must call SWP_FRAMECHANGED when updating the style attribute of a window in order to update internal caches (according to MSDN)
            unsafe {
		        user32::SetWindowPos(self.hwnd, ptr::null_mut(), 0, 0, 0, 0, SWP_FRAMECHANGED | SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOOWNERZORDER | SWP_NOREDRAW | SWP_NOSIZE | SWP_NOSENDCHANGING | SWP_NOZORDER)
            };
		}
	}
	pub fn get_aspect_ratio(&self) -> f32 { self.aspect_ratio }
	/*pub unsafe extern "system" fn query_interface(iid: REFIID, ppvObject: *mut *mut c_void) {
		//let mut mut_self: *mut c_void = ptr::null_mut();
		//let mut_self = &mut *(mut_self as *mut Self);
		let idt = IID_IDropTarget; 
		let iunkwn = IID_IUnknown;
        if ole32::IsEqualGUID(&iunkwn, iid) != 0 || ole32::IsEqualGUID(&idt, iid) != 0 {
            Self::AddRef();
        }
	}
	pub unsafe extern "system" fn AddRef() -> ULONG {
        let mut mut_self: *mut c_void = ptr::null_mut();
		let mut_self = &mut *(mut_self as *mut Self);
		mut_self.ole_reference_count += 1;
		mut_self.ole_reference_count
	}
	let dtvbl: IDropTargetVtbl = IDropTargetVtbl { DragEnter: drag_enter, DragOver: drag_over, DragLeave: drag_leave, Drop: drop };*/
}

impl GenericWindow for WindowsWindow {
	fn reshape_window(&mut self, mut new_x: i32, mut new_y: i32, mut new_width: i32, mut new_height: i32) {
		let mut window_info: WINDOWINFO = unsafe { mem::uninitialized() };
		unsafe {
			window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
		    user32::GetWindowInfo(self.hwnd, &mut window_info);
		}

		self.aspect_ratio = new_width as f32 / new_height as f32;

		if self.window_definitions.has_os_window_border {
			let mut border_rect: RECT = unsafe { mem::zeroed() };
			unsafe {
				user32::AdjustWindowRectEx(&mut border_rect, window_info.dwStyle, FALSE, window_info.dwExStyle)
			};

			new_x += border_rect.left;
			new_y += border_rect.top;

			new_width += border_rect.right - border_rect.left;
		    new_height += border_rect.bottom - border_rect.top;
		}
		let window_x = new_x;
		let window_y = new_y;

		let virtual_size_changed = new_width != self.virtual_width || new_height != self.virtual_height;
		self.virtual_width = new_width;
		self.virtual_height = new_height;
		
		if self.window_definitions.size_will_change_often {
			let old_window_rect = window_info.rcWindow;
			let old_width = old_window_rect.right - old_window_rect.left;
		    let old_height = old_window_rect.bottom - old_window_rect.top;

		    let min_retained_width = if self.window_definitions.expected_max_width != -1 { self.window_definitions.expected_max_width } else { old_width };
		    let min_retained_height = if self.window_definitions.expected_max_height != -1 { self.window_definitions.expected_max_height } else { old_height };

		    new_width = cmp::max(new_width, cmp::min(old_width, min_retained_width));
		    new_height = cmp::max(new_height, cmp::min(old_height, min_retained_height));
		}

		if self.is_maximized() {
			self.restore();
		}

		unsafe {
			user32::SetWindowPos(self.hwnd, ptr::null_mut(), window_x, window_y, new_width, new_height, SWP_NOZORDER | SWP_NOACTIVATE | if self.window_mode == WindowMode::Fullscreen { SWP_NOSENDCHANGING } else { 0 });
		}

		if self.window_definitions.size_will_change_often && virtual_size_changed {
			let vwidth = self.virtual_width.clone();
			let vheight = self.virtual_height.clone();
		    self.adjust_window_region(vwidth, vheight);
	    }
	}
	fn get_fullscreen_info(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) -> bool {
		let true_fullscreen = self.window_mode == WindowMode::Fullscreen;

		unsafe {
			let monitor = user32::MonitorFromWindow(self.hwnd, if true_fullscreen { MONITOR_DEFAULTTOPRIMARY } else { MONITOR_DEFAULTTONEAREST });
			let mut monitor_info: MONITORINFO = mem::uninitialized();
			monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
			user32::GetMonitorInfoW(monitor, &mut monitor_info);

			*x = monitor_info.rcMonitor.left;
	        *y = monitor_info.rcMonitor.top;
	        *width = monitor_info.rcMonitor.right - *x;
	        *height = monitor_info.rcMonitor.bottom - *y;
		}
		true
	}
	fn move_window_to(&self, mut x: i32, mut y: i32) {
		if self.window_definitions.has_os_window_border {
			unsafe {
				let window_style = user32::GetWindowLongW(self.hwnd, GWL_STYLE);
		        let window_ex_style = user32::GetWindowLongW(self.hwnd, GWL_EXSTYLE);

		        // This adjusts a zero rect to give us the size of the border
		        let mut border_rect: RECT = mem::zeroed();
		        user32::AdjustWindowRectEx(&mut border_rect, window_style as u32, FALSE, window_ex_style as u32);

		        // Border rect size is negative
		        x += border_rect.left;
		        y += border_rect.top;

		        user32::SetWindowPos(self.hwnd, ptr::null_mut(), x, y, 0, 0, SWP_NOACTIVATE | SWP_NOSIZE | SWP_NOZORDER);
			}
		}
	}
	fn bring_to_front(&self, force: bool) {
		if self.is_regular_window() {
			unsafe {
				if user32::IsIconic(self.hwnd) != 0 {
					user32::ShowWindow(self.hwnd, SW_RESTORE);
				} else {
					user32::SetActiveWindow(self.hwnd);
				}
			}
		} else {
			let mut hwnd_insert_after = HWND_TOP;
            // By default we activate the window or it isn't actually brought to the front 
		    let mut flags: u32 = SWP_NOMOVE | SWP_NOSIZE | SWP_NOOWNERZORDER;

		    if !force {
		    	flags |= SWP_NOACTIVATE;
		    }

		    if self.window_definitions.is_topmost_window {
                hwnd_insert_after = HWND_TOPMOST;
		    }
            
            unsafe {
		        user32::SetWindowPos(self.hwnd, hwnd_insert_after, 0, 0, 0, 0, flags);
            }
		}
	}
	fn minimize(&self) {
		unsafe { user32::ShowWindow(self.hwnd, SW_MINIMIZE) };
	}
	fn maximize(&self) {
		unsafe { user32::ShowWindow(self.hwnd, SW_MAXIMIZE) };
	}
	fn restore(&self) {
		unsafe { user32::ShowWindow(self.hwnd, SW_RESTORE) };
	}
	fn show(&mut self) {
		if !self.is_visible {
			self.is_visible = true;

			// Do not activate windows that do not take input; e.g. tool-tips and cursor decorators
		    // Also dont activate if a window wants to appear but not activate itself
		    let should_activate = self.window_definitions.accepts_input && self.window_definitions.activate_when_first_shown;
		    unsafe {
		        user32::ShowWindow(self.hwnd, if should_activate { SW_SHOW } else { SW_SHOWNA });
		    }
		}
	}
	fn hide(&mut self) {
		if self.is_visible {
			self.is_visible = false;
			unsafe { user32::ShowWindow(self.hwnd, SW_HIDE) };
		}
	}
    fn set_window_mode(&mut self, new_window_mode: WindowMode) {
		if new_window_mode != self.window_mode {
			self.window_mode = new_window_mode;

			let true_fullscreen = new_window_mode == WindowMode::Fullscreen;

			let mut window_style = unsafe {
				user32::GetWindowLongW(self.hwnd, GWL_STYLE)
			};
			let fullscreen_mode_style = WS_POPUP;
			let mut windowed_mode_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
			if self.is_regular_window() {
				if self.window_definitions.supports_maximize {
					windowed_mode_style |= WS_MAXIMIZEBOX;
				}
				if self.window_definitions.supports_minimize {
					windowed_mode_style |= WS_MINIMIZEBOX;
				}
				if self.window_definitions.has_sizing_frame {
					windowed_mode_style |= WS_THICKFRAME;
				} else {
					windowed_mode_style |= WS_BORDER;
				}
			} else {
				windowed_mode_style |= WS_POPUP | WS_BORDER;
			}

			if new_window_mode == WindowMode::WindowedFullscreen || new_window_mode == WindowMode::Fullscreen {
				let is_borderless_game_window = self.window_definitions.window_type == WindowType::GameWindow && !self.window_definitions.has_os_window_border;
			    unsafe {
				    user32::GetWindowPlacement(self.hwnd, &mut self.pre_fullscreen_window_placement);
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
			        user32::SetWindowLongW(self.hwnd, GWL_STYLE, window_style);
			        user32::SetWindowPos(self.hwnd, ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
			    }

			    if !true_fullscreen {
				    // Ensure the window is restored if we are going for WindowedFullscreen
				    unsafe { user32::ShowWindow(self.hwnd, SW_RESTORE); }
			    }

			    unsafe {
			    	// Get the current window position.
			        let mut client_rect: RECT = mem::uninitialized();

			        user32::GetClientRect(self.hwnd, &mut client_rect);

			        // Grab current monitor data for sizing
			        let monitor = user32::MonitorFromWindow(self.hwnd, if true_fullscreen { MONITOR_DEFAULTTOPRIMARY } else { MONITOR_DEFAULTTONEAREST });
			        let mut monitor_info: MONITORINFO = mem::uninitialized();
			        monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
			        user32::GetMonitorInfoW(monitor, &mut monitor_info);

			        // Get the target client width to send to ReshapeWindow.
			        // Preserve the current res if going to true fullscreen and the monitor supports it and allow the calling code
			        // to resize if required.
			        // Else, use the monitor's res for windowed fullscreen.
			        let monitor_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
			        let target_client_width = if true_fullscreen { cmp::min(monitor_width, client_rect.right - client_rect.left) } else { monitor_width };

			        let monitor_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;
			        let target_client_height = if true_fullscreen { cmp::min(monitor_height, client_rect.bottom - client_rect.top) } else { monitor_height };

     			    // Resize and position fullscreen window
			        self.reshape_window(
				        monitor_info.rcMonitor.left,
				        monitor_info.rcMonitor.top,
				        target_client_width,
				        target_client_height
				    );
			    }
		    } else {
			    // Windowed:

			    // Setup Win32 flags for restored window
			    window_style &= !fullscreen_mode_style as i32;
			    window_style |= windowed_mode_style as i32;
			    unsafe {
			        user32::SetWindowLongW(self.hwnd, GWL_STYLE, window_style);
			        user32::SetWindowPos(self.hwnd, ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);

			        user32::SetWindowPlacement(self.hwnd, &self.pre_fullscreen_window_placement);
			    }
		    }
		}
	}
	fn get_window_mode(&self) -> WindowMode {
		self.window_mode
	}
	fn is_maximized(&self) -> bool {
		let zoomed = unsafe { !!user32::IsZoomed(self.hwnd) };
		zoomed == 1
	}
    fn is_minimized(&self) -> bool {
		let iconic = unsafe { !!user32::IsIconic(self.hwnd) };
		iconic == 1
	}
	fn is_visible(&self) -> bool {
		self.is_visible
	}
	fn get_restored_dimensions(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) -> bool {
		let mut window_placement: WINDOWPLACEMENT = unsafe { mem::uninitialized() };
		window_placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;

		let res = unsafe { user32::GetWindowPlacement(self.hwnd, &mut window_placement) };
		if res != 0 {
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
	        if user32::GetFocus() != self.hwnd {
		        user32::SetFocus(self.hwnd);
	        }
	    }
    }
    fn set_opacity(&mut self, opacity: f32) {
        unsafe { user32::SetLayeredWindowAttributes(self.hwnd, 0, (opacity * 255.0f32) as u8, LWA_ALPHA); }
    }
    fn enable(&self, enable: bool) {
	    unsafe { user32::EnableWindow(self.hwnd, if enable {1} else {0}) };
    }
    fn is_point_in_window(&self, x: i32, y: i32) -> bool {
    	let mut result = false;
    	let region = self.make_window_region_object();
    	let res = unsafe { !!gdi32::PtInRegion(region, x, y) == 1 };
    	unsafe { gdi32::DeleteObject(mem::transmute(region)); }
    	result == res
    }
    fn get_window_border_size(&self) -> u32 {
    	if self.get_definition().window_type == WindowType::GameWindow && !self.get_definition().has_os_window_border {
		    // Our borderless game windows actually have a thick border to allow sizing, which we draw over to simulate
		    // a borderless window. We return zero here so that the game will correctly behave as if this is truly a
		    // borderless window.
		    return 0;
	    }
	    unsafe {
	    	let mut window_info: WINDOWINFO = mem::uninitialized();
	        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
	        user32::GetWindowInfo(self.hwnd, &mut window_info);

	        window_info.cxWindowBorders
	    }
    }
    fn get_os_window_handle(&self) -> *const c_void {
    	self.hwnd as *const c_void
    }
    fn get_window_title_bar_size(&self) -> i32 {
	    unsafe { user32::GetSystemMetrics(SM_CYCAPTION) }
    }
    fn is_foreground_window(&self) -> bool {
    	unsafe { user32::GetForegroundWindow() == self.hwnd }
    }
    fn set_text(&self, text: Vec<u16>) {
    	//TODO: genericize the text variable
    	unsafe { user32::SetWindowTextW(self.hwnd, text.as_ptr()); }
    }
    fn get_definition<'a>(&'a self) -> &'a WindowDefinition {
    	&self.window_definitions
    }
    fn adjust_cached_size(&self, size: &mut (i32, i32)) {
		//Unreal Engine 4's check for if the FGenericWindowDefinition is valid is necessary because this is a pointer. Is it necessary in my code?
		if /* self.window_definitions.is_valid() && */ self.window_definitions.size_will_change_often {
			*size = (self.virtual_width, self.virtual_height);
		} else if !self.hwnd.is_null() {
			unsafe {
				let mut client_rect: RECT = mem::uninitialized();
		        user32::GetClientRect(self.hwnd, &mut client_rect);
		        size.0 = client_rect.right - client_rect.left;
		        size.1 = client_rect.bottom - client_rect.top;
		    }
		}
	}
}