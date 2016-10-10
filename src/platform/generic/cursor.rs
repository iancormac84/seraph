use cgmath::Vector2;
#[cfg(target_os = "windows")]
use winapi::{BOOL, RECT};

type FloatVec2 = Vector2<f32>;

#[derive(Copy, Clone)]
pub enum MouseCursor {
    /** Causes no mouse cursor to be visible */
    None,

    /** Default cursor (arrow) */
    Default,

    /** Text edit beam */
    TextEditBeam,

    /** Resize horizontal */
    ResizeLeftRight,

    /** Resize vertical */
    ResizeUpDown,

    /** Resize diagonal */
    ResizeSouthEast,

    /** Resize other diagonal */
    ResizeSouthWest,

    /** MoveItem */
    CardinalCross,

    /** Target Cross */
    Crosshairs,

    /** Hand cursor */
    Hand,

    /** Grab Hand cursor */
    GrabHand,

    /** Grab Hand cursor closed */
    GrabHandClosed,

    /** a circle with a diagonal line through it */
    SlashedCircle,

    /** Eye-dropper cursor for picking colors */
    EyeDropper,

    /** Custom cursor shape for platforms that support setting a native cursor shape. Same as specifying None if not set. */
    Custom,
}

impl MouseCursor {
    pub fn from_usize(n: usize) -> MouseCursor {
        match n {
            0 => MouseCursor::None,
            1 => MouseCursor::Default,
            2 => MouseCursor::TextEditBeam,
            3 => MouseCursor::ResizeLeftRight,
            4 => MouseCursor::ResizeUpDown,
            5 => MouseCursor::ResizeSouthEast,
            6 => MouseCursor::ResizeSouthWest,
            7 => MouseCursor::CardinalCross,
            8 => MouseCursor::Crosshairs,
            9 => MouseCursor::Hand,
            10 => MouseCursor::GrabHand,
            11 => MouseCursor::GrabHandClosed,
            12 => MouseCursor::SlashedCircle,
            13 => MouseCursor::EyeDropper,
            14 => MouseCursor::Custom,
            _ => MouseCursor::Default,
        }
    }
    pub fn to_usize(&self) -> usize {
        match *self {
            MouseCursor::None => 0,
            MouseCursor::Default => 1,
            MouseCursor::TextEditBeam => 2,
            MouseCursor::ResizeLeftRight => 3,
            MouseCursor::ResizeUpDown => 4,
            MouseCursor::ResizeSouthEast => 5,
            MouseCursor::ResizeSouthWest => 6,
            MouseCursor::CardinalCross => 7,
            MouseCursor::Crosshairs => 8,
            MouseCursor::Hand => 9,
            MouseCursor::GrabHand => 10,
            MouseCursor::GrabHandClosed => 11,
            MouseCursor::SlashedCircle => 12,
            MouseCursor::EyeDropper => 13,
            MouseCursor::Custom => 14,
        }
    }
}

pub trait ICursor {
	/** The position of the cursor */
	fn get_position(&self) -> FloatVec2;

	/** Sets the position of the cursor */
	fn set_position(&mut self, x: i32, y: i32);

	/** Sets the cursor */
	fn set_type(&mut self, new_cursor: MouseCursor);

	/** Gets the current type of the cursor */
	fn get_type(&self) -> &MouseCursor;

	/** Gets the size of the cursor */
	fn get_size(&self, width: &mut i32, height: &mut i32);

	/**
	 * Shows or hides the cursor
	 *
	 * @param bShow	true to show the mouse cursor, false to hide it
	 */
	fn show(&self, show: BOOL);

	/**
	 * Locks the cursor to the passed in bounds
	 * 
	 * @param Bounds	The bounds to lock the cursor to.  Pass None to unlock.
	 */
	fn lock(&self, bounds: *const RECT);
}