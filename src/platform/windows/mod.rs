use std::collections::BTreeMap;
use std::{io, process};
use std::io::Write;
use winapi;
use winapi::{ATOM, BOOL, DWORD, HRESULT, HWND, INT, LPCVOID, LPVOID, RECT, UINT, WORD};

pub mod application;
pub mod cursor;
pub mod dialog;
#[macro_use] pub mod macros;
pub mod utils;
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

lazy_static! {
    static ref WINDOWS_MESSAGE_STRINGS: BTreeMap<u32, &'static str> = {
        let mut result: BTreeMap<u32, &'static str> = BTreeMap::new();
        result.insert(winapi::WM_NULL, "WM_NULL");
        result.insert(winapi::WM_CREATE, "WM_CREATE");
        result.insert(winapi::WM_DESTROY, "WM_DESTROY");
        result.insert(winapi::WM_MOVE, "WM_MOVE");
        result.insert(winapi::WM_SIZE, "WM_SIZE");
        result.insert(winapi::WM_ACTIVATE, "WM_ACTIVATE");
        result.insert(winapi::WM_SETFOCUS, "WM_SETFOCUS");
        result.insert(winapi::WM_KILLFOCUS, "WM_KILLFOCUS");
        result.insert(winapi::WM_ENABLE, "WM_ENABLE");
        result.insert(winapi::WM_SETREDRAW, "WM_SETREDRAW");
        result.insert(winapi::WM_SETTEXT, "WM_SETTEXT");
        result.insert(winapi::WM_GETTEXT, "WM_GETTEXT");
        result.insert(winapi::WM_GETTEXTLENGTH, "WM_GETTEXTLENGTH");
        result.insert(winapi::WM_PAINT, "WM_PAINT");
        result.insert(winapi::WM_CLOSE, "WM_CLOSE");
        result.insert(winapi::WM_QUERYENDSESSION, "WM_QUERYENDSESSION");
        result.insert(winapi::WM_QUERYOPEN, "WM_QUERYOPEN");
        result.insert(winapi::WM_ENDSESSION, "WM_ENDSESSION");
        result.insert(winapi::WM_QUIT, "WM_QUIT");
        result.insert(winapi::WM_ERASEBKGND, "WM_ERASEBKGND");
        result.insert(winapi::WM_SYSCOLORCHANGE, "WM_SYSCOLORCHANGE");
        result.insert(winapi::WM_SHOWWINDOW, "WM_SHOWWINDOW");
        result.insert(winapi::WM_WININICHANGE, "WM_WININICHANGE");
        result.insert(winapi::WM_DEVMODECHANGE, "WM_DEVMODECHANGE");
        result.insert(winapi::WM_ACTIVATEAPP, "WM_ACTIVATEAPP");
        result.insert(winapi::WM_FONTCHANGE, "WM_FONTCHANGE");
        result.insert(winapi::WM_TIMECHANGE, "WM_TIMECHANGE");
        result.insert(winapi::WM_CANCELMODE, "WM_CANCELMODE");
        result.insert(winapi::WM_SETCURSOR, "WM_SETCURSOR");
        result.insert(winapi::WM_MOUSEACTIVATE, "WM_MOUSEACTIVATE");
        result.insert(winapi::WM_CHILDACTIVATE, "WM_CHILDACTIVATE");
        result.insert(winapi::WM_QUEUESYNC, "WM_QUEUESYNC");
        result.insert(winapi::WM_GETMINMAXINFO, "WM_GETMINMAXINFO");
        result.insert(winapi::WM_PAINTICON, "WM_PAINTICON");
        result.insert(winapi::WM_ICONERASEBKGND, "WM_ICONERASEBKGND");
        result.insert(winapi::WM_NEXTDLGCTL, "WM_NEXTDLGCTL");
        result.insert(winapi::WM_SPOOLERSTATUS, "WM_SPOOLERSTATUS");
        result.insert(winapi::WM_DRAWITEM, "WM_DRAWITEM");
        result.insert(winapi::WM_MEASUREITEM, "WM_MEASUREITEM");
        result.insert(winapi::WM_DELETEITEM, "WM_DELETEITEM");
        result.insert(winapi::WM_VKEYTOITEM, "WM_VKEYTOITEM");
        result.insert(winapi::WM_CHARTOITEM, "WM_CHARTOITEM");
        result.insert(winapi::WM_SETFONT, "WM_SETFONT");
        result.insert(winapi::WM_GETFONT, "WM_GETFONT");
        result.insert(winapi::WM_SETHOTKEY, "WM_SETHOTKEY");
        result.insert(winapi::WM_GETHOTKEY, "WM_GETHOTKEY");
        result.insert(winapi::WM_QUERYDRAGICON, "WM_QUERYDRAGICON");
        result.insert(winapi::WM_COMPAREITEM, "WM_COMPAREITEM");
        result.insert(winapi::WM_GETOBJECT, "WM_GETOBJECT");
        result.insert(winapi::WM_COMPACTING, "WM_COMPACTING");
        result.insert(winapi::WM_COMMNOTIFY, "WM_COMMNOTIFY");
        result.insert(winapi::WM_WINDOWPOSCHANGING, "WM_WINDOWPOSCHANGING");
        result.insert(winapi::WM_WINDOWPOSCHANGED, "WM_WINDOWPOSCHANGED");
        result.insert(winapi::WM_POWER, "WM_POWER");
        result.insert(winapi::WM_COPYDATA, "WM_COPYDATA");
        result.insert(winapi::WM_CANCELJOURNAL, "WM_CANCELJOURNAL");
        result.insert(winapi::WM_NOTIFY, "WM_NOTIFY");
        result.insert(winapi::WM_INPUTLANGCHANGEREQUEST, "WM_INPUTLANGCHANGEREQUEST");
        result.insert(winapi::WM_INPUTLANGCHANGE, "WM_INPUTLANGCHANGE");
        result.insert(winapi::WM_TCARD, "WM_TCARD");
        result.insert(winapi::WM_HELP, "WM_HELP");
        result.insert(winapi::WM_USERCHANGED, "WM_USERCHANGED");
        result.insert(winapi::WM_NOTIFYFORMAT, "WM_NOTIFYFORMAT");
        result.insert(winapi::WM_CONTEXTMENU, "WM_CONTEXTMENU");
        result.insert(winapi::WM_STYLECHANGING, "WM_STYLECHANGING");
        result.insert(winapi::WM_STYLECHANGED, "WM_STYLECHANGED");
        result.insert(winapi::WM_DISPLAYCHANGE, "WM_DISPLAYCHANGE");
        result.insert(winapi::WM_GETICON, "WM_GETICON");
        result.insert(winapi::WM_SETICON, "WM_SETICON");
        result.insert(winapi::WM_NCCREATE, "WM_NCCREATE");
        result.insert(winapi::WM_NCDESTROY, "WM_NCDESTROY");
        result.insert(winapi::WM_NCCALCSIZE, "WM_NCCALCSIZE");
        result.insert(winapi::WM_NCHITTEST, "WM_NCHITTEST");
        result.insert(winapi::WM_NCPAINT, "WM_NCPAINT");
        result.insert(winapi::WM_NCACTIVATE, "WM_NCACTIVATE");
        result.insert(winapi::WM_GETDLGCODE, "WM_GETDLGCODE");
        result.insert(winapi::WM_SYNCPAINT, "WM_SYNCPAINT");
        result.insert(winapi::WM_NCMOUSEMOVE, "WM_NCMOUSEMOVE");
        result.insert(winapi::WM_NCLBUTTONDOWN, "WM_NCLBUTTONDOWN");
        result.insert(winapi::WM_NCLBUTTONUP, "WM_NCLBUTTONUP");
        result.insert(winapi::WM_NCLBUTTONDBLCLK, "WM_NCLBUTTONDBLCLK");
        result.insert(winapi::WM_NCRBUTTONDOWN, "WM_NCRBUTTONDOWN");
        result.insert(winapi::WM_NCRBUTTONUP, "WM_NCRBUTTONUP");
        result.insert(winapi::WM_NCRBUTTONDBLCLK, "WM_NCRBUTTONDBLCLK");
        result.insert(winapi::WM_NCMBUTTONDOWN, "WM_NCMBUTTONDOWN");
        result.insert(winapi::WM_NCMBUTTONUP, "WM_NCMBUTTONUP");
        result.insert(winapi::WM_NCMBUTTONDBLCLK, "WM_NCMBUTTONDBLCLK");
        result.insert(winapi::WM_NCXBUTTONDOWN, "WM_NCXBUTTONDOWN");
        result.insert(winapi::WM_NCXBUTTONUP, "WM_NCXBUTTONUP");
        result.insert(winapi::WM_NCXBUTTONDBLCLK, "WM_NCXBUTTONDBLCLK");
        result.insert(winapi::WM_INPUT, "WM_INPUT");
        result.insert(winapi::WM_KEYDOWN, "WM_KEYDOWN");
        result.insert(winapi::WM_KEYUP, "WM_KEYUP");
        result.insert(winapi::WM_CHAR, "WM_CHAR");
        result.insert(winapi::WM_DEADCHAR, "WM_DEADCHAR");
        result.insert(winapi::WM_SYSKEYDOWN, "WM_SYSKEYDOWN");
        result.insert(winapi::WM_SYSKEYUP, "WM_SYSKEYUP");
        result.insert(winapi::WM_SYSCHAR, "WM_SYSCHAR");
        result.insert(winapi::WM_SYSDEADCHAR, "WM_SYSDEADCHAR");
        result.insert(winapi::WM_UNICHAR, "WM_UNICHAR");
        result.insert(winapi::WM_IME_STARTCOMPOSITION, "WM_IME_STARTCOMPOSITION");
        result.insert(winapi::WM_IME_ENDCOMPOSITION, "WM_IME_ENDCOMPOSITION");
        result.insert(winapi::WM_IME_COMPOSITION, "WM_IME_COMPOSITION");
        result.insert(winapi::WM_INITDIALOG, "WM_INITDIALOG");
        result.insert(winapi::WM_COMMAND, "WM_COMMAND");
        result.insert(winapi::WM_SYSCOMMAND, "WM_SYSCOMMAND");
        result.insert(winapi::WM_TIMER, "WM_TIMER");
        result.insert(winapi::WM_HSCROLL, "WM_HSCROLL");
        result.insert(winapi::WM_VSCROLL, "WM_VSCROLL");
        result.insert(winapi::WM_INITMENU, "WM_INITMENU");
        result.insert(winapi::WM_INITMENUPOPUP, "WM_INITMENUPOPUP");
        result.insert(winapi::WM_MENUSELECT, "WM_MENUSELECT");
        result.insert(winapi::WM_MENUCHAR, "WM_MENUCHAR");
        result.insert(winapi::WM_ENTERIDLE, "WM_ENTERIDLE");
        result.insert(winapi::WM_MENURBUTTONUP, "WM_MENURBUTTONUP");
        result.insert(winapi::WM_MENUDRAG, "WM_MENUDRAG");
        result.insert(winapi::WM_MENUGETOBJECT, "WM_MENUGETOBJECT");
        result.insert(winapi::WM_UNINITMENUPOPUP, "WM_UNINITMENUPOPUP");
        result.insert(winapi::WM_MENUCOMMAND, "WM_MENUCOMMAND");
        result.insert(winapi::WM_CHANGEUISTATE, "WM_CHANGEUISTATE");
        result.insert(winapi::WM_UPDATEUISTATE, "WM_UPDATEUISTATE");
        result.insert(winapi::WM_QUERYUISTATE, "WM_QUERYUISTATE");
        result.insert(winapi::WM_CTLCOLORMSGBOX, "WM_CTLCOLORMSGBOX");
        result.insert(winapi::WM_CTLCOLOREDIT, "WM_CTLCOLOREDIT");
        result.insert(winapi::WM_CTLCOLORLISTBOX, "WM_CTLCOLORLISTBOX");
        result.insert(winapi::WM_CTLCOLORBTN, "WM_CTLCOLORBTN");
        result.insert(winapi::WM_CTLCOLORDLG, "WM_CTLCOLORDLG");
        result.insert(winapi::WM_CTLCOLORSCROLLBAR, "WM_CTLCOLORSCROLLBAR");
        result.insert(winapi::WM_CTLCOLORSTATIC, "WM_CTLCOLORSTATIC");
        result.insert(winapi::WM_MOUSEMOVE, "WM_MOUSEMOVE");
        result.insert(winapi::WM_LBUTTONDOWN, "WM_LBUTTONDOWN");
        result.insert(winapi::WM_LBUTTONUP, "WM_LBUTTONUP");
        result.insert(winapi::WM_LBUTTONDBLCLK, "WM_LBUTTONDBLCLK");
        result.insert(winapi::WM_RBUTTONDOWN, "WM_RBUTTONDOWN");
        result.insert(winapi::WM_RBUTTONUP, "WM_RBUTTONUP");
        result.insert(winapi::WM_RBUTTONDBLCLK, "WM_RBUTTONDBLCLK");
        result.insert(winapi::WM_MBUTTONDOWN, "WM_MBUTTONDOWN");
        result.insert(winapi::WM_MBUTTONUP, "WM_MBUTTONUP");
        result.insert(winapi::WM_MBUTTONDBLCLK, "WM_MBUTTONDBLCLK");
        result.insert(winapi::WM_MOUSEWHEEL, "WM_MOUSEWHEEL");
        result.insert(winapi::WM_XBUTTONDOWN, "WM_XBUTTONDOWN");
        result.insert(winapi::WM_XBUTTONUP, "WM_XBUTTONUP");
        result.insert(winapi::WM_XBUTTONDBLCLK, "WM_XBUTTONDBLCLK");
        result.insert(winapi::WM_MOUSEHWHEEL, "WM_MOUSEHWHEEL");
        result.insert(winapi::WM_PARENTNOTIFY, "WM_PARENTNOTIFY");
        result.insert(winapi::WM_ENTERMENULOOP, "WM_ENTERMENULOOP");
        result.insert(winapi::WM_EXITMENULOOP, "WM_EXITMENULOOP");
        result.insert(winapi::WM_NEXTMENU, "WM_NEXTMENU");
        result.insert(winapi::WM_SIZING, "WM_SIZING");
        result.insert(winapi::WM_CAPTURECHANGED, "WM_CAPTURECHANGED");
        result.insert(winapi::WM_MOVING, "WM_MOVING");
        result.insert(winapi::WM_POWERBROADCAST, "WM_POWERBROADCAST");
        result.insert(winapi::WM_DEVICECHANGE, "WM_DEVICECHANGE");
        result.insert(winapi::WM_MDICREATE, "WM_MDICREATE");
        result.insert(winapi::WM_MDIDESTROY, "WM_MDIDESTROY");
        result.insert(winapi::WM_MDIACTIVATE, "WM_MDIACTIVATE");
        result.insert(winapi::WM_MDIRESTORE, "WM_MDIRESTORE");
        result.insert(winapi::WM_MDINEXT, "WM_MDINEXT");
        result.insert(winapi::WM_MDIMAXIMIZE, "WM_MDIMAXIMIZE");
        result.insert(winapi::WM_MDITILE, "WM_MDITILE");
        result.insert(winapi::WM_MDICASCADE, "WM_MDICASCADE");
        result.insert(winapi::WM_MDIICONARRANGE, "WM_MDIICONARRANGE");
        result.insert(winapi::WM_MDIGETACTIVE, "WM_MDIGETACTIVE");
        result.insert(winapi::WM_MDISETMENU, "WM_MDISETMENU");
        result.insert(winapi::WM_ENTERSIZEMOVE, "WM_ENTERSIZEMOVE");
        result.insert(winapi::WM_EXITSIZEMOVE, "WM_EXITSIZEMOVE");
        result.insert(winapi::WM_DROPFILES, "WM_DROPFILES");
        result.insert(winapi::WM_MDIREFRESHMENU, "WM_MDIREFRESHMENU");
        result.insert(winapi::WM_IME_SETCONTEXT, "WM_IME_SETCONTEXT");
        result.insert(winapi::WM_IME_NOTIFY, "WM_IME_NOTIFY");
        result.insert(winapi::WM_IME_CONTROL, "WM_IME_CONTROL");
        result.insert(winapi::WM_IME_COMPOSITIONFULL, "WM_IME_COMPOSITIONFULL");
        result.insert(winapi::WM_IME_SELECT, "WM_IME_SELECT");
        result.insert(winapi::WM_IME_CHAR, "WM_IME_CHAR");
        result.insert(winapi::WM_IME_REQUEST, "WM_IME_REQUEST");
        result.insert(winapi::WM_IME_KEYDOWN, "WM_IME_KEYDOWN");
        result.insert(winapi::WM_IME_KEYUP, "WM_IME_KEYUP");
        result.insert(winapi::WM_NCMOUSEHOVER, "WM_NCMOUSEHOVER");
        result.insert(winapi::WM_MOUSEHOVER, "WM_MOUSEHOVER");
        result.insert(winapi::WM_NCMOUSELEAVE, "WM_NCMOUSELEAVE");
        result.insert(winapi::WM_MOUSELEAVE, "WM_MOUSELEAVE");
        result.insert(winapi::WM_WTSSESSION_CHANGE, "WM_WTSSESSION_CHANGE");
        result.insert(winapi::WM_TABLET_FIRST, "WM_TABLET_FIRST");
        result.insert(winapi::WM_TABLET_FIRST + 1, "WM_TABLET_FIRST + 1");
        result.insert(winapi::WM_TABLET_FIRST + 2, "WM_TABLET_FIRST + 2");
        result.insert(winapi::WM_TABLET_FIRST + 3, "WM_TABLET_FIRST + 3");
        result.insert(winapi::WM_TABLET_FIRST + 4, "WM_TABLET_FIRST + 4");
        result.insert(winapi::WM_TABLET_FIRST + 5, "WM_TABLET_FIRST + 5");
        result.insert(winapi::WM_TABLET_FIRST + 6, "WM_TABLET_FIRST + 6");
        result.insert(winapi::WM_TABLET_FIRST + 7, "WM_TABLET_FIRST + 7");
        result.insert(winapi::WM_TABLET_FIRST + 8, "WM_TABLET_FIRST + 8");
        result.insert(winapi::WM_TABLET_FIRST + 9, "WM_TABLET_FIRST + 9");
        result.insert(winapi::WM_TABLET_FIRST + 10, "WM_TABLET_FIRST + 10");
        result.insert(winapi::WM_TABLET_FIRST + 11, "WM_TABLET_FIRST + 11");
        result.insert(winapi::WM_TABLET_FIRST + 12, "WM_TABLET_FIRST + 12");
        result.insert(winapi::WM_TABLET_FIRST + 13, "WM_TABLET_FIRST + 13");
        result.insert(winapi::WM_TABLET_FIRST + 14, "WM_TABLET_FIRST + 14");
        result.insert(winapi::WM_TABLET_FIRST + 15, "WM_TABLET_FIRST + 15");
        result.insert(winapi::WM_TABLET_FIRST + 16, "WM_TABLET_FIRST + 16");
        result.insert(winapi::WM_TABLET_FIRST + 17, "WM_TABLET_FIRST + 17");
        result.insert(winapi::WM_TABLET_FIRST + 18, "WM_TABLET_FIRST + 18");
        result.insert(winapi::WM_TABLET_FIRST + 19, "WM_TABLET_FIRST + 19");
        result.insert(winapi::WM_TABLET_FIRST + 20, "WM_TABLET_FIRST + 20");
        result.insert(winapi::WM_TABLET_FIRST + 21, "WM_TABLET_FIRST + 21");
        result.insert(winapi::WM_TABLET_FIRST + 22, "WM_TABLET_FIRST + 22");
        result.insert(winapi::WM_TABLET_FIRST + 23, "WM_TABLET_FIRST + 23");
        result.insert(winapi::WM_TABLET_FIRST + 24, "WM_TABLET_FIRST + 24");
        result.insert(winapi::WM_TABLET_FIRST + 25, "WM_TABLET_FIRST + 25");
        result.insert(winapi::WM_TABLET_FIRST + 26, "WM_TABLET_FIRST + 26");
        result.insert(winapi::WM_TABLET_FIRST + 27, "WM_TABLET_FIRST + 27");
        result.insert(winapi::WM_TABLET_FIRST + 28, "WM_TABLET_FIRST + 28");
        result.insert(winapi::WM_TABLET_FIRST + 29, "WM_TABLET_FIRST + 29");
        result.insert(winapi::WM_TABLET_FIRST + 30, "WM_TABLET_FIRST + 30");
        result.insert(winapi::WM_TABLET_LAST, "WM_TABLET_LAST");
        result.insert(winapi::WM_CUT, "WM_CUT");
        result.insert(winapi::WM_COPY, "WM_COPY");
        result.insert(winapi::WM_PASTE, "WM_PASTE");
        result.insert(winapi::WM_CLEAR, "WM_CLEAR");
        result.insert(winapi::WM_UNDO, "WM_UNDO");
        result.insert(winapi::WM_RENDERFORMAT, "WM_RENDERFORMAT");
        result.insert(winapi::WM_RENDERALLFORMATS, "WM_RENDERALLFORMATS");
        result.insert(winapi::WM_DESTROYCLIPBOARD, "WM_DESTROYCLIPBOARD");
        result.insert(winapi::WM_DRAWCLIPBOARD, "WM_DRAWCLIPBOARD");
        result.insert(winapi::WM_PAINTCLIPBOARD, "WM_PAINTCLIPBOARD");
        result.insert(winapi::WM_VSCROLLCLIPBOARD, "WM_VSCROLLCLIPBOARD");
        result.insert(winapi::WM_SIZECLIPBOARD, "WM_SIZECLIPBOARD");
        result.insert(winapi::WM_ASKCBFORMATNAME, "WM_ASKCBFORMATNAME");
        result.insert(winapi::WM_CHANGECBCHAIN, "WM_CHANGECBCHAIN");
        result.insert(winapi::WM_HSCROLLCLIPBOARD, "WM_HSCROLLCLIPBOARD");
        result.insert(winapi::WM_QUERYNEWPALETTE, "WM_QUERYNEWPALETTE");
        result.insert(winapi::WM_PALETTEISCHANGING, "WM_PALETTEISCHANGING");
        result.insert(winapi::WM_PALETTECHANGED, "WM_PALETTECHANGED");
        result.insert(winapi::WM_HOTKEY, "WM_HOTKEY");
        result.insert(winapi::WM_PRINT, "WM_PRINT");
        result.insert(winapi::WM_PRINTCLIENT, "WM_PRINTCLIENT");
        result.insert(winapi::WM_APPCOMMAND, "WM_APPCOMMAND");
        result.insert(winapi::WM_THEMECHANGED, "WM_THEMECHANGED");
        result.insert(winapi::WM_HANDHELDFIRST, "WM_HANDHELDFIRST");
        result.insert(winapi::WM_HANDHELDFIRST + 1, "WM_HANDHELDFIRST + 1");
        result.insert(winapi::WM_HANDHELDFIRST + 2, "WM_HANDHELDFIRST + 2");
        result.insert(winapi::WM_HANDHELDFIRST + 3, "WM_HANDHELDFIRST + 3");
        result.insert(winapi::WM_HANDHELDFIRST + 4, "WM_HANDHELDFIRST + 4");
        result.insert(winapi::WM_HANDHELDFIRST + 5, "WM_HANDHELDFIRST + 5");
        result.insert(winapi::WM_HANDHELDFIRST + 6, "WM_HANDHELDFIRST + 6");
        result.insert(winapi::WM_HANDHELDLAST, "WM_HANDHELDLAST");
        result.insert(winapi::WM_AFXFIRST, "WM_AFXFIRST");
        result.insert(winapi::WM_AFXFIRST + 1, "WM_AFXFIRST + 1");
        result.insert(winapi::WM_AFXFIRST + 2, "WM_AFXFIRST + 2");
        result.insert(winapi::WM_AFXFIRST + 3, "WM_AFXFIRST + 3");
        result.insert(winapi::WM_AFXFIRST + 4, "WM_AFXFIRST + 4");
        result.insert(winapi::WM_AFXFIRST + 5, "WM_AFXFIRST + 5");
        result.insert(winapi::WM_AFXFIRST + 6, "WM_AFXFIRST + 6");
        result.insert(winapi::WM_AFXFIRST + 7, "WM_AFXFIRST + 7");
        result.insert(winapi::WM_AFXFIRST + 8, "WM_AFXFIRST + 8");
        result.insert(winapi::WM_AFXFIRST + 9, "WM_AFXFIRST + 9");
        result.insert(winapi::WM_AFXFIRST + 10, "WM_AFXFIRST + 10");
        result.insert(winapi::WM_AFXFIRST + 11, "WM_AFXFIRST + 11");
        result.insert(winapi::WM_AFXFIRST + 12, "WM_AFXFIRST + 12");
        result.insert(winapi::WM_AFXFIRST + 13, "WM_AFXFIRST + 13");
        result.insert(winapi::WM_AFXFIRST + 14, "WM_AFXFIRST + 14");
        result.insert(winapi::WM_AFXFIRST + 15, "WM_AFXFIRST + 15");
        result.insert(winapi::WM_AFXFIRST + 16, "WM_AFXFIRST + 16");
        result.insert(winapi::WM_AFXFIRST + 17, "WM_AFXFIRST + 17");
        result.insert(winapi::WM_AFXFIRST + 18, "WM_AFXFIRST + 18");
        result.insert(winapi::WM_AFXFIRST + 19, "WM_AFXFIRST + 19");
        result.insert(winapi::WM_AFXFIRST + 20, "WM_AFXFIRST + 20");
        result.insert(winapi::WM_AFXFIRST + 21, "WM_AFXFIRST + 21");
        result.insert(winapi::WM_AFXFIRST + 22, "WM_AFXFIRST + 22");
        result.insert(winapi::WM_AFXFIRST + 23, "WM_AFXFIRST + 23");
        result.insert(winapi::WM_AFXFIRST + 24, "WM_AFXFIRST + 24");
        result.insert(winapi::WM_AFXFIRST + 25, "WM_AFXFIRST + 25");
        result.insert(winapi::WM_AFXFIRST + 26, "WM_AFXFIRST + 26");
        result.insert(winapi::WM_AFXFIRST + 27, "WM_AFXFIRST + 27");
        result.insert(winapi::WM_AFXFIRST + 28, "WM_AFXFIRST + 28");
        result.insert(winapi::WM_AFXFIRST + 29, "WM_AFXFIRST + 29");
        result.insert(winapi::WM_AFXFIRST + 30, "WM_AFXFIRST + 30");
        result.insert(winapi::WM_AFXLAST, "WM_AFXLAST");
        result.insert(winapi::WM_PENWINFIRST, "WM_PENWINFIRST");
        result.insert(winapi::WM_PENWINFIRST + 1, "WM_PENWINFIRST + 1");
        result.insert(winapi::WM_PENWINFIRST + 2, "WM_PENWINFIRST + 2");
        result.insert(winapi::WM_PENWINFIRST + 3, "WM_PENWINFIRST + 3");
        result.insert(winapi::WM_PENWINFIRST + 4, "WM_PENWINFIRST + 4");
        result.insert(winapi::WM_PENWINFIRST + 5, "WM_PENWINFIRST + 5");
        result.insert(winapi::WM_PENWINFIRST + 6, "WM_PENWINFIRST + 6");
        result.insert(winapi::WM_PENWINFIRST + 7, "WM_PENWINFIRST + 7");
        result.insert(winapi::WM_PENWINFIRST + 8, "WM_PENWINFIRST + 8");
        result.insert(winapi::WM_PENWINFIRST + 9, "WM_PENWINFIRST + 9");
        result.insert(winapi::WM_PENWINFIRST + 10, "WM_PENWINFIRST + 10");
        result.insert(winapi::WM_PENWINFIRST + 11, "WM_PENWINFIRST + 11");
        result.insert(winapi::WM_PENWINFIRST + 12, "WM_PENWINFIRST + 12");
        result.insert(winapi::WM_PENWINFIRST + 13, "WM_PENWINFIRST + 13");
        result.insert(winapi::WM_PENWINFIRST + 14, "WM_PENWINFIRST + 14");
        result.insert(winapi::WM_PENWINLAST, "WM_PENWINLAST");
        result.insert(winapi::WM_USER, "WM_USER");
        result.insert(winapi::WM_APP, "WM_APP");
        result.insert(winapi::WM_DWMNCRENDERINGCHANGED, "WM_DWMNCRENDERINGCHANGED");
        result
    };
}

#[cfg(not(debug_assertions))]
pub fn wui_abort(msg: &str, title: Option<&str>) -> ! {
    let _ = writeln!(io::stderr(), "{}", msg);
    let _ = dialog::message_box(None, msg, title, Some(0x10));
    process::exit(1);
}

#[cfg(debug_assertions)]
pub fn wui_abort(msg: &str, title: Option<&str>) -> ! {
    unsafe {
        let _ = writeln!(io::stderr(), "{}", msg);
        let _ = title;
        kernel32::DebugBreak();
        process::exit(1);
    }
}