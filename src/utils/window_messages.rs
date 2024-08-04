#![allow(dead_code)]

use windows::Win32::UI::WindowsAndMessaging::*;

/// Converts a Windows message ID to its corresponding string representation.
/// This function maps known message IDs to their string names, and returns "UNKNOWN_MESSAGE" for unknown message IDs.
pub fn message_to_string(msg: u32) -> &'static str {
    match msg {
        // Sent when an application requests that a window be created.
        WM_CREATE => "WM_CREATE",
        // Sent when a window is being destroyed.
        WM_DESTROY => "WM_DESTROY",
        // Sent after a window has been moved.
        WM_MOVE => "WM_MOVE",
        // Sent after a window has been resized.
        WM_SIZE => "WM_SIZE",
        // Sent when a window is being activated or deactivated.
        WM_ACTIVATE => "WM_ACTIVATE",
        // Sent when a window has gained the keyboard focus.
        WM_SETFOCUS => "WM_SETFOCUS",
        // Sent when a window has lost the keyboard focus.
        WM_KILLFOCUS => "WM_KILLFOCUS",
        // Sent when an application changes the enabled state of a window.
        WM_ENABLE => "WM_ENABLE",
        // Sent to a window to allow changes in that window to be redrawn or to prevent changes in that window from being redrawn.
        WM_SETREDRAW => "WM_SETREDRAW",
        // Sent to set the text of a window.
        WM_SETTEXT => "WM_SETTEXT",
        // Sent to copy the text that corresponds to a window into a buffer provided by the caller.
        WM_GETTEXT => "WM_GETTEXT",
        // Sent to determine the length, in characters, of the text associated with a window.
        WM_GETTEXTLENGTH => "WM_GETTEXTLENGTH",
        // Sent when the system or another application makes a request to paint a portion of an application's window.
        WM_PAINT => "WM_PAINT",
        // Sent as a signal that a window or an application should terminate.
        WM_CLOSE => "WM_CLOSE",
        // Sent when the user chooses to end the session or when an application calls ExitWindows function.
        WM_QUERYENDSESSION => "WM_QUERYENDSESSION",
        // Indicates a request to terminate an application and is generated when the application calls the PostQuitMessage function.
        WM_QUIT => "WM_QUIT",
        // Sent to an icon when the user requests that the window be restored to its previous size and position.
        WM_QUERYOPEN => "WM_QUERYOPEN",
        // Sent when the window background must be erased (for example, when a window is resized).
        WM_ERASEBKGND => "WM_ERASEBKGND",
        // Sent to all top-level windows when a change is made to a system color setting.
        WM_SYSCOLORCHANGE => "WM_SYSCOLORCHANGE",
        // Sent to a window when the window is about to be shown or hidden.
        WM_SHOWWINDOW => "WM_SHOWWINDOW",
        // Sent to all top-level windows after a change is made to a system parameter.
        WM_WININICHANGE => "WM_WININICHANGE",
        // Sent to all top-level windows whenever the user changes device-mode settings.
        WM_DEVMODECHANGE => "WM_DEVMODECHANGE",
        // Sent when a window belonging to a different application than the active window is about to be activated.
        WM_ACTIVATEAPP => "WM_ACTIVATEAPP",
        // Sent to all top-level windows when the font resource changes.
        WM_FONTCHANGE => "WM_FONTCHANGE",
        // Sent to all top-level windows after a change in the system time.
        WM_TIMECHANGE => "WM_TIMECHANGE",
        // Sent to cancel certain modes, such as mouse capture.
        WM_CANCELMODE => "WM_CANCELMODE",
        // Sent if the mouse causes the cursor to move within a window and mouse input is not captured.
        WM_SETCURSOR => "WM_SETCURSOR",
        // Sent when a mouse button is pressed while the cursor is in the client area of a window.
        WM_MOUSEACTIVATE => "WM_MOUSEACTIVATE",
        // Sent to a child window when the user clicks the window's title bar or when the window is activated.
        WM_CHILDACTIVATE => "WM_CHILDACTIVATE",
        // Sent to synchronize the contents of a window.
        WM_QUEUESYNC => "WM_QUEUESYNC",
        // Sent to a window when the size or position of the window is about to change.
        WM_GETMINMAXINFO => "WM_GETMINMAXINFO",
        // Sent to a window when the icon is about to be drawn.
        WM_PAINTICON => "WM_PAINTICON",
        // Sent to a window when the background of the icon is about to be erased.
        WM_ICONERASEBKGND => "WM_ICONERASEBKGND",
        // Sent to a window to change focus to the next control in a dialog box.
        WM_NEXTDLGCTL => "WM_NEXTDLGCTL",
        // Sent to a window when the spooler status changes.
        WM_SPOOLERSTATUS => "WM_SPOOLERSTATUS",
        // Sent to a window when the items in a list box or combo box need to be drawn.
        WM_DRAWITEM => "WM_DRAWITEM",
        // Sent to a window when the items in a list box or combo box need to be measured.
        WM_MEASUREITEM => "WM_MEASUREITEM",
        // Sent to a window when an item is deleted from a list box or combo box.
        WM_DELETEITEM => "WM_DELETEITEM",
        // Sent to a window when a virtual key is pressed in a list box or combo box.
        WM_VKEYTOITEM => "WM_VKEYTOITEM",
        // Sent to a window when a character is typed in a list box or combo box.
        WM_CHARTOITEM => "WM_CHARTOITEM",
        // Sent to a window to change the font.
        WM_SETFONT => "WM_SETFONT",
        // Sent to a window to retrieve the font.
        WM_GETFONT => "WM_GETFONT",
        // Sent to a window to set a hot key.
        WM_SETHOTKEY => "WM_SETHOTKEY",
        // Sent to a window to retrieve the hot key.
        WM_GETHOTKEY => "WM_GETHOTKEY",
        // Sent to a window to query whether it is to be treated as a drag-and-drop source.
        WM_QUERYDRAGICON => "WM_QUERYDRAGICON",
        // Sent to a window to determine the relative position of a new item.
        WM_COMPAREITEM => "WM_COMPAREITEM",
        // Sent to a window when memory is low.
        WM_COMPACTING => "WM_COMPACTING",
        // Sent to a window when the communication status changes.
        WM_COMMNOTIFY => "WM_COMMNOTIFY",
        // Sent to a window when its position or size is about to change.
        WM_WINDOWPOSCHANGING => "WM_WINDOWPOSCHANGING",
        // Sent to a window when its position or size has changed.
        WM_WINDOWPOSCHANGED => "WM_WINDOWPOSCHANGED",
        // Sent to a window when the power status changes.
        WM_POWER => "WM_POWER",
        // Sent to a window when data is being copied.
        WM_COPYDATA => "WM_COPYDATA",
        // Sent to a window to cancel a journaling operation.
        WM_CANCELJOURNAL => "WM_CANCELJOURNAL",
        // Sent to a window when the control needs to be notified.
        WM_NOTIFY => "WM_NOTIFY",
        // Sent to a window when the input language changes.
        WM_INPUTLANGCHANGEREQUEST => "WM_INPUTLANGCHANGEREQUEST",
        // Sent to a window when the input language has changed.
        WM_INPUTLANGCHANGE => "WM_INPUTLANGCHANGE",
        // Sent to a window to request a task card.
        WM_TCARD => "WM_TCARD",
        // Sent to a window to request help.
        WM_HELP => "WM_HELP",
        // Sent to a window when the user changes the user settings.
        WM_USERCHANGED => "WM_USERCHANGED",
        // Sent to a window to request the format of a notification.
        WM_NOTIFYFORMAT => "WM_NOTIFYFORMAT",
        // Sent to a window when the context menu is about to be displayed.
        WM_CONTEXTMENU => "WM_CONTEXTMENU",
        // Sent to a window when the style is about to change.
        WM_STYLECHANGING => "WM_STYLECHANGING",
        // Sent to a window when the style has changed.
        WM_STYLECHANGED => "WM_STYLECHANGED",
        // Sent to a window when the display resolution has changed.
        WM_DISPLAYCHANGE => "WM_DISPLAYCHANGE",
        // Sent to a window to retrieve the icon.
        WM_GETICON => "WM_GETICON",
        // Sent to a window to set the icon.
        WM_SETICON => "WM_SETICON",
        // Sent to a window when it is being created.
        WM_NCCREATE => "WM_NCCREATE",
        // Sent to a window when it is being destroyed.
        WM_NCDESTROY => "WM_NCDESTROY",
        // Sent to a window to calculate the size and position of the client area.
        WM_NCCALCSIZE => "WM_NCCALCSIZE",
        // Sent to a window to determine the position of the cursor.
        WM_NCHITTEST => "WM_NCHITTEST",
        // Sent to a window to paint the non-client area.
        WM_NCPAINT => "WM_NCPAINT",
        // Sent to a window to activate the non-client area.
        WM_NCACTIVATE => "WM_NCACTIVATE",
        // Sent to a window to retrieve the dialog code.
        WM_GETDLGCODE => "WM_GETDLGCODE",
        // Sent to synchronize the painting of a window.
        WM_SYNCPAINT => "WM_SYNCPAINT",
        // Sent to a window when the mouse moves over the non-client area.
        WM_NCMOUSEMOVE => "WM_NCMOUSEMOVE",
        // Sent to a window when the left mouse button is pressed over the non-client area.
        WM_NCLBUTTONDOWN => "WM_NCLBUTTONDOWN",
        // Sent to a window when the left mouse button is released over the non-client area.
        WM_NCLBUTTONUP => "WM_NCLBUTTONUP",
        // Sent to a window when the left mouse button is double-clicked over the non-client area.
        WM_NCLBUTTONDBLCLK => "WM_NCLBUTTONDBLCLK",
        // Sent to a window when the right mouse button is pressed over the non-client area.
        WM_NCRBUTTONDOWN => "WM_NCRBUTTONDOWN",
        // Sent to a window when the right mouse button is released over the non-client area.
        WM_NCRBUTTONUP => "WM_NCRBUTTONUP",
        // Sent to a window when the right mouse button is double-clicked over the non-client area.
        WM_NCRBUTTONDBLCLK => "WM_NCRBUTTONDBLCLK",
        // Sent to a window when the middle mouse button is pressed over the non-client area.
        WM_NCMBUTTONDOWN => "WM_NCMBUTTONDOWN",
        // Sent to a window when the middle mouse button is released over the non-client area.
        WM_NCMBUTTONUP => "WM_NCMBUTTONUP",
        // Sent to a window when the middle mouse button is double-clicked over the non-client area.
        WM_NCMBUTTONDBLCLK => "WM_NCMBUTTONDBLCLK",
        // Sent to a window when a key is pressed.
        WM_KEYDOWN => "WM_KEYDOWN",
        // Sent to a window when a key is released.
        WM_KEYUP => "WM_KEYUP",
        // Sent to a window when a character is typed.
        WM_CHAR => "WM_CHAR",
        // Sent to a window when a dead character is typed.
        WM_DEADCHAR => "WM_DEADCHAR",
        // Sent to a window when a system key is pressed.
        WM_SYSKEYDOWN => "WM_SYSKEYDOWN",
        // Sent to a window when a system key is released.
        WM_SYSKEYUP => "WM_SYSKEYUP",
        // Sent to a window when a system character is typed.
        WM_SYSCHAR => "WM_SYSCHAR",
        // Sent to a window when a system dead character is typed.
        WM_SYSDEADCHAR => "WM_SYSDEADCHAR",
        // Sent to a window after a keystroke message is posted to the window.
        WM_KEYLAST => "WM_KEYLAST",
        // Sent to a window when an input method editor (IME) starts composition.
        WM_IME_STARTCOMPOSITION => "WM_IME_STARTCOMPOSITION",
        // Sent to a window when an input method editor (IME) ends composition.
        WM_IME_ENDCOMPOSITION => "WM_IME_ENDCOMPOSITION",
        // Sent to a window when an input method editor (IME) composes.
        WM_IME_COMPOSITION => "WM_IME_COMPOSITION",
        // Sent to a window when a dialog box is initialized.
        WM_INITDIALOG => "WM_INITDIALOG",
        // Sent to a window when a command is sent from a menu or control.
        WM_COMMAND => "WM_COMMAND",
        // Sent to a window when a system command is sent.
        WM_SYSCOMMAND => "WM_SYSCOMMAND",
        // Sent to a window when a timer expires.
        WM_TIMER => "WM_TIMER",
        // Sent to a window when a scroll bar is scrolled.
        WM_HSCROLL => "WM_HSCROLL",
        // Sent to a window when a scroll bar is scrolled vertically.
        WM_VSCROLL => "WM_VSCROLL",
        // Sent to a window when a menu is initialized.
        WM_INITMENU => "WM_INITMENU",
        // Sent to a window when a popup menu is initialized.
        WM_INITMENUPOPUP => "WM_INITMENUPOPUP",
        // Sent to a window when a menu item is selected.
        WM_MENUSELECT => "WM_MENUSELECT",
        // Sent to a window when a menu character is typed.
        WM_MENUCHAR => "WM_MENUCHAR",
        // Sent to a window when the user is idle in a menu.
        WM_ENTERIDLE => "WM_ENTERIDLE",
        // Sent to a window to get the color of a message box.
        WM_CTLCOLORMSGBOX => "WM_CTLCOLORMSGBOX",
        // Sent to a window to get the color of an edit control.
        WM_CTLCOLOREDIT => "WM_CTLCOLOREDIT",
        // Sent to a window to get the color of a list box.
        WM_CTLCOLORLISTBOX => "WM_CTLCOLORLISTBOX",
        // Sent to a window to get the color of a button.
        WM_CTLCOLORBTN => "WM_CTLCOLORBTN",
        // Sent to a window to get the color of a dialog box.
        WM_CTLCOLORDLG => "WM_CTLCOLORDLG",
        // Sent to a window to get the color of a scroll bar.
        WM_CTLCOLORSCROLLBAR => "WM_CTLCOLORSCROLLBAR",
        // Sent to a window to get the color of a static control.
        WM_CTLCOLORSTATIC => "WM_CTLCOLORSTATIC",
        // Sent to a window when the mouse moves.
        WM_MOUSEMOVE => "WM_MOUSEMOVE",
        // Sent to a window when the left mouse button is pressed.
        WM_LBUTTONDOWN => "WM_LBUTTONDOWN",
        // Sent to a window when the left mouse button is released.
        WM_LBUTTONUP => "WM_LBUTTONUP",
        // Sent to a window when the left mouse button is double-clicked.
        WM_LBUTTONDBLCLK => "WM_LBUTTONDBLCLK",
        // Sent to a window when the right mouse button is pressed.
        WM_RBUTTONDOWN => "WM_RBUTTONDOWN",
        // Sent to a window when the right mouse button is released.
        WM_RBUTTONUP => "WM_RBUTTONUP",
        // Sent to a window when the right mouse button is double-clicked.
        WM_RBUTTONDBLCLK => "WM_RBUTTONDBLCLK",
        // Sent to a window when the middle mouse button is pressed.
        WM_MBUTTONDOWN => "WM_MBUTTONDOWN",
        // Sent to a window when the middle mouse button is released.
        WM_MBUTTONUP => "WM_MBUTTONUP",
        // Sent to a window when the middle mouse button is double-clicked.
        WM_MBUTTONDBLCLK => "WM_MBUTTONDBLCLK",
        // Sent to a window when a parent control is notified.
        WM_PARENTNOTIFY => "WM_PARENTNOTIFY",
        // Sent to a window when the user enters the menu loop.
        WM_ENTERMENULOOP => "WM_ENTERMENULOOP",
        // Sent to a window when the user exits the menu loop.
        WM_EXITMENULOOP => "WM_EXITMENULOOP",
        // Sent to a window when the next menu item is selected.
        WM_NEXTMENU => "WM_NEXTMENU",
        // Sent to a window when it is being resized.
        WM_SIZING => "WM_SIZING",
        // Sent to a window when the capture has changed.
        WM_CAPTURECHANGED => "WM_CAPTURECHANGED",
        // Sent to a window when it is being moved.
        WM_MOVING => "WM_MOVING",
        // Sent to a window when a power broadcast is sent.
        WM_POWERBROADCAST => "WM_POWERBROADCAST",
        // Sent to a window when a device change occurs.
        WM_DEVICECHANGE => "WM_DEVICECHANGE",
        // Sent to a window when a multiple document interface (MDI) child window is created.
        WM_MDICREATE => "WM_MDICREATE",
        // Sent to a window when a multiple document interface (MDI) child window is destroyed.
        WM_MDIDESTROY => "WM_MDIDESTROY",
        // Sent to a window when a multiple document interface (MDI) child window is activated.
        WM_MDIACTIVATE => "WM_MDIACTIVATE",
        // Sent to a window when a multiple document interface (MDI) child window is restored.
        WM_MDIRESTORE => "WM_MDIRESTORE",
        // Sent to a window when the next multiple document interface (MDI) child window is selected.
        WM_MDINEXT => "WM_MDINEXT",
        // Sent to a window when a multiple document interface (MDI) child window is maximized.
        WM_MDIMAXIMIZE => "WM_MDIMAXIMIZE",
        // Sent to a window to tile multiple document interface (MDI) child windows.
        WM_MDITILE => "WM_MDITILE",
        // Sent to a window to cascade multiple document interface (MDI) child windows.
        WM_MDICASCADE => "WM_MDICASCADE",
        // Sent to a window to arrange icons for multiple document interface (MDI) child windows.
        WM_MDIICONARRANGE => "WM_MDIICONARRANGE",
        // Sent to a window to get the active multiple document interface (MDI) child window.
        WM_MDIGETACTIVE => "WM_MDIGETACTIVE",
        // Sent to a window to set the menu for a multiple document interface (MDI) child window.
        WM_MDISETMENU => "WM_MDISETMENU",
        // Sent to a window when the user begins to move or size the window.
        WM_ENTERSIZEMOVE => "WM_ENTERSIZEMOVE",
        // Sent to a window when the user finishes moving or sizing the window.
        WM_EXITSIZEMOVE => "WM_EXITSIZEMOVE",
        // Sent to a window when files are dropped.
        WM_DROPFILES => "WM_DROPFILES",
        // Sent to a window to refresh the multiple document interface (MDI) menu.
        WM_MDIREFRESHMENU => "WM_MDIREFRESHMENU",
        // Sent to a window to set the context for an input method editor (IME).
        WM_IME_SETCONTEXT => "WM_IME_SETCONTEXT",
        // Sent to a window to notify it of changes to an input method editor (IME).
        WM_IME_NOTIFY => "WM_IME_NOTIFY",
        // Sent to a window to control the input method editor (IME).
        WM_IME_CONTROL => "WM_IME_CONTROL",
        // Sent to a window when the input method editor (IME) composition is full.
        WM_IME_COMPOSITIONFULL => "WM_IME_COMPOSITIONFULL",
        // Sent to a window to select the input method editor (IME).
        WM_IME_SELECT => "WM_IME_SELECT",
        // Sent to a window when a character is typed using the input method editor (IME).
        WM_IME_CHAR => "WM_IME_CHAR",
        // Sent to a window when a key is pressed using the input method editor (IME).
        WM_IME_KEYDOWN => "WM_IME_KEYDOWN",
        // Sent to a window when a key is released using the input method editor (IME).
        WM_IME_KEYUP => "WM_IME_KEYUP",
        // Sent to a window when the mouse hovers over the non-client area.
        WM_NCMOUSEHOVER => "WM_NCMOUSEHOVER",
        // Sent to a window when the mouse leaves the non-client area.
        WM_NCMOUSELEAVE => "WM_NCMOUSELEAVE",
        // Sent to a window when the session changes.
        WM_WTSSESSION_CHANGE => "WM_WTSSESSION_CHANGE",
        // Sent to a window for the first tablet input message.
        WM_TABLET_FIRST => "WM_TABLET_FIRST",
        // Sent to a window for the last tablet input message.
        WM_TABLET_LAST => "WM_TABLET_LAST",
        // Sent to a window when the content is cut to the clipboard.
        WM_CUT => "WM_CUT",
        // Sent to a window when the content is copied to the clipboard.
        WM_COPY => "WM_COPY",
        // Sent to a window when the content is pasted from the clipboard.
        WM_PASTE => "WM_PASTE",
        // Sent to a window when the content is cleared.
        WM_CLEAR => "WM_CLEAR",
        // Sent to a window to undo the last action.
        WM_UNDO => "WM_UNDO",
        // Sent to a window to render the format.
        WM_RENDERFORMAT => "WM_RENDERFORMAT",
        // Sent to a window to render all formats.
        WM_RENDERALLFORMATS => "WM_RENDERALLFORMATS",
        // Sent to a window to destroy the clipboard.
        WM_DESTROYCLIPBOARD => "WM_DESTROYCLIPBOARD",
        // Sent to a window to draw the clipboard.
        WM_DRAWCLIPBOARD => "WM_DRAWCLIPBOARD",
        // Sent to a window to paint the clipboard.
        WM_PAINTCLIPBOARD => "WM_PAINTCLIPBOARD",
        // Sent to a window to scroll the clipboard.
        WM_VSCROLLCLIPBOARD => "WM_VSCROLLCLIPBOARD",
        // Sent to a window to size the clipboard.
        WM_SIZECLIPBOARD => "WM_SIZECLIPBOARD",
        // Sent to a window to ask for the clipboard format name.
        WM_ASKCBFORMATNAME => "WM_ASKCBFORMATNAME",
        // Sent to a window to change the clipboard chain.
        WM_CHANGECBCHAIN => "WM_CHANGECBCHAIN",
        // Sent to a window to scroll the horizontal clipboard.
        WM_HSCROLLCLIPBOARD => "WM_HSCROLLCLIPBOARD",
        // Sent to a window to query the new palette.
        WM_QUERYNEWPALETTE => "WM_QUERYNEWPALETTE",
        // Sent to a window to indicate that the palette is changing.
        WM_PALETTEISCHANGING => "WM_PALETTEISCHANGING",
        // Sent to a window to indicate that the palette has changed.
        WM_PALETTECHANGED => "WM_PALETTECHANGED",
        // Sent to a window to set a hot key.
        WM_HOTKEY => "WM_HOTKEY",
        // Sent to a window to print the client area.
        WM_PRINT => "WM_PRINT",
        // Sent to a window to print the client area.
        WM_PRINTCLIENT => "WM_PRINTCLIENT",
        // Sent to a window when the application command is received.
        WM_APPCOMMAND => "WM_APPCOMMAND",
        // Sent to a window when the theme has changed.
        WM_THEMECHANGED => "WM_THEMECHANGED",
        // Sent to a window when the clipboard is updated.
        WM_CLIPBOARDUPDATE => "WM_CLIPBOARDUPDATE",
        // Sent to a window when the desktop window manager (DWM) composition has changed.
        WM_DWMCOMPOSITIONCHANGED => "WM_DWMCOMPOSITIONCHANGED",
        // Sent to a window when the desktop window manager (DWM) non-client rendering has changed.
        WM_DWMNCRENDERINGCHANGED => "WM_DWMNCRENDERINGCHANGED",
        // Sent to a window when the desktop window manager (DWM) colorization color has changed.
        WM_DWMCOLORIZATIONCOLORCHANGED => "WM_DWMCOLORIZATIONCOLORCHANGED",
        // Sent to a window when the desktop window manager (DWM) window maximized has changed.
        WM_DWMWINDOWMAXIMIZEDCHANGE => "WM_DWMWINDOWMAXIMIZEDCHANGE",
        // Sent to a window to send the iconic thumbnail.
        WM_DWMSENDICONICTHUMBNAIL => "WM_DWMSENDICONICTHUMBNAIL",
        // Sent to a window to send the iconic live preview bitmap.
        WM_DWMSENDICONICLIVEPREVIEWBITMAP => "WM_DWMSENDICONICLIVEPREVIEWBITMAP",
        // Sent to a window to get the title bar information.
        WM_GETTITLEBARINFOEX => "WM_GETTITLEBARINFOEX",
        // Default case for unknown messages.
        _ => "UNKNOWN_MESSAGE",
    }
}
