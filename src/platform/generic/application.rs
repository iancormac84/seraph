use crate::generic::application_message_handler::ApplicationMessageHandler;
use crate::generic::cursor::ICursor;
use crate::generic::window::GenericWindow;
use bitflags::bitflags;
use std::rc::Rc;

pub static mut DEBUG_SAFE_ZONE_RATIO: f32 = 1.0;
pub static mut DEBUG_ACTION_ZONE_RATIO: f32 = 1.0;

bitflags! {
    struct ModifierKey: u8 {
        const NONE = 0;
        const CONTROL = 1 << 0;
        const ALT = 1 << 1;
        const SHIFT = 1 << 2;
        const COMMAND = 1 << 3;
    }
}

impl ModifierKey {
    pub fn from_bools(control: bool, alt: bool, shift: bool, command: bool) -> ModifierKey {
        let mut modifier_mask = ModifierKey::NONE;
        if control {
            modifier_mask |= ModifierKey::CONTROL
        };
        if alt {
            modifier_mask |= ModifierKey::ALT
        };
        if shift {
            modifier_mask |= ModifierKey::SHIFT
        };
        if command {
            modifier_mask |= ModifierKey::COMMAND
        };

        modifier_mask
    }
}

pub enum PopUpOrientation {
    Horizontal,
    Vertical,
}

//TODO ModifierKeysState struct

#[derive(PartialEq, Default, Debug, Clone)]
pub struct PlatformRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct MonitorInfo {
    pub name: String,
    pub id: String,
    pub native_width: i32,
    pub native_height: i32,
    pub is_primary: bool,
}

pub enum WindowTitleAlignment {
    Left,
    Center,
    Right,
}

pub trait GenericApplication {
    type Cursor: ICursor;
    type Window: GenericWindow;

    fn set_message_handler(&mut self, in_message_handler: &Rc<dyn ApplicationMessageHandler>);
    fn get_message_handler(&self) -> &Rc<dyn ApplicationMessageHandler>;
    //fn poll_game_device_state(&self, time_delta: f32);
    fn pump_messages(&self, time_delta: f32);
    //fn process_deferred_events(&self, time_delta: f32);
    //fn tick(&self, time_delta: f32);
    //fn make_window(&self) -> Rc<Self::Window>;
}
