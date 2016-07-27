extern crate winapi;

pub enum WindowMode {
	/** The window is in true fullscreen mode */
	Fullscreen,
	/** The window has no border and takes up the entire area of the screen */
	WindowedFullscreen,
	/** The window has a border and may not take up the entire screen area */
	Windowed,
}

pub struct Window {
	hwnd: winapi::HWND,
	region_height: u32,
	region_width: u32,
    window_mode: WindowMode,
    ole_reference_count: u32,
    pre_fullscreen_window_placement: winapi::WINDOWPLACEMENT,
    pre_parent_minimized_window_placement: winapi::WINDOWPLACEMENT,
    virtual_height: u32,
    virtual_width: u32,
    aspect_ratio: f32,
    is_visible: bool,
}