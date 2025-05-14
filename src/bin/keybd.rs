// use std::time::{self, Duration, Instant};
// use windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, KBDLLHOOKSTRUCT, LLKHF_ALTDOWN, LLKHF_EXTENDED,
    LLKHF_INJECTED, LLKHF_LOWER_IL_INJECTED, LLKHF_UP, MSG, PM_REMOVE, PeekMessageA,
    SetWindowsHookExA, TranslateMessage, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP,
};

// This is a "LowLevelKeyboardProc"
// https://learn.microsoft.com/en-us/windows/win32/winmsg/lowlevelkeyboardproc

#[allow(non_snake_case)]
unsafe extern "system" fn kb_hookfn(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    // Windows API: If nCode is less than zero, the hook procedure must pass the message
    // to the CallNextHookEx function without further processing and should return the value
    // returned by CallNextHookEx.
    if nCode < 0 {
        unsafe {
            return CallNextHookEx(None, nCode, wParam, lParam);
        }
    }

    let message = match wParam.0 as u32 {
        WM_KEYDOWN => "WM_KEYDOWN",
        WM_KEYUP => "WM_KEYUP",
        WM_SYSKEYDOWN => "WM_SYSKEYDOWN",
        WM_SYSKEYUP => "WM_SYSKEYUP",
        _ => "Unrecognized keyboard message",
    };

    let hookstruct: KBDLLHOOKSTRUCT = unsafe { *(lParam.0 as *const KBDLLHOOKSTRUCT) };
    let vk_code = hookstruct.vkCode;
    let scan_code = hookstruct.scanCode;
    let time_seconds: f64 = hookstruct.time as f64 / 1000.0;

    let flags = hookstruct.flags;

    print!("{message:>14} │ 0x{vk_code:02x} │ 0x{scan_code:02x} │ {time_seconds:>11.3} │");

    if flags.contains(LLKHF_EXTENDED) {
        print!(" LLKHF_EXTENDED");
    }
    if flags.contains(LLKHF_LOWER_IL_INJECTED) {
        print!(" LLKHF_LOWER_IL_INJECTED");
    }
    if flags.contains(LLKHF_INJECTED) {
        print!(" LLHKF_INJECTED");
    }
    if flags.contains(LLKHF_ALTDOWN) {
        print!(" LLHKF_ALTDOWN");
    }
    if flags.contains(LLKHF_UP) {
        print!(" LLKHF_UP");
    }

    println!();

    // Return the button press
    unsafe {
        return CallNextHookEx(None, nCode, wParam, lParam);
    }

    // Eat the button press
    // return LRESULT(1);
}

pub fn main() {
    let _kb_hhk = {
        let idhook = WH_KEYBOARD_LL;
        let lpfn = Some(kb_hookfn as unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT);
        let hmod = None;
        let dwthreadid = 0;
        unsafe {
            match SetWindowsHookExA(idhook, lpfn, hmod, dwthreadid) {
                Ok(hhk) => hhk,
                Err(e) => {
                    println!("Error hooking: {e}");
                    return;
                }
            }
        }
    };

    println!();
    println!("Displaying key events. Press CTRL+C to exit.");
    println!("Warning: these key presses are not suppressed.");
    println!("So, don't test this while the terminal is the active window.");
    println!("Otherwise, everything you type will show up when you exit the program.");
    println!();
    println!("       Message │  VK  │ Scan │   Time (s)  │ Flags");
    println!("───────────────┼──────┼──────┼─────────────┼─────────────────────────────────────");
    loop {
        let mut lpmsg = MSG::default();
        unsafe {
            if PeekMessageA(&mut lpmsg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&lpmsg);
                let _ = DispatchMessageA(&lpmsg);
            }
        }
    }

    // let start_time = Instant::now();
    // let block_time = Duration::from_millis(10000);

    // while Instant::now() < start_time + block_time {
    //     let mut lpmsg = MSG::default();
    //     unsafe {
    //         if PeekMessageA(&mut lpmsg, None, 0, 0, PM_REMOVE).as_bool() {
    //             let _ = TranslateMessage(&lpmsg);
    //             let _ = DispatchMessageA(&lpmsg);
    //         }
    //     }
    // }

    // unsafe {
    //     match UnhookWindowsHookEx(kb_hhk) {
    //         Ok(()) => (),
    //         Err(e) => println!("Error unhooking: {e}"),
    //     }
    // }
    // println!("Done.");
}
