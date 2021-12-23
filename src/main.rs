use seraph::generic::window::GenericWindow;
use seraph::generic::window_definition::WindowActivationPolicy;
use seraph::generic::{WindowDefinition, WindowSizeLimits, WindowTransparency, WindowType};
use seraph::windows::application::{
    create_windows_application, WindowsApplication, WINDOWS_APPLICATION,
};
use seraph::windows::utils::ToWide;
use windows::Win32::{
    Foundation::PWSTR,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{LoadImageW, IDI_APPLICATION},
};

fn main() {
    let wd = WindowDefinition {
        window_type: WindowType::Normal,
        x_desired_position_on_screen: 0.0,
        y_desired_position_on_screen: 0.0,
        width_desired_on_screen: 800.0,
        height_desired_on_screen: 600.0,
        transparency_support: WindowTransparency::PerWindow,
        has_os_window_border: true,
        appears_in_taskbar: true,
        is_topmost_window: true,
        accepts_input: true,
        activation_policy: WindowActivationPolicy::Always,
        focus_when_first_shown: true,
        has_close_button: true,
        supports_minimize: true,
        supports_maximize: true,
        is_modal_window: false,
        is_regular_window: true,
        has_sizing_frame: true,
        size_will_change_often: false,
        should_preserve_aspect_ratio: true,
        expected_max_width: 1280,
        expected_max_height: 640,
        title: "Cormac's Test Window".to_string(),
        opacity: 1.0,
        corner_radius: 1,
        size_limits: WindowSizeLimits {
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
        },
        manual_dpi: false,
    };
    println!("Made WindowDefinition");
    let icon = unsafe { LoadImageW(0, IDI_APPLICATION, 1, 0, 0, 0x00008000) };
    println!("Made icon");
    let inst_handle = unsafe { GetModuleHandleW(PWSTR("".to_wide_null().as_mut_ptr())) };
    let application = create_windows_application(inst_handle, icon.0);
    println!("Made application. address is {:p}", application);
    println!("Also, application debug is {:#?}", &*application);
    let rc_window = application.make_window();
    application.initialize_window(&rc_window, wd, &None, true);
    rc_window.borrow().show();
    application.pump_messages(0.0);
}
