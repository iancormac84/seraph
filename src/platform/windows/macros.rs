#[macro_export]
macro_rules! wui_abort {
    ($($args:tt)*) => {
        match format!($($args)*) {
            msg => $crate::windows::wui_abort(&msg, None)
        }
    };
}
