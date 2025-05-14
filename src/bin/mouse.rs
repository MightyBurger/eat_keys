use std::time::{self, Duration, Instant};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, LLMHF_INJECTED, LLMHF_LOWER_IL_INJECTED, MSG, MSLLHOOKSTRUCT,
    PM_REMOVE, PeekMessageA, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, WH_MOUSE_LL,
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_XBUTTONDOWN, WM_XBUTTONUP,
};

// This is a "LowLevelMouseProc"
// https://learn.microsoft.com/en-us/windows/win32/winmsg/lowlevelmouseproc

#[allow(non_snake_case)]
unsafe extern "system" fn mouse_hookfn(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    // Windows API: If nCode is less than zero, the hook procedure must pass the message
    // to the CallNextHookEx function without further processing and should return the value
    // returned by CallNextHookEx.
    if nCode < 0 {
        unsafe {
            return CallNextHookEx(None, nCode, wParam, lParam);
        }
    }

    // Let's not spam with MOUSEMOVE events yeah?
    if matches!(wParam.0 as u32, WM_MOUSEMOVE) {
        unsafe {
            return CallNextHookEx(None, nCode, wParam, lParam);
        }
    }

    let message = match wParam.0 as u32 {
        WM_LBUTTONDOWN => "WM_LBUTTONDOWN",
        WM_LBUTTONUP => "WM_LBUTTONUP",
        WM_MBUTTONDOWN => "WM_MBUTTONDOWN",
        WM_MBUTTONUP => "WM_MBUTTONUP",
        WM_MOUSEMOVE => "WM_MOUSEMOVE",
        WM_MOUSEWHEEL => "WM_MOUSEWHEEL",
        WM_RBUTTONDOWN => "WM_RBUTTONDOWN",
        WM_RBUTTONUP => "WM_RBUTTONUP",
        WM_XBUTTONDOWN => "WM_XBUTTONDOWN",
        WM_XBUTTONUP => "WM_XBUTTONUP",
        _ => "Unrecognized mouse message",
    };

    let hookstruct: MSLLHOOKSTRUCT = unsafe { *(lParam.0 as *const MSLLHOOKSTRUCT) };
    let _pt = hookstruct.pt;
    let mouse_data = hookstruct.mouseData;
    let time_seconds: f64 = hookstruct.time as f64 / 1000.0;

    let flags = hookstruct.flags;

    print!("{message:>14} │ 0x{mouse_data:08x} │ {time_seconds:>11.3} │");

    if flags & LLMHF_LOWER_IL_INJECTED != 0 {
        print!(" LLMHF_LOWER_IL_INJECTED");
    }
    if flags & LLMHF_INJECTED != 0 {
        print!(" LLMHF_INJECTED");
    }

    println!();

    // Return the keypress
    unsafe {
        return CallNextHookEx(None, nCode, wParam, lParam);
    }

    // Eat the keypress
    // return LRESULT(1);
}

pub fn main() {
    let mouse_hhk = {
        let idhook = WH_MOUSE_LL;
        let lpfn = Some(mouse_hookfn as unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT);
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

    println!("Displaying mouse events. Press CTRL+C to exit.");
    println!();
    println!("       Message │  MouseData │   Time (s)  │ Flags");
    println!("───────────────┼────────────┼─────────────┼─────────────────────────────────────");

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
    //     match UnhookWindowsHookEx(mouse_hhk) {
    //         Ok(()) => (),
    //         Err(e) => println!("Error unhooking: {e}"),
    //     }
    // }
    // println!("Done.");
}
