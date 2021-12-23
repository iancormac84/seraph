use std::ptr;

pub enum ForceFeedbackChannelType {
    LEFT_LARGE,
    LEFT_SMALL,
    RIGHT_LARGE,
    RIGHT_SMALL,
}

pub struct ForceFeedbackValues {
    left_large: f32,
    left_small: f32,
    right_large: f32,
    right_small: f32,
}

impl ForceFeedbackValues {
    pub fn new() -> ForceFeedbackValues {
        ForceFeedbackValues {
            left_large: 0.0f32,
            left_small: 0.0f32,
            right_large: 0.0f32,
            right_small: 0.0f32,
        }
    }
}

pub struct HapticFeedbackBuffer {
    raw_data: Vec<u8>,
    current_ptr: u32,
    buffer_length: i32,
    samples_sent: i32,
    finished_playing: bool,
    sampling_rate: i32,
    scale_factor: f32,
}

impl HapticFeedbackBuffer {
    pub fn new() -> HapticFeedbackBuffer {
        HapticFeedbackBuffer {
            raw_data: vec![],
            current_ptr: 0,
            buffer_length: 0,
            samples_sent: 0,
            finished_playing: false,
            sampling_rate: 0,
            scale_factor: 0.0f32,
        }
    }
    pub fn needs_update(&self) -> bool {
        !self.finished_playing
    }
}

pub struct HapticFeedbackValues {
    frequency: f32,
    amplitude: f32,
    haptic_buffer: *mut HapticFeedbackBuffer,
}

impl HapticFeedbackValues {
    pub fn new() -> HapticFeedbackValues {
        HapticFeedbackValues {
            frequency: 0.0f32,
            amplitude: 0.0f32,
            haptic_buffer: ptr::null_mut(),
        }
    }
    pub fn from_freq_and_amplitude(in_frequency: f32, in_amplitude: f32) -> HapticFeedbackValues {
        HapticFeedbackValues {
            frequency: if in_frequency < 0.0f32 {
                0.0f32
            } else if in_frequency > 1.0f32 {
                1.0f32
            } else {
                in_frequency
            },
            amplitude: if in_amplitude < 0.0f32 {
                0.0f32
            } else if in_amplitude > 1.0f32 {
                1.0f32
            } else {
                in_amplitude
            },
            haptic_buffer: ptr::null_mut(),
        }
    }
}

pub trait IInputIterface {
    fn set_force_feedback_channel_value(
        &mut self,
        controller_id: i32,
        channel_type: ForceFeedbackChannelType,
        value: f32,
    );
    fn set_force_feedback_channel_values(
        &mut self,
        controller_id: i32,
        values: &ForceFeedbackValues,
    );
    fn set_haptic_feedback_values(
        &mut self,
        controller_id: i32,
        hand: i32,
        values: &HapticFeedbackValues,
    );
    //fn set_light_color(&mut self, controller_id: i32, struct FColor Color);
}
