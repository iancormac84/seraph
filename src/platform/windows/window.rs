use dwmapi;
use gdi32;
use platform::generic::window::{GenericWindow, WindowMode};
use platform::generic::window_definition::{WindowDefinition, WindowTransparency, WindowType};
use platform::windows::application::WindowsApplication;
use platform::windows::application::WINDOWS_APPLICATION;
use platform::windows::utils::ToWide;
use std::cell::{Cell, RefCell};
use std::{cmp, io, mem, panic, ptr};
use std::error::Error;
use std::ops::DerefMut;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::{Arc, Weak};
use super::{DWMNCRP_DISABLED, DWMWA_ALLOW_NCPAINT, DWMWA_NCRENDERING_POLICY, MARGINS, WINDOWINFO};
use user32;
use winapi::{
	BOOL, DWORD, FALSE, GWL_EXSTYLE, GWL_STYLE, HINSTANCE, HMENU, HRESULT, HRGN, HWND, HWND_TOP, HWND_TOPMOST, INT, LPVOID, LPCWSTR, LWA_ALPHA, MB_ICONEXCLAMATION, MB_OK, MONITORINFO, MONITOR_DEFAULTTOPRIMARY, MONITOR_DEFAULTTONEAREST, POINTL,
	RECT, SM_CYCAPTION, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, SW_SHOWNA, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW,
	SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER, S_OK, WINDOWPLACEMENT, WS_BORDER, WS_CAPTION, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_APPWINDOW,
	WS_EX_COMPOSITED, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_THICKFRAME
};

pub const APP_WINDOW_CLASS: &'static str = "CormacWindow";

//TODO can I make this capable of clone? I want to try this so I don't have to do a clone in the WindowsApplication::find_window_by_hwnd method.
#[derive(Debug)]
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
    window_definitions: RefCell<Rc<RefCell<WindowDefinition>>>,
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
				hwnd: Cell::new(ptr::null_mut()),
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
				window_definitions: RefCell::new(Rc::new(RefCell::new(WindowDefinition::default()))),
            }
        }
	}
	pub fn make() -> Rc<RefCell<WindowsWindow>> {
		println!("WindowsWindow::make");
		Rc::new(RefCell::new(WindowsWindow::new()))
	}
	pub fn initialize(&self, definition: &Rc<RefCell<WindowDefinition>>, instance: HINSTANCE, parent: Option<Rc<WindowsWindow>>, show_immediately: bool) {
		println!("Just reach in initialize");
		
		self.window_definitions.borrow_mut().clone_from(definition);
		
        let mut window_ex_style: u32 = 0;
        let mut window_style: u32 = 0;

		let windef_borrow = self.window_definitions.borrow();

		let x_initial_rect = windef_borrow.borrow().x_desired_position_on_screen;
	    let y_initial_rect = windef_borrow.borrow().y_desired_position_on_screen;

	    let width_initial = windef_borrow.borrow().width_desired_on_screen;
	    let height_initial = windef_borrow.borrow().height_desired_on_screen;

	    let mut client_x = x_initial_rect as i32;
	    let mut client_y = y_initial_rect as i32;
	    let mut client_width = width_initial as i32;
	    let mut client_height = height_initial as i32;
	    let mut window_x = client_x;
	    let mut window_y = client_y;
	    let mut window_width = client_width;
	    let mut window_height = client_height;
		println!("about to borrow application");
		let application_supports_per_pixel_blending = unsafe { WINDOWS_APPLICATION.unwrap().get_window_transparency_support() } == WindowTransparency::PerPixel;
        println!("Borrow panic potential suspect innocent");

	    if !windef_borrow.borrow().has_os_window_border {
	    	window_ex_style = WS_EX_WINDOWEDGE;

	    	if windef_borrow.borrow().transparency_support == WindowTransparency::PerWindow {
	    		window_ex_style |= WS_EX_LAYERED;
	    	} else if windef_borrow.borrow().transparency_support == WindowTransparency::PerPixel {
	    		if application_supports_per_pixel_blending {
	    			window_ex_style |= WS_EX_COMPOSITED;
	    		}
	    	}
	    	window_style = WS_POPUP | WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
	    	if windef_borrow.borrow().appears_in_taskbar {
	    		window_ex_style |= WS_EX_APPWINDOW;
	    	} else {
	    		window_ex_style |= WS_EX_TOOLWINDOW;
	    	}
	    	if windef_borrow.borrow().is_topmost_window {
	    		// Tool tips are always top most windows
			    window_ex_style |= WS_EX_TOPMOST;
	    	}
	    	if !windef_borrow.borrow().accepts_input {
	    		// Window should never get input
			    window_ex_style |= WS_EX_TRANSPARENT;
	    	}
	    } else {
	    	// OS Window border setup
		    window_ex_style = WS_EX_APPWINDOW;
		    window_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
		    if windef_borrow.borrow().is_regular_window {
		    	if windef_borrow.borrow().supports_maximize {
				    window_style |= WS_MAXIMIZEBOX;
			    }

			    if windef_borrow.borrow().supports_minimize {
				    window_style |= WS_MINIMIZEBOX;
			    }

			    if windef_borrow.borrow().has_sizing_frame	{
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
	    println!("WindowsWindow self is {:p}", self);
		//let len = Weak::upgrade(&self.owning_application).unwrap().borrow().windows.len();
		//println!("The vec len is {}", len);
		println!("self debug is {:#?}", self);
		println!("self.hwnd is {:p}", self.hwnd.get());
	    println!("self.owning_application is {:p}", &Weak::upgrade(&self.owning_application));
		self.hwnd.set(match create_window(
		    window_ex_style, APP_WINDOW_CLASS.to_wide_null().as_ptr(),
		    (&windef_borrow.borrow().title[..]).to_wide_null().as_ptr(),
		    window_style, window_x, window_y, window_width, window_height,
		    if parent.is_some() { parent.unwrap().hwnd.get() } else { ptr::null_mut() },
		    ptr::null_mut(), instance, ptr::null_mut()) {
                Ok(hwnd) => hwnd,
                Err(err) => {
                  	wui_abort!(
                        "\
                            Window Creation Failed!: {}\r\n\
                        ",
                        err)
                }
		});
		println!("self.hwnd is now {:p}", self.hwnd.get());
	    
        println!("CreateWindowExW called");
		self.virtual_width.set(client_width);
		self.virtual_height.set(client_height);

	    // We call reshape window here because we didn't take into account the non-client area
	    // in the initial creation of the window. Slate should only pass client area dimensions.
	    // Reshape window may resize the window if the non-client area is encroaching on our
	    // desired client area space.
	    self.reshape_window(&mut client_x, &mut client_y, &mut client_width, &mut client_height);

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
	    if windef_borrow.borrow().transparency_support == WindowTransparency::PerWindow {
	    	let opacity = windef_borrow.borrow().opacity;
	    	self.set_opacity(opacity);
	    }
	    if !windef_borrow.borrow().has_os_window_border {
	    	let rendering_policy = DWMNCRP_DISABLED;

	    	unsafe {
	    		if super::DwmSetWindowAttribute(self.hwnd.get(), mem::transmute(DWMWA_NCRENDERING_POLICY), mem::transmute(&rendering_policy), mem::size_of::<DWORD>() as u32) != S_OK {
	    			println!("Warning: {}", io::Error::last_os_error());
	    		}
	    		let enable_nc_paint = FALSE;
	    		if super::DwmSetWindowAttribute(self.hwnd.get(), mem::transmute(DWMWA_ALLOW_NCPAINT), mem::transmute(&enable_nc_paint), mem::size_of::<BOOL>() as u32) != S_OK {
                    println!("Warning: {}", io::Error::last_os_error());
	    		}
	    		if application_supports_per_pixel_blending && windef_borrow.borrow().transparency_support == WindowTransparency::PerPixel {
	    			let mut margins = MARGINS {cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1};
	    			if super::DwmExtendFrameIntoClientArea(self.hwnd.get(), &margins) != 0 {
                        println!("Warning: {}", io::Error::last_os_error());
	    			}
	    		}
	    	}
	    }

	    if windef_borrow.borrow().is_regular_window && !windef_borrow.borrow().has_os_window_border {
	    	window_style |= WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU;

	    	if windef_borrow.borrow().supports_maximize {
			    window_style |= WS_MAXIMIZEBOX;
		    }
		    if windef_borrow.borrow().supports_minimize {
		    	window_style |= WS_MINIMIZEBOX;
		    }
		    if windef_borrow.borrow().has_sizing_frame {
			    window_style |= WS_THICKFRAME;
		    }

		    unsafe {
		    	if user32::SetWindowLongW(self.hwnd.get(), GWL_STYLE, window_style as i32) == 0 {
		    		println!("Warning: {}", io::Error::last_os_error());
		    	}
		    	user32::SetWindowPos(self.hwnd.get(), ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
		    	self.adjust_window_region(client_width, client_height); //Adjusts region{width, height}
		    }
	    }
	}
	pub fn get_hwnd(&self) -> HWND {
		self.hwnd.get()
	}
	pub fn make_window_region_object(&self) -> HRGN {
        let mut region: HRGN;
		let windef_borrow = self.window_definitions.borrow();
        if self.region_width.get() != -1 && self.region_height.get() != -1 {
        	if self.is_maximized() {
        		if windef_borrow.borrow().window_type == WindowType::GameWindow && !windef_borrow.borrow().has_os_window_border {
        			// Windows caches the cxWindowBorders size at window creation. Even if borders are removed or resized Windows will continue to use this value when evaluating regions
				    // and sizing windows. When maximized this means that our window position will be offset from the screen origin by (-cxWindowBorders,-cxWindowBorders). We want to
				    // display only the region within the maximized screen area, so offset our upper left and lower right by cxWindowBorders.
				    unsafe {
				        let mut window_info: WINDOWINFO = mem::zeroed();
				        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
				        super::GetWindowInfo(self.hwnd.get(), &mut window_info);

				        region = gdi32::CreateRectRgn(window_info.cxWindowBorders as i32, window_info.cxWindowBorders as i32, self.region_width.get() + window_info.cxWindowBorders as i32, self.region_height.get() + window_info.cxWindowBorders as i32);
				    }
        		} else {
                    let window_border_size = self.get_window_border_size();
        		    unsafe {
        			    region = gdi32::CreateRectRgn(window_border_size as i32, window_border_size as i32, self.region_width.get() - window_border_size as i32, self.region_height.get() - window_border_size as i32);
        		    }
        		}
        	} else {
        		let use_corner_radius = self.window_mode == WindowMode::Windowed && windef_borrow.borrow().transparency_support != WindowTransparency::PerPixel && windef_borrow.borrow().corner_radius > 0;
        		if use_corner_radius {
        			unsafe {
        				region = gdi32::CreateRoundRectRgn(0, 0, self.region_width.get() + 1, self.region_height.get() + 1, windef_borrow.borrow().corner_radius, windef_borrow.borrow().corner_radius);
        			}
        		} else {
        			unsafe {
						region = gdi32::CreateRectRgn(0, 0, self.region_width.get(), self.region_height.get());
        			}
        		}
        	}
        } else {
            let mut rect_wnd: RECT = unsafe { mem::uninitialized() };
            unsafe { 
            	user32::GetWindowRect(self.hwnd.get(), &mut rect_wnd);
            	region = gdi32::CreateRectRgn(0, 0, rect_wnd.right - rect_wnd.left, rect_wnd.bottom - rect_wnd.top);
            }
        }
        region
	}
	pub fn adjust_window_region(&self, width: i32, height: i32) {
		self.region_width.set(width);
		self.region_height.set(height);
		let region = self.make_window_region_object();
		unsafe {
			if user32::SetWindowRgn(self.hwnd.get(), region, FALSE) == 0 {
            	println!("Warning: {}", io::Error::last_os_error());
			} 
		}
	}
	pub fn on_parent_window_minimized(&mut self) {
	    // This function is called from SW_PARENTCLOSING, because there's a bug in Win32 that causes the equivalent SW_PARENTOPENING
	    // message to restore in an incorrect state (eg, it will lose the maximized status of the window)
	    // To work around this, we cache our window placement here so that we can restore it later (see OnParentWindowRestored)
	    unsafe { user32::GetWindowPlacement(self.hwnd.get(), &mut self.pre_parent_minimized_window_placement); }
	}
	pub fn on_parent_window_restored(&self) {
	    // This function is called from SW_PARENTOPENING so that we can restore the window placement that was cached in OnParentWindowMinimized
	    unsafe { user32::SetWindowPlacement(self.hwnd.get(), &self.pre_parent_minimized_window_placement); }
	}
	pub fn is_enabled(&self) -> bool {
	    let res = unsafe { !!::user32::IsWindowEnabled(self.hwnd.get()) };
	    res == 1
    }
    pub fn is_regular_window(&self) -> bool {
		let windef_borrow = self.window_definitions.borrow();
		let sec_borrow = windef_borrow.borrow();
		sec_borrow.is_regular_window
	}
	pub fn on_transparency_support_changed(&mut self, new_transparency: WindowTransparency) {
		let windef_borrow = self.window_definitions.borrow();
		if windef_borrow.borrow().transparency_support == WindowTransparency::PerPixel {
            let style = unsafe { user32::GetWindowLongW(self.hwnd.get(), GWL_EXSTYLE) };

            if new_transparency == WindowTransparency::PerPixel {
            	unsafe {
            		user32::SetWindowLongW(self.hwnd.get(), GWL_EXSTYLE, style | WS_EX_COMPOSITED as i32)
            	};
            	let margins: MARGINS = MARGINS {cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1};

            	let res = unsafe { super::DwmExtendFrameIntoClientArea(self.hwnd.get(), &margins) };
            	if res != 0 {
            	    println!("Warning: {}", io::Error::last_os_error());
            	}
            } else {
            	unsafe {
            		user32::SetWindowLongW(self.hwnd.get(), GWL_EXSTYLE, style & !WS_EX_COMPOSITED as i32)
            	};
            }

            // Must call SWP_FRAMECHANGED when updating the style attribute of a window in order to update internal caches (according to MSDN)
            unsafe {
		        user32::SetWindowPos(self.hwnd.get(), ptr::null_mut(), 0, 0, 0, 0, SWP_FRAMECHANGED | SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOOWNERZORDER | SWP_NOREDRAW | SWP_NOSIZE | SWP_NOSENDCHANGING | SWP_NOZORDER)
            };
		}
	}
	pub fn get_aspect_ratio(&self) -> f32 { self.aspect_ratio.get() }
}

impl GenericWindow for WindowsWindow {
	fn reshape_window(&self, new_x: &mut i32, new_y: &mut i32, new_width: &mut i32, new_height: &mut i32) {
		let mut window_info: WINDOWINFO = unsafe { mem::uninitialized() };
		unsafe {
			window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
		    super::GetWindowInfo(self.hwnd.get(), &mut window_info);
		}

		self.aspect_ratio.set(*new_width as f32 / *new_height as f32);

		let windef_borrow = self.window_definitions.borrow();

		if windef_borrow.borrow().has_os_window_border {
			let mut border_rect: RECT = unsafe { mem::zeroed() };
			unsafe {
				user32::AdjustWindowRectEx(&mut border_rect, window_info.dwStyle, FALSE, window_info.dwExStyle)
			};

			*new_x += border_rect.left;
			*new_y += border_rect.top;

			*new_width += border_rect.right - border_rect.left;
		    *new_height += border_rect.bottom - border_rect.top;
		}
		let window_x = new_x;
		let window_y = new_y;

		let virtual_size_changed = *new_width != self.virtual_width.get() || *new_height != self.virtual_height.get();
		self.virtual_width.set(*new_width);
		self.virtual_height.set(*new_height);
		
		if windef_borrow.borrow().size_will_change_often {
			let old_window_rect = window_info.rcWindow;
			let old_width = old_window_rect.right - old_window_rect.left;
		    let old_height = old_window_rect.bottom - old_window_rect.top;

		    let min_retained_width = if windef_borrow.borrow().expected_max_width != -1 { windef_borrow.borrow().expected_max_width } else { old_width };
		    let min_retained_height = if windef_borrow.borrow().expected_max_height != -1 { windef_borrow.borrow().expected_max_height } else { old_height };

		    *new_width = cmp::max(*new_width, cmp::min(old_width, min_retained_width));
		    *new_height = cmp::max(*new_height, cmp::min(old_height, min_retained_height));
		}

		if self.is_maximized() {
			self.restore();
		}

		unsafe {
			user32::SetWindowPos(self.hwnd.get(), ptr::null_mut(), *window_x, *window_y, *new_width, *new_height, SWP_NOZORDER | SWP_NOACTIVATE | if self.window_mode == WindowMode::Fullscreen { SWP_NOSENDCHANGING } else { 0 });
		}

		if windef_borrow.borrow().size_will_change_often && virtual_size_changed {
			let vwidth = self.virtual_width.clone();
			let vheight = self.virtual_height.clone();
			self.adjust_window_region(vwidth.get(), vheight.get());
	    }
	}
	fn get_fullscreen_info(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) -> bool {
		let true_fullscreen = self.window_mode == WindowMode::Fullscreen;

		unsafe {
			let monitor = user32::MonitorFromWindow(self.hwnd.get(), if true_fullscreen { MONITOR_DEFAULTTOPRIMARY } else { MONITOR_DEFAULTTONEAREST });
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
	fn move_window_to(&self, x: &mut i32, y: &mut i32) {
		let windef_borrow = self.window_definitions.borrow();
		if windef_borrow.borrow().has_os_window_border {
			unsafe {
				let window_style = user32::GetWindowLongW(self.hwnd.get(), GWL_STYLE);
		        let window_ex_style = user32::GetWindowLongW(self.hwnd.get(), GWL_EXSTYLE);

		        // This adjusts a zero rect to give us the size of the border
		        let mut border_rect: RECT = mem::zeroed();
		        user32::AdjustWindowRectEx(&mut border_rect, window_style as u32, FALSE, window_ex_style as u32);

		        // Border rect size is negative
		        *x += border_rect.left;
		        *y += border_rect.top;

		        user32::SetWindowPos(self.hwnd.get(), ptr::null_mut(), *x, *y, 0, 0, SWP_NOACTIVATE | SWP_NOSIZE | SWP_NOZORDER);
			}
		}
	}
	fn bring_to_front(&self, force: bool) {
		let windef_borrow = self.window_definitions.borrow();
		if self.is_regular_window() {
			unsafe {
				if user32::IsIconic(self.hwnd.get()) != 0 {
					user32::ShowWindow(self.hwnd.get(), SW_RESTORE);
				} else {
					user32::SetActiveWindow(self.hwnd.get());
				}
			}
		} else {
			let mut hwnd_insert_after = HWND_TOP;
            // By default we activate the window or it isn't actually brought to the front 
		    let mut flags: u32 = SWP_NOMOVE | SWP_NOSIZE | SWP_NOOWNERZORDER;

		    if !force {
		    	flags |= SWP_NOACTIVATE;
		    }

		    if windef_borrow.borrow().is_topmost_window {
                hwnd_insert_after = HWND_TOPMOST;
		    }
            
            unsafe {
		        user32::SetWindowPos(self.hwnd.get(), hwnd_insert_after, 0, 0, 0, 0, flags);
            }
		}
	}
	fn minimize(&self) {
		unsafe { user32::ShowWindow(self.hwnd.get(), SW_MINIMIZE) };
	}
	fn maximize(&self) {
		unsafe { user32::ShowWindow(self.hwnd.get(), SW_MAXIMIZE) };
	}
	fn restore(&self) {
		unsafe { user32::ShowWindow(self.hwnd.get(), SW_RESTORE) };
	}
	fn show(&self) {
		let windef_borrow = self.window_definitions.borrow();
		if !self.is_visible.get() {
			self.is_visible.set(true);

			// Do not activate windows that do not take input; e.g. tool-tips and cursor decorators
		    // Also dont activate if a window wants to appear but not activate itself
		    let should_activate = windef_borrow.borrow().accepts_input && windef_borrow.borrow().activate_when_first_shown;
		    unsafe {
		        user32::ShowWindow(self.hwnd.get(), if should_activate { SW_SHOW } else { SW_SHOWNA });
		    }
		}
	}
	fn hide(&self) {
		if self.is_visible.get() {
			self.is_visible.set(false);
			unsafe { user32::ShowWindow(self.hwnd.get(), SW_HIDE) };
		}
	}
    fn set_window_mode(&mut self, new_window_mode: WindowMode) {
		let windef_borrow = self.window_definitions.borrow();
		if new_window_mode != self.window_mode {
			self.window_mode = new_window_mode;

			let true_fullscreen = new_window_mode == WindowMode::Fullscreen;

			let mut window_style = unsafe {
				user32::GetWindowLongW(self.hwnd.get(), GWL_STYLE)
			};
			let fullscreen_mode_style = WS_POPUP;
			let mut windowed_mode_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
			if self.is_regular_window() {
				if windef_borrow.borrow().supports_maximize {
					windowed_mode_style |= WS_MAXIMIZEBOX;
				}
				if windef_borrow.borrow().supports_minimize {
					windowed_mode_style |= WS_MINIMIZEBOX;
				}
				if windef_borrow.borrow().has_sizing_frame {
					windowed_mode_style |= WS_THICKFRAME;
				} else {
					windowed_mode_style |= WS_BORDER;
				}
			} else {
				windowed_mode_style |= WS_POPUP | WS_BORDER;
			}

			if new_window_mode == WindowMode::WindowedFullscreen || new_window_mode == WindowMode::Fullscreen {
				let is_borderless_game_window = windef_borrow.borrow().window_type == WindowType::GameWindow && !windef_borrow.borrow().has_os_window_border;
			    unsafe {
				    user32::GetWindowPlacement(self.hwnd.get(), &mut self.pre_fullscreen_window_placement);
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
			        user32::SetWindowLongW(self.hwnd.get(), GWL_STYLE, window_style);
			        user32::SetWindowPos(self.hwnd.get(), ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
			    }

			    if !true_fullscreen {
				    // Ensure the window is restored if we are going for WindowedFullscreen
				    unsafe { user32::ShowWindow(self.hwnd.get(), SW_RESTORE); }
			    }

			    unsafe {
			    	// Get the current window position.
			        let mut client_rect: RECT = mem::uninitialized();

			        user32::GetClientRect(self.hwnd.get(), &mut client_rect);

			        // Grab current monitor data for sizing
			        let monitor = user32::MonitorFromWindow(self.hwnd.get(), if true_fullscreen { MONITOR_DEFAULTTOPRIMARY } else { MONITOR_DEFAULTTONEAREST });
			        let mut monitor_info: MONITORINFO = mem::uninitialized();
			        monitor_info.cbSize = mem::size_of::<MONITORINFO>() as u32;
			        user32::GetMonitorInfoW(monitor, &mut monitor_info);

			        // Get the target client width to send to ReshapeWindow.
			        // Preserve the current res if going to true fullscreen and the monitor supports it and allow the calling code
			        // to resize if required.
			        // Else, use the monitor's res for windowed fullscreen.
			        let monitor_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
			        let mut target_client_width = if true_fullscreen { cmp::min(monitor_width, client_rect.right - client_rect.left) } else { monitor_width };

			        let monitor_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;
			        let mut target_client_height = if true_fullscreen { cmp::min(monitor_height, client_rect.bottom - client_rect.top) } else { monitor_height };

     			    // Resize and position fullscreen window
			        self.reshape_window(
				        &mut monitor_info.rcMonitor.left,
				        &mut monitor_info.rcMonitor.top,
				        &mut target_client_width,
				        &mut target_client_height
				    );
			    }
		    } else {
			    // Windowed:

			    // Setup Win32 flags for restored window
			    window_style &= !fullscreen_mode_style as i32;
			    window_style |= windowed_mode_style as i32;
			    unsafe {
			        user32::SetWindowLongW(self.hwnd.get(), GWL_STYLE, window_style);
			        user32::SetWindowPos(self.hwnd.get(), ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);

			        user32::SetWindowPlacement(self.hwnd.get(), &self.pre_fullscreen_window_placement);
			    }
		    }
		}
	}
	fn get_window_mode(&self) -> WindowMode {
		self.window_mode
	}
	fn is_maximized(&self) -> bool {
		let zoomed = unsafe { !!user32::IsZoomed(self.hwnd.get()) };
		zoomed == 1
	}
    fn is_minimized(&self) -> bool {
		let iconic = unsafe { !!user32::IsIconic(self.hwnd.get()) };
		iconic == 1
	}
	fn is_visible(&self) -> bool {
		self.is_visible.get()
	}
	fn get_restored_dimensions(&self, x: &mut i32, y: &mut i32, width: &mut i32, height: &mut i32) -> bool {
		let mut window_placement: WINDOWPLACEMENT = unsafe { mem::uninitialized() };
		window_placement.length = mem::size_of::<WINDOWPLACEMENT>() as u32;

		let res = unsafe { user32::GetWindowPlacement(self.hwnd.get(), &mut window_placement) };
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
	        if user32::GetFocus() != self.hwnd.get() {
		        user32::SetFocus(self.hwnd.get());
	        }
	    }
    }
    fn set_opacity(&self, opacity: f32) {
        unsafe { user32::SetLayeredWindowAttributes(self.hwnd.get(), 0, (opacity * 255.0f32) as u8, LWA_ALPHA); }
    }
    fn enable(&self, enable: bool) {
	    unsafe { user32::EnableWindow(self.hwnd.get(), if enable {1} else {0}) };
    }
    fn is_point_in_window(&self, x: i32, y: i32) -> bool {
    	let mut result = false;
    	let region = self.make_window_region_object();
    	let res = unsafe { !!gdi32::PtInRegion(region, x, y) == 1 };
    	unsafe { gdi32::DeleteObject(mem::transmute(region)); }
    	result == res
    }
    fn get_window_border_size(&self) -> u32 {
		let windef_borrow = self.window_definitions.borrow();
    	if windef_borrow.borrow().window_type == WindowType::GameWindow && !windef_borrow.borrow().has_os_window_border {
		    // Our borderless game windows actually have a thick border to allow sizing, which we draw over to simulate
		    // a borderless window. We return zero here so that the game will correctly behave as if this is truly a
		    // borderless window.
		    return 0;
	    }
	    unsafe {
	    	let mut window_info: WINDOWINFO = mem::uninitialized();
	        window_info.cbSize = mem::size_of::<WINDOWINFO>() as u32;
	        super::GetWindowInfo(self.hwnd.get(), &mut window_info);

	        window_info.cxWindowBorders
	    }
    }
    fn get_os_window_handle(&self) -> *const c_void {
    	self.hwnd.get() as *const c_void
    }
    fn get_window_title_bar_size(&self) -> i32 {
	    unsafe { user32::GetSystemMetrics(SM_CYCAPTION) }
    }
    fn is_foreground_window(&self) -> bool {
    	unsafe { user32::GetForegroundWindow() == self.hwnd.get() }
    }
    fn set_text(&self, text: Vec<u16>) {
    	//TODO: genericize the text variable
    	unsafe { user32::SetWindowTextW(self.hwnd.get(), text.as_ptr()); }
    }
    fn get_definition(&self) -> Rc<RefCell<WindowDefinition>> {
		self.window_definitions.borrow().clone()
    }
    fn adjust_cached_size(&self, size: &mut (i32, i32)) {
		let windef_borrow = self.window_definitions.borrow();
		//Unreal Engine 4's check for if the FGenericWindowDefinition is valid is necessary because this is a pointer. Is it necessary in my code?
		if /* self.window_definitions.is_valid() && */ windef_borrow.borrow().size_will_change_often {
			*size = (self.virtual_width.get(), self.virtual_height.get());
		} else if !self.hwnd.get().is_null() {
			unsafe {
				let mut client_rect: RECT = mem::uninitialized();
		        user32::GetClientRect(self.hwnd.get(), &mut client_rect);
		        size.0 = client_rect.right - client_rect.left;
		        size.1 = client_rect.bottom - client_rect.top;
		    }
		}
	}
}

fn create_window(ex_style: DWORD, class_name: LPCWSTR, window_name: LPCWSTR,
        style: DWORD, x: INT, y: INT, width: INT, height: INT,
        wnd_parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
    ) -> io::Result<HWND> {
	match unsafe {
		user32::CreateWindowExW(
            ex_style, class_name, window_name,
            style, x, y, width, height,
            wnd_parent, menu, instance, param
    )} {
        v if v.is_null() => Err(io::Error::last_os_error()),
        v => Ok(v)
    }
}