extern crate seraph;
extern crate user32;
extern crate winapi;

use seraph::generic::{WindowDefinition, WindowSizeLimits, WindowTransparency, WindowType};
use seraph::platform::WindowsApplication;
use std::rc::Rc;
use std::ptr;
use winapi::{HICON, IDI_APPLICATION, LPCWSTR};

fn main() {
    let wd = WindowDefinition {
    	window_type: WindowType::Normal,	
        x_desired_position_on_screen: 0.0,
        y_desired_position_on_screen: 0.0,	
        width_desired_on_screen: 800.0,	
        height_desired_on_screen: 600.0,	
        transparency_support: WindowTransparency::PerWindow,	
        has_os_window_border: true,	
        appears_in_taskbar: true,	
        is_topmost_window: true,	
        accepts_input: true,	
        activate_when_first_shown: true,	
        focus_when_first_shown: true,	
        has_close_button: true,	
        supports_minimize: true,	
        supports_maximize: true,	
        is_modal_window: false,	
        is_regular_window: true,	
        has_sizing_frame: true,
        size_will_change_often: false,	
        should_preserve_aspect_ratio: true,	
        expected_max_width: 1280,	
        expected_max_height: 640,	
        title: "Cormac's Test Window".to_string(),	
        opacity: 1.0,	
        corner_radius: 1,
        size_limits: WindowSizeLimits {
        	min_width: None,
	        min_height: None,
	        max_width: None,
	        max_height: None,
        },
    };
    println!("Made WindowDefinition");
    let icon = unsafe {
    	user32::LoadImageW(ptr::null_mut(), IDI_APPLICATION as LPCWSTR, 1, 0, 0, 0x00008000) as HICON
    };
    println!("Made icon");
    let application = WindowsApplication::new(ptr::null_mut(), icon);
    println!("Made application");
    let new_window = application.make_window();
    println!("Made new_window");
    application.initialize_window(new_window, &Rc::new(wd), None, true);
}