use crate::generic::application_message_handler::{ApplicationMessageHandler, GamepadKeyNames};
use crate::generic::iinputinterface::ForceFeedbackValues;
use std::rc::Rc;

pub const MAX_NUM_XINPUT_CONTROLLERS: usize = 4;
pub const MAX_NUM_CONTROLLER_BUTTONS: usize = 24;

struct ControllerState {
    /** Last frame's button states, so we only send events on edges */
    button_states: [bool; MAX_NUM_CONTROLLER_BUTTONS],

    /** Next time a repeat event should be generated for each button */
    next_repeat_time: [f64; MAX_NUM_CONTROLLER_BUTTONS],

    /** Raw Left thumb x analog value */
    left_x_analog: u16,

    /** Raw left thumb y analog value */
    left_y_analog: u16,

    /** Raw Right thumb x analog value */
    right_x_analog: u16,

    /** Raw Right thumb x analog value */
    right_y_analog: u16,

    /** Left Trigger analog value */
    left_trigger_analog: u8,

    /** Right trigger analog value */
    right_trigger_analog: u8,

    /** If the controller is currently connected */
    is_connected: bool,

    /** Id of the controller */
    controller_id: i32,

    /** Current force feedback values */
    force_feedback: ForceFeedbackValues,

    last_large_value: f32,
    last_small_value: f32,
}

pub struct XInputInterface {
    needs_controller_state_update: bool,
    is_gamepad_attached: bool,
    x360_to_xbox_controller_mapping: [u8; MAX_NUM_CONTROLLER_BUTTONS],
    controller_states: [ControllerState; MAX_NUM_XINPUT_CONTROLLERS],
    initial_button_replay_delay: f32,
    button_replay_delay: f32,
    buttons: [GamepadKeyNames; MAX_NUM_CONTROLLER_BUTTONS],
    message_handler: Rc<dyn ApplicationMessageHandler>,
}

impl XInputInterface {
    pub fn set_needs_controller_state_update(&mut self) {
        self.needs_controller_state_update = true;
    }
}
