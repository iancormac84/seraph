use platform::generic::application_message_handler::WindowSizeLimits;

#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub enum WindowTransparency {
	/** Value indicating that a window does not support transparency */
	None = 0,

	/** Value indicating that a window supports transparency at the window level (one opacity applies to the entire window) */
	PerWindow = 1,

	/** Value indicating that a window supports per-pixel alpha blended transparency */
    PerPixel = 2,
}

#[derive(PartialEq, Clone)]
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