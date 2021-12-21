use crate::generic::window::GenericWindow;
use cgmath::{Vector2, Vector3};
use std::rc::Rc;

pub enum MouseButtons {
    Left = 0,
    Middle,
    Right,
    Thumb01,
    Thumb02,

    Invalid,
}

//I came across a similar implementation in the Github repository https://github.com/coeuvre/hammer-rs.git
#[derive(PartialEq, Clone, Debug)]
pub struct WindowSizeLimits {
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
}

impl WindowSizeLimits {
    pub fn set_min_width(&mut self, in_value: Option<f32>) -> &WindowSizeLimits {
        self.min_width = in_value;
        self
    }
    pub fn get_min_width(&self) -> Option<f32> {
        self.min_width
    }

    pub fn set_min_height(&mut self, in_value: Option<f32>) -> &WindowSizeLimits {
        self.min_height = in_value;
        self
    }
    pub fn get_min_height(&self) -> Option<f32> {
        self.min_height
    }

    pub fn set_max_width(&mut self, in_value: Option<f32>) -> &WindowSizeLimits {
        self.max_width = in_value;
        self
    }
    pub fn get_max_width(&self) -> Option<f32> {
        self.max_width
    }

    pub fn set_max_height(&mut self, in_value: Option<f32>) -> &WindowSizeLimits {
        self.max_height = in_value;
        self
    }
    pub fn get_max_height(&self) -> Option<f32> {
        self.max_height
    }
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
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON1: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton1");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON2: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton2");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON3: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton3");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON4: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton4");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON5: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton5");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON6: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton6");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON7: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton7");
pub const MOTION_CONTROLLER_LEFT_FACE_BUTTON8: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_FaceButton8");
pub const MOTION_CONTROLLER_LEFT_SHOULDER: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Shoulder");
pub const MOTION_CONTROLLER_LEFT_TRIGGER: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Trigger");
pub const MOTION_CONTROLLER_LEFT_GRIP1: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Grip1");
pub const MOTION_CONTROLLER_LEFT_GRIP2: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Grip2");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_UP: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick_Up");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_DOWN: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick_Down");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_LEFT: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick_Left");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_RIGHT: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick_Right");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON1: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton1");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON2: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton2");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON3: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton3");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON4: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton4");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON5: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton5");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON6: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton6");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON7: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton7");
pub const MOTION_CONTROLLER_RIGHT_FACE_BUTTON8: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_FaceButton8");
pub const MOTION_CONTROLLER_RIGHT_SHOULDER: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Shoulder");
pub const MOTION_CONTROLLER_RIGHT_TRIGGER: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Trigger");
pub const MOTION_CONTROLLER_RIGHT_GRIP1: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Grip1");
pub const MOTION_CONTROLLER_RIGHT_GRIP2: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Grip2");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_UP: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick_Up");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_DOWN: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick_Down");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_LEFT: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick_Left");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_RIGHT: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick_Right");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_X: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick_X");
pub const MOTION_CONTROLLER_LEFT_THUMBSTICK_Y: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Thumbstick_Y");
pub const MOTION_CONTROLLER_LEFT_TRIGGER_AXIS: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_TriggerAxis");
pub const MOTION_CONTROLLER_LEFT_GRIP1_AXIS: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Grip1Axis");
pub const MOTION_CONTROLLER_LEFT_GRIP2_AXIS: GamepadKeyNames =
    GamepadKeyNames("MotionController_Left_Grip2Axis");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_X: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick_X");
pub const MOTION_CONTROLLER_RIGHT_THUMBSTICK_Y: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Thumbstick_Y");
pub const MOTION_CONTROLLER_RIGHT_TRIGGER_AXIS: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_TriggerAxis");
pub const MOTION_CONTROLLER_RIGHT_GRIP1_AXIS: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Grip1Axis");
pub const MOTION_CONTROLLER_RIGHT_GRIP2_AXIS: GamepadKeyNames =
    GamepadKeyNames("MotionController_Right_Grip2Axis");

pub enum WindowActivation {
    Activate = 0,
    ActivateByMouse,
    Deactivate,
}

pub enum WindowZone {
    NotInWindow,
    TopLeftBorder,
    TopBorder,
    TopRightBorder,
    LeftBorder,
    ClientArea,
    RightBorder,
    BottomLeftBorder,
    BottomBorder,
    BottomRightBorder,
    TitleBar,
    MinimizeButton,
    MaximizeButton,
    CloseButton,
    SysMenu,
    /* No zone specified
    Unspecified	= 0,*/
}

impl WindowZone {
    pub fn to_usize(&self) -> usize {
        match *self {
            WindowZone::NotInWindow => 0,
            WindowZone::TopLeftBorder => 1,
            WindowZone::TopBorder => 2,
            WindowZone::TopRightBorder => 3,
            WindowZone::LeftBorder => 4,
            WindowZone::ClientArea => 5,
            WindowZone::RightBorder => 6,
            WindowZone::BottomLeftBorder => 7,
            WindowZone::BottomBorder => 8,
            WindowZone::BottomRightBorder => 9,
            WindowZone::TitleBar => 10,
            WindowZone::MinimizeButton => 11,
            WindowZone::MaximizeButton => 12,
            WindowZone::CloseButton => 13,
            WindowZone::SysMenu => 14,
        }
    }
}

pub enum WindowAction {
    ClickedNonClientArea = 1,
    Maximize = 2,
    Restore = 3,
    WindowMenu = 4,
}

pub enum DropEffect {
    None = 0,
    Copy = 1,
    Move = 2,
    Link = 3,
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
//NOTE the only implementer of the FGenericApplicationMessageHandler class is FSlateApplication.
//NOTE SWindow (E:\Devel\study\UnrealEngine\Engine\Source\Runtime\SlateCore\Public\Widgets\SWindow.h) holds a FGenericWindow member variable.
pub trait ApplicationMessageHandler {
    fn should_process_user_input_messages(&self, platform_window: &Rc<dyn GenericWindow>) -> bool;
    fn on_key_char(&self, character: char, is_repeat: bool) -> bool;
    fn on_key_down(&self, key_code: i32, character_code: u32, is_repeat: bool) -> bool;
    fn on_key_up(&self, key_code: i32, character_code: u32, is_repeat: bool) -> bool;
    fn on_mouse_down(&self, window: &Rc<dyn GenericWindow>, button: MouseButtons) -> bool;
    fn on_mouse_down_with_cursor_pos(
        &self,
        window: &Rc<dyn GenericWindow>,
        button: MouseButtons,
        cursor_pos: Vector2<f32>,
    ) -> bool;
    fn on_mouse_up(&self, button: MouseButtons) -> bool;
    fn on_mouse_up_with_cursor_pos(&self, button: MouseButtons, cursor_pos: Vector2<f32>) -> bool;
    fn on_mouse_double_click(&self, window: &Rc<dyn GenericWindow>, button: MouseButtons) -> bool;
    fn on_mouse_double_click_with_cursor_pos(
        &self,
        window: &Rc<dyn GenericWindow>,
        button: MouseButtons,
        cursor_pos: Vector2<f32>,
    ) -> bool;
    fn on_mouse_wheel(&self, delta: f32) -> bool;
    fn on_mouse_wheel_with_cursor_pos(&self, delta: f32, cursor_pos: Vector2<f32>) -> bool;
    fn on_mouse_move(&self) -> bool;
    fn on_raw_mouse_move(&self, x: i32, y: i32) -> bool;
    fn on_cursor_set(&self) -> bool;
    fn on_controller_analog(
        &self,
        key_name: GamepadKeyNames,
        controller_id: i32,
        analog_value: f32,
    ) -> bool;
    fn on_controller_button_pressed(
        &self,
        key_name: GamepadKeyNames,
        controller_id: i32,
        is_repeat: bool,
    ) -> bool;
    fn on_controller_button_released(
        &self,
        key_name: GamepadKeyNames,
        controller_id: i32,
        is_repeat: bool,
    ) -> bool;
    fn on_begin_gesture(&self);
    fn on_touch_gesture(
        &self,
        gesture_type: GestureEvent,
        delta: Vector2<f32>,
        wheel_delta: f32,
        is_direction_inverted_from_device: bool,
    ) -> bool;
    fn on_end_gesture(&self);
    fn on_touch_started(
        &self,
        window: &Rc<dyn GenericWindow>,
        location: Vector2<f32>,
        touch_index: i32,
        controller_id: i32,
    ) -> bool;
    fn on_touch_moved(&self, location: Vector2<f32>, touch_index: i32, controller_id: i32) -> bool;
    fn on_touch_ended(&self, location: Vector2<f32>, touch_index: i32, controller_id: i32) -> bool;
    fn on_motion_detected(
        &self,
        tilt: Vector3<f32>,
        rotation_rate: Vector3<f32>,
        gravity: Vector3<f32>,
        acceleration: Vector3<f32>,
        controller_id: i32,
    ) -> bool;
    fn on_size_changed(
        &self,
        window: &Rc<dyn GenericWindow>,
        width: i32,
        height: i32,
        was_minimized: bool,
    ) -> bool;
    fn on_os_paint(&self, window: &Rc<dyn GenericWindow>);
    fn get_size_limits_for_window(&self, window: &Rc<dyn GenericWindow>) -> WindowSizeLimits;
    fn on_resizing_window(&self, window: &Rc<dyn GenericWindow>);
    fn begin_reshaping_window(&self, window: &Rc<dyn GenericWindow>) -> bool;
    fn finished_reshaping_window(&self, window: &Rc<dyn GenericWindow>);
    fn on_moved_window(&self, window: &Rc<dyn GenericWindow>, x: i32, y: i32);
    fn on_window_activation_changed(
        &self,
        window: &Rc<dyn GenericWindow>,
        activation_type: WindowActivation,
    ) -> bool;
    fn on_application_activation_changed(&self, is_active: bool) -> bool;
    fn on_convertible_laptop_mode_changed(&self) -> bool;
    fn get_window_zone_for_point(
        &self,
        window: &Rc<dyn GenericWindow>,
        x: i32,
        y: i32,
    ) -> WindowZone;
    fn on_window_close(&self, window: &Rc<dyn GenericWindow>);
    fn on_drag_enter_text(&self, window: &Rc<dyn GenericWindow>, text: &String) -> DropEffect;
    //Change Vec<String> to Vec<PathBuf> or Vec<OsString>?
    fn on_drag_enter_files(
        &self,
        window: &Rc<dyn GenericWindow>,
        files: &Vec<String>,
    ) -> DropEffect;
    fn on_drag_enter_external(
        &self,
        window: &Rc<dyn GenericWindow>,
        text: &String,
        files: &Vec<String>,
    ) -> DropEffect;
    fn on_drag_over(&self, window: &Rc<dyn GenericWindow>) -> DropEffect;
    fn on_drag_leave(&self, window: &Rc<dyn GenericWindow>);
    fn on_drag_drop(&self, window: &Rc<dyn GenericWindow>) -> DropEffect;
    fn on_window_action(&self, window: &Rc<dyn GenericWindow>, action_type: WindowAction) -> bool;
}
