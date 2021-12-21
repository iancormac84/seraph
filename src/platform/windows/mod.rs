use std::io::Write;
use std::{io, process};

pub mod application;
pub mod cursor;
pub mod dialog;
#[macro_use]
pub mod macros;
pub mod utils;
pub mod window;
pub mod xinputinterface;

/*static WINDOWS_MESSAGE_STRINGS: Lazy<Mutex<BTreeMap<u32, &'static str>>> = Lazy::new(|| {
    let mut result: BTreeMap<u32, &'static str> = BTreeMap::new();
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_NULL, "WM_NULL");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CREATE,
        "WM_CREATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DESTROY,
        "WM_DESTROY",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_MOVE, "WM_MOVE");
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_SIZE, "WM_SIZE");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ACTIVATE,
        "WM_ACTIVATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETFOCUS,
        "WM_SETFOCUS",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_KILLFOCUS,
        "WM_KILLFOCUS",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ENABLE,
        "WM_ENABLE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETREDRAW,
        "WM_SETREDRAW",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETTEXT,
        "WM_SETTEXT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETTEXT,
        "WM_GETTEXT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETTEXTLENGTH,
        "WM_GETTEXTLENGTH",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PAINT,
        "WM_PAINT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CLOSE,
        "WM_CLOSE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_QUERYENDSESSION,
        "WM_QUERYENDSESSION",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_QUERYOPEN,
        "WM_QUERYOPEN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ENDSESSION,
        "WM_ENDSESSION",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_QUIT, "WM_QUIT");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ERASEBKGND,
        "WM_ERASEBKGND",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYSCOLORCHANGE,
        "WM_SYSCOLORCHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SHOWWINDOW,
        "WM_SHOWWINDOW",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_WININICHANGE,
        "WM_WININICHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DEVMODECHANGE,
        "WM_DEVMODECHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ACTIVATEAPP,
        "WM_ACTIVATEAPP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_FONTCHANGE,
        "WM_FONTCHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TIMECHANGE,
        "WM_TIMECHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CANCELMODE,
        "WM_CANCELMODE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETCURSOR,
        "WM_SETCURSOR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOUSEACTIVATE,
        "WM_MOUSEACTIVATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CHILDACTIVATE,
        "WM_CHILDACTIVATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_QUEUESYNC,
        "WM_QUEUESYNC",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETMINMAXINFO,
        "WM_GETMINMAXINFO",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PAINTICON,
        "WM_PAINTICON",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ICONERASEBKGND,
        "WM_ICONERASEBKGND",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NEXTDLGCTL,
        "WM_NEXTDLGCTL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SPOOLERSTATUS,
        "WM_SPOOLERSTATUS",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DRAWITEM,
        "WM_DRAWITEM",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MEASUREITEM,
        "WM_MEASUREITEM",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DELETEITEM,
        "WM_DELETEITEM",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_VKEYTOITEM,
        "WM_VKEYTOITEM",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CHARTOITEM,
        "WM_CHARTOITEM",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETFONT,
        "WM_SETFONT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETFONT,
        "WM_GETFONT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETHOTKEY,
        "WM_SETHOTKEY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETHOTKEY,
        "WM_GETHOTKEY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_QUERYDRAGICON,
        "WM_QUERYDRAGICON",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_COMPAREITEM,
        "WM_COMPAREITEM",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETOBJECT,
        "WM_GETOBJECT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_COMPACTING,
        "WM_COMPACTING",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_COMMNOTIFY,
        "WM_COMMNOTIFY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_WINDOWPOSCHANGING,
        "WM_WINDOWPOSCHANGING",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_WINDOWPOSCHANGED,
        "WM_WINDOWPOSCHANGED",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_POWER,
        "WM_POWER",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_COPYDATA,
        "WM_COPYDATA",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CANCELJOURNAL,
        "WM_CANCELJOURNAL",
    );
    result.insert(
        windows::Win32::UI::Controls::RichEdit::WM_NOTIFY,
        "WM_NOTIFY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_INPUTLANGCHANGEREQUEST,
        "WM_INPUTLANGCHANGEREQUEST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_INPUTLANGCHANGE,
        "WM_INPUTLANGCHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TCARD,
        "WM_TCARD",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_HELP, "WM_HELP");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_USERCHANGED,
        "WM_USERCHANGED",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NOTIFYFORMAT,
        "WM_NOTIFYFORMAT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CONTEXTMENU,
        "WM_CONTEXTMENU",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_STYLECHANGING,
        "WM_STYLECHANGING",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_STYLECHANGED,
        "WM_STYLECHANGED",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DISPLAYCHANGE,
        "WM_DISPLAYCHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETICON,
        "WM_GETICON",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SETICON,
        "WM_SETICON",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE,
        "WM_NCCREATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCDESTROY,
        "WM_NCDESTROY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCCALCSIZE,
        "WM_NCCALCSIZE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCHITTEST,
        "WM_NCHITTEST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCPAINT,
        "WM_NCPAINT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCACTIVATE,
        "WM_NCACTIVATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_GETDLGCODE,
        "WM_GETDLGCODE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYNCPAINT,
        "WM_SYNCPAINT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCMOUSEMOVE,
        "WM_NCMOUSEMOVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONDOWN,
        "WM_NCLBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONUP,
        "WM_NCLBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCLBUTTONDBLCLK,
        "WM_NCLBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCRBUTTONDOWN,
        "WM_NCRBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCRBUTTONUP,
        "WM_NCRBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCRBUTTONDBLCLK,
        "WM_NCRBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCMBUTTONDOWN,
        "WM_NCMBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCMBUTTONUP,
        "WM_NCMBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCMBUTTONDBLCLK,
        "WM_NCMBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCXBUTTONDOWN,
        "WM_NCXBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCXBUTTONUP,
        "WM_NCXBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCXBUTTONDBLCLK,
        "WM_NCXBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_INPUT,
        "WM_INPUT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN,
        "WM_KEYDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_KEYUP,
        "WM_KEYUP",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_CHAR, "WM_CHAR");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DEADCHAR,
        "WM_DEADCHAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYSKEYDOWN,
        "WM_SYSKEYDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYSKEYUP,
        "WM_SYSKEYUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYSCHAR,
        "WM_SYSCHAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYSDEADCHAR,
        "WM_SYSDEADCHAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_UNICHAR,
        "WM_UNICHAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_STARTCOMPOSITION,
        "WM_IME_STARTCOMPOSITION",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_ENDCOMPOSITION,
        "WM_IME_ENDCOMPOSITION",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_COMPOSITION,
        "WM_IME_COMPOSITION",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_INITDIALOG,
        "WM_INITDIALOG",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_COMMAND,
        "WM_COMMAND",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SYSCOMMAND,
        "WM_SYSCOMMAND",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TIMER,
        "WM_TIMER",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HSCROLL,
        "WM_HSCROLL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_VSCROLL,
        "WM_VSCROLL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_INITMENU,
        "WM_INITMENU",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_INITMENUPOPUP,
        "WM_INITMENUPOPUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MENUSELECT,
        "WM_MENUSELECT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MENUCHAR,
        "WM_MENUCHAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ENTERIDLE,
        "WM_ENTERIDLE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MENURBUTTONUP,
        "WM_MENURBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MENUDRAG,
        "WM_MENUDRAG",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MENUGETOBJECT,
        "WM_MENUGETOBJECT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_UNINITMENUPOPUP,
        "WM_UNINITMENUPOPUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MENUCOMMAND,
        "WM_MENUCOMMAND",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CHANGEUISTATE,
        "WM_CHANGEUISTATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_UPDATEUISTATE,
        "WM_UPDATEUISTATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_QUERYUISTATE,
        "WM_QUERYUISTATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLORMSGBOX,
        "WM_CTLCOLORMSGBOX",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLOREDIT,
        "WM_CTLCOLOREDIT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLORLISTBOX,
        "WM_CTLCOLORLISTBOX",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLORBTN,
        "WM_CTLCOLORBTN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLORDLG,
        "WM_CTLCOLORDLG",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLORSCROLLBAR,
        "WM_CTLCOLORSCROLLBAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CTLCOLORSTATIC,
        "WM_CTLCOLORSTATIC",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOUSEMOVE,
        "WM_MOUSEMOVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN,
        "WM_LBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONUP,
        "WM_LBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDBLCLK,
        "WM_LBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_RBUTTONDOWN,
        "WM_RBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_RBUTTONUP,
        "WM_RBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_RBUTTONDBLCLK,
        "WM_RBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MBUTTONDOWN,
        "WM_MBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MBUTTONUP,
        "WM_MBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MBUTTONDBLCLK,
        "WM_MBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOUSEWHEEL,
        "WM_MOUSEWHEEL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_XBUTTONDOWN,
        "WM_XBUTTONDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_XBUTTONUP,
        "WM_XBUTTONUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_XBUTTONDBLCLK,
        "WM_XBUTTONDBLCLK",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOUSEHWHEEL,
        "WM_MOUSEHWHEEL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PARENTNOTIFY,
        "WM_PARENTNOTIFY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ENTERMENULOOP,
        "WM_ENTERMENULOOP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_EXITMENULOOP,
        "WM_EXITMENULOOP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NEXTMENU,
        "WM_NEXTMENU",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SIZING,
        "WM_SIZING",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CAPTURECHANGED,
        "WM_CAPTURECHANGED",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOVING,
        "WM_MOVING",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_POWERBROADCAST,
        "WM_POWERBROADCAST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DEVICECHANGE,
        "WM_DEVICECHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDICREATE,
        "WM_MDICREATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIDESTROY,
        "WM_MDIDESTROY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIACTIVATE,
        "WM_MDIACTIVATE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIRESTORE,
        "WM_MDIRESTORE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDINEXT,
        "WM_MDINEXT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIMAXIMIZE,
        "WM_MDIMAXIMIZE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDITILE,
        "WM_MDITILE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDICASCADE,
        "WM_MDICASCADE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIICONARRANGE,
        "WM_MDIICONARRANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIGETACTIVE,
        "WM_MDIGETACTIVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDISETMENU,
        "WM_MDISETMENU",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ENTERSIZEMOVE,
        "WM_ENTERSIZEMOVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_EXITSIZEMOVE,
        "WM_EXITSIZEMOVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DROPFILES,
        "WM_DROPFILES",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MDIREFRESHMENU,
        "WM_MDIREFRESHMENU",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_SETCONTEXT,
        "WM_IME_SETCONTEXT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_NOTIFY,
        "WM_IME_NOTIFY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_CONTROL,
        "WM_IME_CONTROL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_COMPOSITIONFULL,
        "WM_IME_COMPOSITIONFULL",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_SELECT,
        "WM_IME_SELECT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_CHAR,
        "WM_IME_CHAR",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_REQUEST,
        "WM_IME_REQUEST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_KEYDOWN,
        "WM_IME_KEYDOWN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_IME_KEYUP,
        "WM_IME_KEYUP",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCMOUSEHOVER,
        "WM_NCMOUSEHOVER",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOUSEHOVER,
        "WM_MOUSEHOVER",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_NCMOUSELEAVE,
        "WM_NCMOUSELEAVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_MOUSELEAVE,
        "WM_MOUSELEAVE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_WTSSESSION_CHANGE,
        "WM_WTSSESSION_CHANGE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST,
        "WM_TABLET_FIRST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 1,
        "WM_TABLET_FIRST + 1",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 2,
        "WM_TABLET_FIRST + 2",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 3,
        "WM_TABLET_FIRST + 3",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 4,
        "WM_TABLET_FIRST + 4",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 5,
        "WM_TABLET_FIRST + 5",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 6,
        "WM_TABLET_FIRST + 6",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 7,
        "WM_TABLET_FIRST + 7",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 8,
        "WM_TABLET_FIRST + 8",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 9,
        "WM_TABLET_FIRST + 9",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 10,
        "WM_TABLET_FIRST + 10",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 11,
        "WM_TABLET_FIRST + 11",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 12,
        "WM_TABLET_FIRST + 12",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 13,
        "WM_TABLET_FIRST + 13",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 14,
        "WM_TABLET_FIRST + 14",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 15,
        "WM_TABLET_FIRST + 15",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 16,
        "WM_TABLET_FIRST + 16",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 17,
        "WM_TABLET_FIRST + 17",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 18,
        "WM_TABLET_FIRST + 18",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 19,
        "WM_TABLET_FIRST + 19",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 20,
        "WM_TABLET_FIRST + 20",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 21,
        "WM_TABLET_FIRST + 21",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 22,
        "WM_TABLET_FIRST + 22",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 23,
        "WM_TABLET_FIRST + 23",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 24,
        "WM_TABLET_FIRST + 24",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 25,
        "WM_TABLET_FIRST + 25",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 26,
        "WM_TABLET_FIRST + 26",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 27,
        "WM_TABLET_FIRST + 27",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 28,
        "WM_TABLET_FIRST + 28",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 29,
        "WM_TABLET_FIRST + 29",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_FIRST + 30,
        "WM_TABLET_FIRST + 30",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_TABLET_LAST,
        "WM_TABLET_LAST",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_CUT, "WM_CUT");
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_COPY, "WM_COPY");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PASTE,
        "WM_PASTE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CLEAR,
        "WM_CLEAR",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_UNDO, "WM_UNDO");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_RENDERFORMAT,
        "WM_RENDERFORMAT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_RENDERALLFORMATS,
        "WM_RENDERALLFORMATS",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DESTROYCLIPBOARD,
        "WM_DESTROYCLIPBOARD",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DRAWCLIPBOARD,
        "WM_DRAWCLIPBOARD",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PAINTCLIPBOARD,
        "WM_PAINTCLIPBOARD",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_VSCROLLCLIPBOARD,
        "WM_VSCROLLCLIPBOARD",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_SIZECLIPBOARD,
        "WM_SIZECLIPBOARD",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_ASKCBFORMATNAME,
        "WM_ASKCBFORMATNAME",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_CHANGECBCHAIN,
        "WM_CHANGECBCHAIN",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HSCROLLCLIPBOARD,
        "WM_HSCROLLCLIPBOARD",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_QUERYNEWPALETTE,
        "WM_QUERYNEWPALETTE",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PALETTEISCHANGING,
        "WM_PALETTEISCHANGING",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PALETTECHANGED,
        "WM_PALETTECHANGED",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY,
        "WM_HOTKEY",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PRINT,
        "WM_PRINT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PRINTCLIENT,
        "WM_PRINTCLIENT",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_APPCOMMAND,
        "WM_APPCOMMAND",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_THEMECHANGED,
        "WM_THEMECHANGED",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST,
        "WM_HANDHELDFIRST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST + 1,
        "WM_HANDHELDFIRST + 1",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST + 2,
        "WM_HANDHELDFIRST + 2",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST + 3,
        "WM_HANDHELDFIRST + 3",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST + 4,
        "WM_HANDHELDFIRST + 4",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST + 5,
        "WM_HANDHELDFIRST + 5",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDFIRST + 6,
        "WM_HANDHELDFIRST + 6",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_HANDHELDLAST,
        "WM_HANDHELDLAST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST,
        "WM_AFXFIRST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 1,
        "WM_AFXFIRST + 1",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 2,
        "WM_AFXFIRST + 2",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 3,
        "WM_AFXFIRST + 3",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 4,
        "WM_AFXFIRST + 4",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 5,
        "WM_AFXFIRST + 5",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 6,
        "WM_AFXFIRST + 6",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 7,
        "WM_AFXFIRST + 7",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 8,
        "WM_AFXFIRST + 8",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 9,
        "WM_AFXFIRST + 9",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 10,
        "WM_AFXFIRST + 10",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 11,
        "WM_AFXFIRST + 11",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 12,
        "WM_AFXFIRST + 12",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 13,
        "WM_AFXFIRST + 13",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 14,
        "WM_AFXFIRST + 14",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 15,
        "WM_AFXFIRST + 15",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 16,
        "WM_AFXFIRST + 16",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 17,
        "WM_AFXFIRST + 17",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 18,
        "WM_AFXFIRST + 18",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 19,
        "WM_AFXFIRST + 19",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 20,
        "WM_AFXFIRST + 20",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 21,
        "WM_AFXFIRST + 21",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 22,
        "WM_AFXFIRST + 22",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 23,
        "WM_AFXFIRST + 23",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 24,
        "WM_AFXFIRST + 24",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 25,
        "WM_AFXFIRST + 25",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 26,
        "WM_AFXFIRST + 26",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 27,
        "WM_AFXFIRST + 27",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 28,
        "WM_AFXFIRST + 28",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 29,
        "WM_AFXFIRST + 29",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXFIRST + 30,
        "WM_AFXFIRST + 30",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_AFXLAST,
        "WM_AFXLAST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST,
        "WM_PENWINFIRST",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 1,
        "WM_PENWINFIRST + 1",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 2,
        "WM_PENWINFIRST + 2",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 3,
        "WM_PENWINFIRST + 3",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 4,
        "WM_PENWINFIRST + 4",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 5,
        "WM_PENWINFIRST + 5",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 6,
        "WM_PENWINFIRST + 6",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 7,
        "WM_PENWINFIRST + 7",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 8,
        "WM_PENWINFIRST + 8",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 9,
        "WM_PENWINFIRST + 9",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 10,
        "WM_PENWINFIRST + 10",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 11,
        "WM_PENWINFIRST + 11",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 12,
        "WM_PENWINFIRST + 12",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 13,
        "WM_PENWINFIRST + 13",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINFIRST + 14,
        "WM_PENWINFIRST + 14",
    );
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_PENWINLAST,
        "WM_PENWINLAST",
    );
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_USER, "WM_USER");
    result.insert(windows::Win32::UI::WindowsAndMessaging::WM_APP, "WM_APP");
    result.insert(
        windows::Win32::UI::WindowsAndMessaging::WM_DWMNCRENDERINGCHANGED,
        "WM_DWMNCRENDERINGCHANGED",
    );
    Mutex::new(result)
});*/

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
        windows::Win32::System::Diagnostics::Debug::DebugBreak();
        process::exit(1);
    }
}
