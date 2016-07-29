extern crate dwmapi;
extern crate gdi32;
extern crate user32;
extern crate winapi;

use std::{cmp, io, mem, ptr};
use std::rc::Rc;
use winapi::{
	FALSE, GWL_EXSTYLE, GWL_STYLE, HRGN, HWND, HWND_TOP, HWND_TOPMOST, LWA_ALPHA, MARGINS, MONITORINFO, MONITOR_DEFAULTTOPRIMARY, MONITOR_DEFAULTTONEAREST, RECT, SM_CYCAPTION, SW_HIDE,
	SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWNA, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW, SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER,
	WINDOWINFO, WINDOWPLACEMENT, WS_BORDER, WS_CAPTION, WS_EX_COMPOSITED, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_THICKFRAME
};

pub enum WindowType {
	/** Value indicating that this is a standard, general-purpose window */
    Normal,
	/** Value indicating that this is a window used for a popup menu */
	Menu,
	/** Value indicating that this is a window used for a tooltip */
	ToolTip,
	/** Value indicating that this is a window used for a notification toast */
	Notification,
	/** Value indicating that this is a window used for a cursor decorator */
	CursorDecorator
}

#[derive(PartialEq, Copy, Clone)]
pub enum WindowMode {
	/** The window is in true fullscreen mode */
	Fullscreen,
	/** The window has no border and takes up the entire area of the screen */
	WindowedFullscreen,
	/** The window has a border and may not take up the entire screen area */
	Windowed,
}

#[derive(PartialEq)]
pub enum WindowTransparency {
	/** Value indicating that a window does not support transparency */
	None = 0,

	/** Value indicating that a window supports transparency at the window level (one opacity applies to the entire window) */
	PerWindow = 1,

	/** Value indicating that a window supports per-pixel alpha blended transparency */
    PerPixel = 2,
}

struct WindowSizeLimits {
	min_width: Option<f32>,
	min_height: Option<f32>,
	max_width: Option<f32>,
	max_height: Option<f32>,
}

struct WindowDefinition {
	/** Window type */
	window_type: WindowType,
	
	/** The initially desired horizontal screen position */
	x_desired_position_on_screen: f32,
	/** The initially desired vertical screen position */
	y_desired_position_on_screen: f32,

	/** The initially desired width */
	width_desired_on_screen: f32,
	/** The initially desired height */
	height_desired_on_screen: f32,

	/** the level of transparency supported by this window */
	transparency_support: WindowTransparency,

	/** true if the window is using the os window border instead of a slate created one */
	has_os_window_border: bool,
	/** should this window show up in the taskbar */
	appears_in_taskbar: bool,
	/** true if the window should be on top of all other windows; false otherwise */
	is_topmost_window: bool,
	/** true if the window accepts input; false if the window is non-interactive */
	accepts_input: bool,
	/** true if this window will be activated when it is first shown */
	activate_when_first_shown: bool,
	/** true if this window will be focused when it is first shown */
	focus_when_first_shown: bool,
	/** true if this window displays an enabled close button on the toolbar area */
	has_close_button: bool,
	/** true if this window displays an enabled minimize button on the toolbar area */
	supports_minimize: bool,
	/** true if this window displays an enabled maximize button on the toolbar area */
	supports_maximize: bool,

	/** true if the window is modal (prevents interacting with its parent) */
	is_modal_window: bool,
	/** true if this is a vanilla window, or one being used for some special purpose: e.g. tooltip or menu */
	is_regular_window: bool,
	/** true if this is a user-sized window with a thick edge */
	has_sizing_frame: bool,
	/** true if we expect the size of this window to change often, such as if its animated, or if it recycled for tool-tips. */
	size_will_change_often: bool,
	/** true if the window should preserve its aspect ratio when resized by user */
	should_preserve_aspect_ratio: bool,
	/** The expected maximum width of the window.  May be used for performance optimization when SizeWillChangeOften is set. */
	expected_max_width: i32,
	/** The expected maximum height of the window.  May be used for performance optimization when SizeWillChangeOften is set. */
	expected_max_height: i32,

	/** the title of the window */
	title: String,
	/** opacity of the window (0-1) */
	opacity: f32,
	/** the radius of the corner rounding of the window */
	corner_radius: i32,

	size_limits: WindowSizeLimits,
}

pub struct Window {
	app_window_class: &'static str,
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

impl Window {
	pub fn new() -> Window {
		unsafe {
		    let mut wnd_plcment: WINDOWPLACEMENT = mem::zeroed();
		    let mut wnd_plcment1: WINDOWPLACEMENT = mem::zeroed();
		    wnd_plcment.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
		    wnd_plcment1.length = mem::size_of::<WINDOWPLACEMENT>() as u32;
		    Window {
			    app_window_class: "CormacWindow",
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
	pub fn get_hwnd(&self) -> HWND {
		self.hwnd
	}
	pub fn make_window_region_object(&self) -> HRGN {
        let mut region: HRGN;
        if self.region_width != -1 && self.region_height != -1 {
        	if self.is_maximized() {
        		let window_border_size = self.get_window_border_size();
        		unsafe {
        			region = gdi32::CreateRectRgn(window_border_size as i32, window_border_size as i32, self.region_width - window_border_size as i32, self.region_height - window_border_size as i32);
        		}
        	} else {
        		let use_corner_radius = self.window_definitions.transparency_support != WindowTransparency::PerPixel && self.window_definitions.corner_radius > 0;
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
				let err = io::Error::last_os_error();
            	println!("Warning: {}", err);
			} 
		}
	}
	//Trait method?
	pub fn reshape_window(&mut self, mut new_x: i32, mut new_y: i32, mut new_width: i32, mut new_height: i32) {
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
	//Trait method?
	pub fn get_fullscreen_info(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) -> bool {
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
	//Trait method?
	pub fn move_window_to(&self, mut x: i32, mut y: i32) {
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
	//Trait method?
	pub fn bring_to_front(&self, force: bool) {
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
	//Trait method?
	pub fn minimize(&self) {
		unsafe { user32::ShowWindow(self.hwnd, SW_MINIMIZE) };
	}
	//Trait method?
	pub fn maximize(&self) {
		unsafe { user32::ShowWindow(self.hwnd, SW_MAXIMIZE) };
	}
	//Trait method?
	pub fn restore(&self) {
		unsafe { user32::ShowWindow(self.hwnd, SW_RESTORE) };
	}
	//Trait method?
	pub fn show(&mut self) {
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
	//Trait method?
	pub fn hide(&mut self) {
		if self.is_visible {
			self.is_visible = false;
			unsafe { user32::ShowWindow(self.hwnd, SW_HIDE) };
		}
	}
	//Trait method?
	pub fn set_window_mode(&mut self, new_window_mode: WindowMode) {
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
			    unsafe {
				    user32::GetWindowPlacement(self.hwnd, &mut self.pre_fullscreen_window_placement);
			    }

			    // Setup Win32 flags for fullscreen window
			    window_style &= !windowed_mode_style as i32;
			    window_style |= fullscreen_mode_style as i32;
            
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
	//Trait method?
	pub fn get_window_mode(&self) -> WindowMode {
		self.window_mode
	}
	//Trait method?
	pub fn is_maximized(&self) -> bool {
		let zoomed = unsafe { !!user32::IsZoomed(self.hwnd) };
		zoomed == 1
	}
	//Trait method?
	pub fn is_minimized(&self) -> bool {
		let iconic = unsafe { !!user32::IsIconic(self.hwnd) };
		iconic == 1
	}
	//Trait method?
	pub fn is_visible(&self) -> bool {
		self.is_visible
	}
	//Trait method?
	pub fn get_restored_dimensions(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) -> bool {
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
	pub fn adjust_cached_size(&self, size: &mut (i32, i32)) {
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
	/** Sets focus on the native window */
	//Trait method?
	pub fn set_window_focus(&mut self) {
		unsafe {
	        if user32::GetFocus() != self.hwnd {
		        user32::SetFocus(self.hwnd);
	        }
	    }
    }
    //Trait method?
    pub fn set_opacity(&mut self, opacity: f32) {
        unsafe { user32::SetLayeredWindowAttributes(self.hwnd, 0, (opacity * 255.0f32) as u8, LWA_ALPHA); }
    }
    //Trait method?
    pub fn enable(&self, enable: bool) {
	    unsafe { user32::EnableWindow(self.hwnd, if enable {1} else {0}) };
    }
    pub fn is_enabled(&self) -> bool {
	    let res = unsafe { !!::user32::IsWindowEnabled(self.hwnd) };
	    res == 1
    }
    //Trait method?
    pub fn is_point_in_window(&self, x: i32, y: i32) -> bool {
    	let mut result = false;
    	let region = self.make_window_region_object();
    	let res = unsafe { !!gdi32::PtInRegion(region, x, y) == 1 };
    	unsafe { gdi32::DeleteObject(mem::transmute(region)); }
    	result == res
    }
    //Trait method?
    pub fn get_window_border_size(&self) -> u32 {
	    unsafe {
	    	let mut window_info: WINDOWINFO = mem::uninitialized();
	        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
	        user32::GetWindowInfo(self.hwnd, &mut window_info);

	        window_info.cxWindowBorders
	    }
    }
    //Trait method?
    pub fn get_window_title_bar_size(&self) -> i32 {
	    unsafe { user32::GetSystemMetrics(SM_CYCAPTION) }
    }
    //Trait method?
    pub fn is_foreground_window(&self) -> bool {
    	unsafe { user32::GetForegroundWindow() == self.hwnd }
    }
    pub fn set_text(&self, text: Vec<u16>) {
    	//TODO: genericize the text variable
    	unsafe { user32::SetWindowTextW(self.hwnd, text.as_ptr()); }
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
            		let err = io::Error::last_os_error();
            	    println!("Warning: {}", err);
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
}