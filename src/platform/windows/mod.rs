use winapi::{ATOM, BOOL, DWORD, HRESULT, HWND, INT, LPCVOID, LPVOID, RECT, UINT, WORD};

pub mod application;
pub mod cursor;
pub mod window;
pub mod xinputinterface;

extern "system" {
	pub fn GetWindowInfo(hwnd: HWND, pwi: PWINDOWINFO) -> BOOL;
	pub fn DwmExtendFrameIntoClientArea(hwnd: HWND, pmarinset: *const MARGINS) -> HRESULT;
	pub fn DwmGetWindowAttribute(hWnd: HWND, dwAttribute: DWORD, pvAttribute: LPVOID,
                                 cbAttribute: DWORD) -> HRESULT;
    pub fn DwmSetWindowAttribute(hwnd: HWND, dwAttribute: DWORD, pvAttribute: LPCVOID,
                                 cbAttribute: DWORD) -> HRESULT;
    pub fn DwmIsCompositionEnabled(pfEnabled: *mut BOOL) -> HRESULT;
}

pub const DLGC_WANTALLKEYS: DWORD = 0x0004;

#[repr(C)]
#[derive(Clone)]
pub struct FILTERKEYS {
    cbSize: UINT,
    dwFlags: DWORD,
    iWaitMSec: DWORD,            // Acceptance Delay
    iDelayMSec: DWORD,           // Delay Until Repeat
    iRepeatMSec: DWORD,          // Repeat Rate
    iBounceMSec: DWORD,          // Debounce Time
}

#[repr(C)]
#[derive(Clone)]
pub struct STICKYKEYS {
    cbSize: UINT,
    dwFlags: DWORD,
}

#[repr(C)]
#[derive(Clone)]
pub struct TOGGLEKEYS {
    cbSize: UINT,
    dwFlags: DWORD,
}

#[repr(C)]
#[derive(Clone)]
pub struct WINDOWINFO {
    cbSize: DWORD,
    rcWindow: RECT,
    rcClient: RECT,
    dwStyle: DWORD,
    dwExStyle: DWORD,
    dwWindowStatus: DWORD,
    cxWindowBorders: UINT,
    cyWindowBorders: UINT,
    atomWindowType: ATOM,
    wCreatorVersion: WORD,
}
pub type PWINDOWINFO = *mut WINDOWINFO;

#[repr(C)]
#[derive(Clone)]
pub struct MARGINS {
    cxLeftWidth: INT,      // width of left border that retains its size
    cxRightWidth: INT,     // width of right border that retains its size
    cyTopHeight: INT,      // height of top border that retains its size
    cyBottomHeight: INT,   // height of bottom border that retains its size
}

ENUM!{enum DWMNCRENDERINGPOLICY{
    DWMNCRP_USEWINDOWSTYLE, // Enable/disable non-client rendering based on window style
    DWMNCRP_DISABLED,       // Disabled non-client rendering; window style is ignored
    DWMNCRP_ENABLED,        // Enabled non-client rendering; window style is ignored
    DWMNCRP_LAST,
}}

pub const FKF_FILTERKEYSON: DWORD  =    0x00000001;
pub const FKF_AVAILABLE: DWORD     =    0x00000002;
pub const FKF_HOTKEYACTIVE: DWORD  =    0x00000004;
pub const FKF_CONFIRMHOTKEY: DWORD =    0x00000008;

pub const IMN_CLOSESTATUSWINDOW: DWORD =           0x0001;
pub const IMN_OPENSTATUSWINDOW: DWORD =            0x0002;
pub const IMN_CHANGECANDIDATE: DWORD =             0x0003;
pub const IMN_CLOSECANDIDATE: DWORD =              0x0004;
pub const IMN_OPENCANDIDATE: DWORD =               0x0005;
pub const IMN_SETCONVERSIONMODE: DWORD =           0x0006;
pub const IMN_SETSENTENCEMODE: DWORD =             0x0007;
pub const IMN_SETOPENSTATUS: DWORD =               0x0008;
pub const IMN_SETCANDIDATEPOS: DWORD =             0x0009;
pub const IMN_SETCOMPOSITIONFONT: DWORD =          0x000A;
pub const IMN_SETCOMPOSITIONWINDOW: DWORD =        0x000B;
pub const IMN_SETSTATUSWINDOWPOS: DWORD =          0x000C;
pub const IMN_GUIDELINE: DWORD =                   0x000D;
pub const IMN_PRIVATE: DWORD =                     0x000E;

pub const IMR_COMPOSITIONWINDOW: DWORD =           0x0001;
pub const IMR_CANDIDATEWINDOW: DWORD =             0x0002;
pub const IMR_COMPOSITIONFONT: DWORD =             0x0003;
pub const IMR_RECONVERTSTRING: DWORD =             0x0004;
pub const IMR_CONFIRMRECONVERTSTRING: DWORD =      0x0005;
pub const IMR_QUERYCHARPOSITION: DWORD =           0x0006;
pub const IMR_DOCUMENTFEED: DWORD =                0x0007;

pub const SKF_STICKYKEYSON: DWORD =    0x00000001;
pub const SKF_AVAILABLE: DWORD =       0x00000002;
pub const SKF_HOTKEYACTIVE: DWORD =    0x00000004;
pub const SKF_CONFIRMHOTKEY: DWORD =   0x00000008;
pub const SKF_HOTKEYSOUND: DWORD =     0x00000010;
pub const SKF_INDICATOR: DWORD =       0x00000020;
pub const SKF_AUDIBLEFEEDBACK: DWORD = 0x00000040;
pub const SKF_TRISTATE: DWORD =        0x00000080;
pub const SKF_TWOKEYSOFF: DWORD =      0x00000100;

pub const TKF_TOGGLEKEYSON: DWORD =    0x00000001;
pub const TKF_AVAILABLE: DWORD =       0x00000002;
pub const TKF_HOTKEYACTIVE: DWORD =    0x00000004;
pub const TKF_CONFIRMHOTKEY: DWORD =   0x00000008;
pub const TKF_HOTKEYSOUND: DWORD =     0x00000010;
pub const TKF_INDICATOR: DWORD =       0x00000020;

pub const DWMWA_NCRENDERING_ENABLED: DWORD = 1;
pub const DWMWA_NCRENDERING_POLICY: DWORD = 2;
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWORD = 2;
pub const DWMWA_ALLOW_NCPAINT: DWORD = 4;
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWORD = 5;
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWORD = 6;
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWORD = 7;
pub const DWMWA_FLIP3D_POLICY: DWORD = 8;
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWORD = 9;
pub const DWMWA_HAS_ICONIC_BITMAP: DWORD = 10;
pub const DWMWA_DISALLOW_PEEK: DWORD = 11;
pub const DWMWA_EXCLUDED_FROM_PEEK: DWORD = 12;
pub const DWMWA_CLOAK: DWORD = 13;
pub const DWMWA_CLOAKED: DWORD = 14;
pub const DWMWA_FREEZE_REPRESENTATION: DWORD = 15;
pub const DWMWA_LAST: DWORD = 16;