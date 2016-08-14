bitflags! {
    flags ModifierKey: u8 {
        const NONE = 0,
        const CONTROL = 1 << 0,
        const ALT = 1 << 1,
        const SHIFT = 1 << 2,
        const COMMAND = 1 << 3,
    }
}

impl ModifierKey {
	pub fn from_bools(control: bool, alt: bool, shift: bool, command: bool) -> ModifierKey {
		let mut modifier_mask: ModifierKey = NONE;
		if control { modifier_mask |= CONTROL };
		if alt     { modifier_mask |= ALT };
		if shift   { modifier_mask |= SHIFT };
		if command { modifier_mask |= COMMAND };

		modifier_mask
	}
}

pub enum PopUpOrientation {
	Horizontal,
	Vertical,
}

//TODO ModifierKeysState struct

pub struct PlatformRect {
	pub left: i32,
	pub top: i32,
	pub right: i32,
	pub bottom: i32,
}

pub struct MonitorInfo {
	name: String,
	id: String,
	native_width: i32,
	native_height: i32,
	is_primary: bool,
}

pub struct DisplayMetrics {
	primary_display_width: i32,
	primary_display_height: i32,
	monitor_info: Vec<MonitorInfo>,
	primary_display_work_area_rect: PlatformRect,
	virtual_display_rect: PlatformRect,
	//TODO: The following should be a Vector2D
	title_safe_padding_size: (i32, i32),
	//TODO: The following should be a Vector2D
	action_safe_padding_size: (i32, i32),
}

pub enum WindowTitleAlignment {
	Left,
	Center,
	Right,
}

//TODO GenericApplication trait