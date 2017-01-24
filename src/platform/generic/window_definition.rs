use platform::generic::application_message_handler::WindowSizeLimits;
use std::default;

#[derive(PartialEq, Clone, Debug)]
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
	CursorDecorator,
	/** Value indicating that this is a game window */
	GameWindow,
}

#[derive(PartialEq, Clone, Debug)]
pub enum WindowTransparency {
	/** Value indicating that a window does not support transparency */
	None = 0,

	/** Value indicating that a window supports transparency at the window level (one opacity applies to the entire window) */
	PerWindow = 1,

	/** Value indicating that a window supports per-pixel alpha blended transparency */
    PerPixel = 2,
}

#[derive(PartialEq, Clone, Debug)]
pub struct WindowDefinition {
	/** Window type */
	pub window_type: WindowType,
	
	/** The initially desired horizontal screen position */
	pub x_desired_position_on_screen: f32,
	/** The initially desired vertical screen position */
	pub y_desired_position_on_screen: f32,

	/** The initially desired width */
	pub width_desired_on_screen: f32,
	/** The initially desired height */
	pub height_desired_on_screen: f32,

	/** the level of transparency supported by this window */
	pub transparency_support: WindowTransparency,

	/** true if the window is using the os window border instead of a slate created one */
	pub has_os_window_border: bool,
	/** should this window show up in the taskbar */
	pub appears_in_taskbar: bool,
	/** true if the window should be on top of all other windows; false otherwise */
	pub is_topmost_window: bool,
	/** true if the window accepts input; false if the window is non-interactive */
	pub accepts_input: bool,
	/** true if this window will be activated when it is first shown */
	pub activate_when_first_shown: bool,
	/** true if this window will be focused when it is first shown */
	pub focus_when_first_shown: bool,
	/** true if this window displays an enabled close button on the toolbar area */
	pub has_close_button: bool,
	/** true if this window displays an enabled minimize button on the toolbar area */
	pub supports_minimize: bool,
	/** true if this window displays an enabled maximize button on the toolbar area */
	pub supports_maximize: bool,

	/** true if the window is modal (prevents interacting with its parent) */
	pub is_modal_window: bool,
	/** true if this is a vanilla window, or one being used for some special purpose: e.g. tooltip or menu */
	pub is_regular_window: bool,
	/** true if this is a user-sized window with a thick edge */
	pub has_sizing_frame: bool,
	/** true if we expect the size of this window to change often, such as if its animated, or if it recycled for tool-tips. */
	pub size_will_change_often: bool,
	/** true if the window should preserve its aspect ratio when resized by user */
	pub should_preserve_aspect_ratio: bool,
	/** The expected maximum width of the window.  May be used for performance optimization when SizeWillChangeOften is set. */
	pub expected_max_width: i32,
	/** The expected maximum height of the window.  May be used for performance optimization when SizeWillChangeOften is set. */
	pub expected_max_height: i32,

	/** the title of the window */
	pub title: String,
	/** opacity of the window (0-1) */
	pub opacity: f32,
	/** the radius of the corner rounding of the window */
	pub corner_radius: i32,

	pub size_limits: WindowSizeLimits,
}

impl default::Default for WindowDefinition {
	fn default() -> WindowDefinition {
		WindowDefinition {
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
            	min_width: Some(50.0),
	            min_height: Some(50.0),
	            max_width: Some(1366.0),
	            max_height: Some(705.0),
            },
        }
	}
}