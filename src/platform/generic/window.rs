use crate::generic::window_definition::WindowDefinition;
use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum WindowMode {
    /** The window is in true fullscreen mode */
    Fullscreen,
    /** The window has no border and takes up the entire area of the screen */
    WindowedFullscreen,
    /** The window has a border and may not take up the entire screen area */
    Windowed,
}

pub trait GenericWindow {
    fn reshape_window(
        &self,
        new_x: &mut i32,
        new_y: &mut i32,
        new_width: &mut i32,
        new_height: &mut i32,
    );
    fn get_fullscreen_info(
        &self,
        x: &mut i32,
        y: &mut i32,
        width: &mut i32,
        height: &mut i32,
    ) -> bool;
    fn move_window_to(&self, x: &mut i32, y: &mut i32);
    fn bring_to_front(&self, force: bool);
    //fn HACK_force_to_front(&mut self);
    //fn destroy(&mut self);
    fn minimize(&self);
    fn maximize(&self);
    fn restore(&self);
    fn show(&self);
    fn hide(&self);
    fn set_window_mode(&mut self, new_window_mode: WindowMode);
    fn get_window_mode(&self) -> WindowMode;
    fn is_maximized(&self) -> bool;
    fn is_minimized(&self) -> bool;
    fn is_visible(&self) -> bool;
    fn get_restored_dimensions(
        &self,
        x: &mut i32,
        y: &mut i32,
        width: &mut i32,
        height: &mut i32,
    ) -> bool;
    fn set_window_focus(&mut self);
    fn set_opacity(&self, opacity: f32);
    fn enable(&self, enable: bool);
    fn is_point_in_window(&self, x: i32, y: i32) -> bool;
    fn get_window_border_size(&self) -> u32;
    fn get_window_title_bar_size(&self) -> i32;
    fn get_os_window_handle(&self) -> *const c_void;
    fn is_foreground_window(&self) -> bool;
    fn is_fullscreen_supported(&self) -> bool;
    fn set_text(&self, text: Vec<u16>);
    fn get_definition(&self) -> Rc<RefCell<WindowDefinition>>;
    fn adjust_cached_size(&self, size: &mut (i32, i32));
}
