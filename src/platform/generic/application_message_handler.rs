pub enum MouseButtons {
	Left = 0,
	Middle,
	Right,
	Thumb01,
	Thumb02,

	Invalid
}

//I came across a similar implementation in the Github repository https://github.com/coeuvre/hammer-rs.git
#[derive(PartialEq, Clone)]
pub struct WindowSizeLimits {
	min_width: Option<f32>,
	min_height: Option<f32>,
	max_width: Option<f32>,
	max_height: Option<f32>,
}

//TODO implement GenericApplicationMessageHandler trait.