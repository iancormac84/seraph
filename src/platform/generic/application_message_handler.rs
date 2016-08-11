use platform::generic::window::GenericWindow;
use std::rc::Rc;

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

pub struct GamepadKeyNames(pub &'static str);
pub const INVALID: GamepadKeyNames = GamepadKeyNames("");
pub const LEFT_ANALOG_X: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftX");
pub const LEFT_ANALOG_Y: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftY");
pub const RIGHT_ANALOG_X: GamepadKeyNames = GamepadKeyNames("Gamepad_RightX");
pub const RIGHT_ANALOG_Y: GamepadKeyNames = GamepadKeyNames("Gamepad_RightY");
pub const LEFT_TRIGGER_ANALOG: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftTriggerAxis");
pub const RIGHT_TRIGGER_ANALOG: GamepadKeyNames = GamepadKeyNames("Gamepad_RightTriggerAxis");
pub const LEFT_THUMB: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftThumbstick");
pub const RIGHT_THUMB: GamepadKeyNames = GamepadKeyNames("Gamepad_RightThumbstick");
pub const SPECIAL_LEFT: GamepadKeyNames = GamepadKeyNames("Gamepad_Special_Left");
pub const SPECIAL_LEFT_X: GamepadKeyNames = GamepadKeyNames("Gamepad_Special_Left_X");
pub const SPECIAL_LEFT_Y: GamepadKeyNames = GamepadKeyNames("Gamepad_Special_Left_Y");
pub const SPECIAL_RIGHT: GamepadKeyNames = GamepadKeyNames("Gamepad_Special_Right");
pub const FACE_BUTTON_BOTTOM: GamepadKeyNames = GamepadKeyNames("Gamepad_FaceButton_Bottom");
pub const FACE_BUTTON_RIGHT: GamepadKeyNames = GamepadKeyNames("Gamepad_FaceButton_Right");
pub const FACE_BUTTON_LEFT: GamepadKeyNames = GamepadKeyNames("Gamepad_FaceButton_Left");
pub const FACE_BUTTON_TOP: GamepadKeyNames = GamepadKeyNames("Gamepad_FaceButton_Top");
pub const LEFT_SHOULDER: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftShoulder");
pub const RIGHT_SHOULDER: GamepadKeyNames = GamepadKeyNames("Gamepad_RightShoulder");
pub const LEFT_TRIGGER_THRESHOLD: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftTrigger");
pub const RIGHT_TRIGGER_THRESHOLD: GamepadKeyNames = GamepadKeyNames("Gamepad_RightTrigger");
pub const DPAD_UP: GamepadKeyNames = GamepadKeyNames("Gamepad_DPad_Up");
pub const DPAD_DOWN: GamepadKeyNames = GamepadKeyNames("Gamepad_DPad_Down");
pub const DPAD_RIGHT: GamepadKeyNames = GamepadKeyNames("Gamepad_DPad_Right");
pub const DPAD_LEFT: GamepadKeyNames = GamepadKeyNames("Gamepad_DPad_Left");
pub const LEFT_STICK_UP: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftStick_Up");
pub const LEFT_STICK_DOWN: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftStick_Down");
pub const LEFT_STICK_RIGHT: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftStick_Right");
pub const LEFT_STICK_LEFT: GamepadKeyNames = GamepadKeyNames("Gamepad_LeftStick_Left");
pub const RIGHT_STICK_UP: GamepadKeyNames = GamepadKeyNames("Gamepad_RightStick_Up");
pub const RIGHT_STICK_DOWN: GamepadKeyNames = GamepadKeyNames("Gamepad_RightStick_Down");
pub const RIGHT_STICK_RIGHT: GamepadKeyNames = GamepadKeyNames("Gamepad_RightStick_Right");
pub const RIGHT_STICK_LEFT: GamepadKeyNames = GamepadKeyNames("Gamepad_RightStick_Left");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON1: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton1");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON2: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton2");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON3: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton3");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON4: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton4");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON5: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton5");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON6: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton6");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON7: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton7");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON8: GamepadKeyNames = GamepadKeyNames("MotionController_Left_FaceButton8");
pub const MOTION_CONTROLLER_LEFT_SHOULDER: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Shoulder");
pub const MOTION_CONTROLLER_LEFT_TRIGGER: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Trigger");
pub const MOTION_CONTROLLER_LEFT_GRIP1: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Grip1");
pub const MOTION_CONTROLLER_LEFT_GRIP2: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Grip2");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_UP: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick_Up");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_DOWN: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick_Down");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_LEFT: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick_Left");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_RIGHT: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick_Right");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON1: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton1");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON2: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton2");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON3: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton3");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON4: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton4");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON5: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton5");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON6: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton6");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON7: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton7");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON8: GamepadKeyNames = GamepadKeyNames("MotionController_Right_FaceButton8");
pub const MOTION_CONTROLLER_RIGHT_SHOULDER: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Shoulder");
pub const MOTION_CONTROLLER_RIGHT_TRIGGER: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Trigger");
pub const MOTION_CONTROLLER_RIGHT_GRIP1: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Grip1");
pub const MOTION_CONTROLLER_RIGHT_GRIP2: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Grip2");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_UP: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick_Up");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_DOWN: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick_Down");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_LEFT: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick_Left");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_RIGHT: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick_Right");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_X: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick_X");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_Y: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Thumbstick_Y");
pub const MOTION_CONTROLLER_LEFT_TRIGGER_AXIS: GamepadKeyNames = GamepadKeyNames("MotionController_Left_TriggerAxis");
pub const MOTION_CONTROLLER_LEFT_GRIP1_AXIS: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Grip1Axis");
pub const MOTION_CONTROLLER_LEFT_GRIP2_AXIS: GamepadKeyNames = GamepadKeyNames("MotionController_Left_Grip2Axis");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_X: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick_X");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_Y: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Thumbstick_Y");
pub const MOTION_CONTROLLER_RIGHT_TRIGGER_AXIS: GamepadKeyNames = GamepadKeyNames("MotionController_Right_TriggerAxis");
pub const MOTION_CONTROLLER_RIGHT_GRIP1_AXIS: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Grip1Axis");
pub const MOTION_CONTROLLER_RIGHT_GRIP2_AXIS: GamepadKeyNames = GamepadKeyNames("MotionController_Right_Grip2Axis");

pub enum WindowActivation {
	Activate = 0,
	ActivateByMouse,
	Deactivate
}

pub enum WindowZone {
	NotInWindow			= 0,
	TopLeftBorder		= 1,
	TopBorder			= 2,
	TopRightBorder		= 3,
	LeftBorder			= 4,
	ClientArea			= 5,
	RightBorder			= 6,
	BottomLeftBorder	= 7,
	BottomBorder		= 8,
	BottomRightBorder	= 9,
	TitleBar			= 10,
	MinimizeButton		= 11,
	MaximizeButton		= 12,
	CloseButton			= 13,
	SysMenu				= 14,

	/* No zone specified
	Unspecified	= 0,*/
}

pub enum WindowAction {
	ClickedNonClientArea	= 1,
	Maximize				= 2,
	Restore					= 3,
	WindowMenu				= 4,
}


pub enum DropEffect {
	None   = 0,
	Copy   = 1,
	Move   = 2,
	Link   = 3,
}


pub enum GestureEvent {
	None,
	Scroll,
	Magnify,
	Swipe,
	Rotate,
	Count,
}

//TODO are all the Rc<GenericWindow> variables the best they can be?
pub trait ApplicationMessageHandler {
    fn should_process_user_input_messages(&self, platform_window: Rc<GenericWindow>) -> bool;
    fn on_key_char(&self, character: char, is_repeat: bool) -> bool;
    fn on_key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> bool;
    fn on_key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> bool;
    fn on_mouse_down(&self, window: Rc<GenericWindow>, button: MouseButtons) -> bool;
    fn on_mouse_down_with_cursor_pos(&self, window: Rc<GenericWindow>, button: MouseButtons, cursor_pos: (i32, i32)) -> bool;
    fn on_mouse_up(&self, button: MouseButtons) -> bool;
    fn on_mouse_up_with_cursor_pos(&self, button: MouseButtons, cursor_pos: (i32, i32)) -> bool;
    fn on_mouse_double_click(&self, window: Rc<GenericWindow>, button: MouseButtons) -> bool;
    fn on_mouse_double_click_with_cursor_pos(&self, window: Rc<GenericWindow>, button: MouseButtons, cursor_pos: (i32, i32)) -> bool;
    fn on_mouse_wheel(&self, delta: f32) -> bool;
    fn on_mouse_wheel_with_cursor_pos(&self, delta: f32, cursor_pos: (i32, i32)) -> bool;
    fn on_mouse_move(&self) -> bool;
    fn on_raw_mouse_move(&self, x: i32, y: i32) -> bool;
    fn on_cursor_set(&self) -> bool;
    fn on_controller_analog(&self, key_name: GamepadKeyNames, controller_id: i32, analog_value: f32) -> bool;
    fn on_controller_button_pressed(&self, key_name: GamepadKeyNames, controller_id: i32, is_repeat: bool) -> bool;
    fn on_controller_button_released(&self, key_name: GamepadKeyNames, controller_id: i32, is_repeat: bool) -> bool;
    fn on_begin_gesture(&self);
    fn on_touch_gesture(&self, gesture_type: GestureEvent, delta: (i32, i32), wheel_delta: f32, is_direction_inverted_from_device: bool) -> bool;
    fn on_end_gesture(&self);
    fn on_touch_started(&self, window: Rc<GenericWindow>, location: (i32, i32), touch_index: i32, controller_id: i32) -> bool;
    fn on_touch_moved(&self, location: (i32, i32), touch_index: i32, controller_id: i32) -> bool;
    fn on_touch_ended(&self, location: (i32, i32), touch_index: i32, controller_id: i32) -> bool;
    fn on_motion_detected(&self, tilt: (i32, i32, i32), rotation_rate: (i32, i32, i32), gravity: (i32, i32, i32), acceleration: (i32, i32, i32), controller_id: i32) -> bool;
    fn on_size_changed(&self, window: Rc<GenericWindow>, width: i32, height: i32, was_minimized: bool) -> bool;
    fn on_os_paint(&self, window: Rc<GenericWindow>);
    fn get_size_limits_for_window(&self, window: Rc<GenericWindow>) -> WindowSizeLimits;
    fn on_resizing_window(&self, window: Rc<GenericWindow>);
    fn begin_reshaping_window(&self, window: Rc<GenericWindow>) -> bool;
    fn finished_reshaping_window(&self, window: Rc<GenericWindow>);
    fn on_moved_window(&self, window: Rc<GenericWindow>, x: i32, y: i32);
    fn on_window_activation_changed(&self, window: Rc<GenericWindow>, activation_type: WindowActivation) -> bool;
    fn on_application_activation_changed(&self, is_active: bool) -> bool;
    fn on_convertible_laptop_mode_changed(&self) -> bool;
    fn get_window_zone_for_point(&self, window: Rc<GenericWindow>, x: i32, y: i32) -> WindowZone;
    fn on_window_close(&self, window: Rc<GenericWindow>);
    fn on_drag_enter_text(&self, window: Rc<GenericWindow>, text: String) -> DropEffect;
    //Change Vec<String> to Vec<PathBuf> or Vec<OsString>?
    fn on_drag_enter_files(&self, window: Rc<GenericWindow>, files: Vec<String>) -> DropEffect;
    fn on_drag_enter_external(&self, window: Rc<GenericWindow>, text: String, files: Vec<String>) -> DropEffect;
    fn on_drag_over(&self, window: Rc<GenericWindow>) -> DropEffect;
    fn on_drag_leave(&self, window: Rc<GenericWindow>);
    fn on_drag_drop(&self, window: Rc<GenericWindow>) -> DropEffect;
    fn on_window_action(&self, window: Rc<GenericWindow>, action_type: WindowAction) -> bool;
}