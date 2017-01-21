use cgmath::Vector2;
use platform::generic::cursor::{ICursor, MouseCursor};
use std::ffi::OsStr;
use std::{mem, ptr};
use std::os::windows::ffi::OsStrExt;
use user32;
use winapi::{BOOL, FALSE, HCURSOR, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_IBEAM, IDC_NO, IDC_SIZEALL, IDC_SIZENESW, IDC_SIZENS, IDC_SIZENWSE, IDC_SIZEWE, POINT, RECT, TRUE};

type FloatVec2 = Vector2<f32>;

#[derive(PartialEq, Debug)]
pub struct WindowsCursor {
	pub current_type: MouseCursor,
    pub cursor_handles: [HCURSOR; 15],
}

impl WindowsCursor {
	pub fn new() -> WindowsCursor {
		unsafe {
		    let mut windows_cursor: WindowsCursor = mem::uninitialized();
		    for i in 0..15 {
		    	windows_cursor.cursor_handles[i] = ptr::null_mut();
		    	let mut cursor_handle: HCURSOR = ptr::null_mut();
		    	match MouseCursor::from_usize(i) {
		    		MouseCursor::None | MouseCursor::Custom => {},
		    		MouseCursor::Default => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_ARROW);
		    		},
		    		MouseCursor::TextEditBeam => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_IBEAM);
		    		},
		    		MouseCursor::ResizeLeftRight => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_SIZEWE);
		    		},
		    		MouseCursor::ResizeUpDown => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_SIZENS);
		    		},
		    		MouseCursor::ResizeSouthEast => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_SIZENWSE);
		    		},
		    		MouseCursor::ResizeSouthWest => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_SIZENESW);
		    		},
		    		MouseCursor::CardinalCross => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_SIZEALL);
		    		},
		    		MouseCursor::Crosshairs => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_CROSS);
		    		},
		    		MouseCursor::Hand => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_HAND);
		    		},
		    		//TODO
		    		MouseCursor::GrabHand => {
		    			cursor_handle = user32::LoadCursorFromFileW(OsStr::new(r"F:\Programs\Epic Games\4.14\Engine\Content\Editor\Slate\Cursor\grabhand.cur").encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr());
		    			if cursor_handle.is_null() {
				            // Failed to load file, fall back
				            cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_HAND);
			            }
		    		},
		    		MouseCursor::GrabHandClosed => {
		    			cursor_handle = user32::LoadCursorFromFileW(OsStr::new(r"F:\Programs\Epic Games\4.14\Engine\Content\Editor\Slate\Cursor\grabhand_closed.cur").encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr());
		    			if cursor_handle.is_null() {
				            // Failed to load file, fall back
				            cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_HAND);
			            }
		    		},
		    		MouseCursor::SlashedCircle => {
		    			cursor_handle = user32::LoadCursorW(ptr::null_mut(), IDC_NO);
		    		},
		    		MouseCursor::EyeDropper => {
		    			cursor_handle = user32::LoadCursorFromFileW(OsStr::new(r"F:\Programs\Epic Games\4.14\Engine\Content\Editor\Slate\Cursor\eyedropper.cur").encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>().as_ptr());
		    		}
		    	}
		    	windows_cursor.cursor_handles[i] = cursor_handle;
		    }
            windows_cursor.set_type(MouseCursor::Default);
            windows_cursor
		}
	}
	pub fn set_custom_shape(&mut self, cursor_handle: HCURSOR) {
		let mouse_cursor = MouseCursor::Custom;
		self.cursor_handles[mouse_cursor.to_usize()] = cursor_handle;
	}
}

impl ICursor for WindowsCursor {
	fn get_position(&self) -> FloatVec2 {
		unsafe {
            let mut cursor_pos: POINT = mem::uninitialized();
            user32::GetCursorPos(&mut cursor_pos);

            FloatVec2::new(cursor_pos.x as f32, cursor_pos.y as f32)
		}
	}
	fn set_position(&mut self, x: i32, y: i32) {
		unsafe { user32::SetCursorPos(x, y); }
	}
	fn set_type(&mut self, new_cursor: MouseCursor) {
		self.current_type = new_cursor;
		unsafe {
			user32::SetCursor(self.cursor_handles[new_cursor as usize]);
		}
	}
	fn get_type<'a>(&'a self) -> &'a MouseCursor {
		&self.current_type
	}
	fn get_size(&self, width: &mut i32, height: &mut i32) {
		*width = 16;
		*height = 16;
	}
	fn show(&self, show: BOOL) {
        unsafe {
        	if show == TRUE {
        		// Show mouse cursor. Each time ShowCursor(true) is called an internal value is incremented so we 
		        // call ShowCursor until the cursor is actually shown (>= 0 value returned by showcursor)
		        while user32::ShowCursor(TRUE) < 0 {};
	        } else {
	        	// Disable the cursor.  Wait until its actually disabled.
		        while user32::ShowCursor(FALSE) >= 0 {};
        	}
        }
	}
	fn lock(&self, bounds: *const RECT) {
		unsafe { user32::ClipCursor(bounds); }
	}
}