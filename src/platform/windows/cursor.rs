use crate::{
    core::math::color::Color,
    generic::cursor::{ICursor, MouseCursor, TagRect},
    platform::windows::utils::ToWide,
};
use glam::Vector2;
use std::path::Path;
use windows::Win32::{
    Foundation::{POINT, PWSTR, RECT},
    UI::WindowsAndMessaging::{
        ClipCursor, GetCursorPos, LoadCursorFromFileW, LoadCursorW, SetCursor, SetCursorPos,
        ShowCursor, HCURSOR, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_IBEAM, IDC_NO, IDC_SIZEALL,
        IDC_SIZENESW, IDC_SIZENS, IDC_SIZENWSE, IDC_SIZEWE,
    },
};

type FloatVec2 = Vector2<f32>;
impl TagRect for RECT {}

#[derive(PartialEq, Debug)]
pub struct WindowsCursor {
    pub current_type: MouseCursor,
    pub cursor_handles: [HCURSOR; 15],
    pub cursor_override_handles: [HCURSOR; 15],
}

impl WindowsCursor {
    pub fn new() -> WindowsCursor {
        let mut cursor_handles = [0; 15];
        let mut cursor_override_handles = [0; 15];
        unsafe {
            for i in 0..15 {
                let mut cursor_handle = HCURSOR::default();
                match MouseCursor::from_usize(i) {
                    MouseCursor::None | MouseCursor::Custom => {}
                    MouseCursor::Default => {
                        cursor_handle = LoadCursorW(0, IDC_ARROW);
                    }
                    MouseCursor::TextEditBeam => {
                        cursor_handle = LoadCursorW(0, IDC_IBEAM);
                    }
                    MouseCursor::ResizeLeftRight => {
                        cursor_handle = LoadCursorW(0, IDC_SIZEWE);
                    }
                    MouseCursor::ResizeUpDown => {
                        cursor_handle = LoadCursorW(0, IDC_SIZENS);
                    }
                    MouseCursor::ResizeSouthEast => {
                        cursor_handle = LoadCursorW(0, IDC_SIZENWSE);
                    }
                    MouseCursor::ResizeSouthWest => {
                        cursor_handle = LoadCursorW(0, IDC_SIZENESW);
                    }
                    MouseCursor::CardinalCross => {
                        cursor_handle = LoadCursorW(0, IDC_SIZEALL);
                    }
                    MouseCursor::Crosshairs => {
                        cursor_handle = LoadCursorW(0, IDC_CROSS);
                    }
                    MouseCursor::Hand => {
                        cursor_handle = LoadCursorW(0, IDC_HAND);
                    }
                    //TODO
                    MouseCursor::GrabHand => {
                        cursor_handle = LoadCursorFromFileW(PWSTR(r"F:\Programs\Epic Games\4.14\Engine\Content\Editor\Slate\Cursor\grabhand.cur".to_wide_null().as_mut_ptr()));
                        if cursor_handle == 0 {
                            // Failed to load file, fall back
                            cursor_handle = LoadCursorW(0, IDC_HAND);
                        }
                    }
                    MouseCursor::GrabHandClosed => {
                        cursor_handle = LoadCursorFromFileW(PWSTR(r"F:\Programs\Epic Games\4.14\Engine\Content\Editor\Slate\Cursor\grabhand_closed.cur".to_wide_null().as_mut_ptr()));
                        if cursor_handle == 0 {
                            // Failed to load file, fall back
                            cursor_handle = LoadCursorW(0, IDC_HAND);
                        }
                    }
                    MouseCursor::SlashedCircle => {
                        cursor_handle = LoadCursorW(0, IDC_NO);
                    }
                    MouseCursor::EyeDropper => {
                        cursor_handle = LoadCursorFromFileW(PWSTR(r"F:\Programs\Epic Games\4.14\Engine\Content\Editor\Slate\Cursor\eyedropper.cur".to_wide_null().as_mut_ptr()));
                    }
                }
                cursor_handles[i] = cursor_handle;
            }
        }
        WindowsCursor {
            current_type: MouseCursor::Default,
            cursor_handles,
            cursor_override_handles,
        }
    }
    pub fn set_custom_shape(&mut self, cursor_handle: HCURSOR) {
        let mouse_cursor = MouseCursor::Custom;
        self.cursor_handles[mouse_cursor.to_usize()] = cursor_handle;
    }
}

impl ICursor for WindowsCursor {
    type Rect = RECT;
    fn create_cursor_from_file<P: AsRef<Path>>(
        path_to_cursor_without_extension: P,
        hotspot: Vector2<f32>,
    ) -> Option<Self> {
        None
    }
    fn is_create_cursor_from_rgba_buffer_supported() -> bool {
        true
    }
    fn create_cursor_from_rgba_buffer(
        pixels: Color,
        width: i32,
        height: i32,
        hotspot: Vector2<f32>,
    ) -> Option<Self> {
        None
    }
    fn get_position(&self) -> FloatVec2 {
        unsafe {
            let mut cursor_pos = POINT::default();
            GetCursorPos(&mut cursor_pos);

            FloatVec2::new(cursor_pos.x as f32, cursor_pos.y as f32)
        }
    }
    fn set_position(&mut self, x: i32, y: i32) {
        unsafe {
            SetCursorPos(x, y);
        }
    }
    fn set_type(&mut self, new_cursor: MouseCursor) {
        self.current_type = new_cursor;
        unsafe {
            if self.cursor_override_handles[new_cursor as usize] != 0 {
                SetCursor(self.cursor_override_handles[new_cursor as usize]);
            } else {
                SetCursor(self.cursor_handles[new_cursor as usize]);
            }
        }
    }
    fn get_type<'a>(&'a self) -> &'a MouseCursor {
        &self.current_type
    }
    fn get_size(&self, width: &mut i32, height: &mut i32) {
        *width = 16;
        *height = 16;
    }
    fn show(&self, show: bool) {
        unsafe {
            if show {
                // Show mouse cursor. Each time ShowCursor(true) is called an internal value is incremented so we
                // call ShowCursor until the cursor is actually shown (>= 0 value returned by showcursor)
                while ShowCursor(true) < 0 {}
            } else {
                // Disable the cursor.  Wait until its actually disabled.
                while ShowCursor(false) >= 0 {}
            }
        }
    }
    fn lock(&self, bounds: *const Self::Rect) {
        unsafe {
            // Lock/Unlock the cursor
            ClipCursor(bounds);
            // If the cursor is not visible and we're running game, assume we're in a mode where the mouse is controlling the camera and lock it to the center of the widget.
        }
    }
    fn set_type_shape(
        &mut self,
        cursor_type: MouseCursor,
        in_cursor_handle: *const std::ffi::c_void,
    ) {
        let cursor_handle = in_cursor_handle as HCURSOR;
        self.cursor_override_handles[cursor_type as usize] = cursor_handle;
        if self.current_type == cursor_type {
            self.set_type(self.current_type);
        }
    }
}
