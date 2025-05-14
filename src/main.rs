use std::time::{self, Duration, Instant};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, MSG, PM_REMOVE, PeekMessageA, SetWindowsHookExA,
    TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP,
};

#[allow(non_snake_case)]
unsafe extern "system" fn hookfn(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    let t = Instant::now();
    match wParam.0 as u32 {
        WM_KEYDOWN => println!("WM_KEYDOWN    {:?}", t),
        WM_KEYUP => println!("WM_KEYUP      {:?}", t),
        WM_SYSKEYDOWN => println!("WM_SYSKEYDOWN {:?}", t),
        WM_SYSKEYUP => println!("WM_SYSKEYUP   {:?}", t),
        other => println!("Unrecognized keyboard message: {:?}", other),
    }
    if nCode < 0 {
        unsafe {
            return CallNextHookEx(None, nCode, wParam, lParam);
        }
    } else {
        return LRESULT(1);
    };
}

pub fn main() {
    let idhook = WH_KEYBOARD_LL;
    let lpfn = Some(hookfn as unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT);
    let hmod = None;
    let dwthreadid = 0;

    unsafe {
        let hhk = match SetWindowsHookExA(idhook, lpfn, hmod, dwthreadid) {
            Ok(hhk) => hhk,
            Err(e) => {
                println!("Error hooking: {e}");
                return;
            }
        };
        println!("Eating and displaying keys for 5 seconds.");

        let start_time = Instant::now();
        let block_time = Duration::from_millis(5000);

        while Instant::now() < start_time + block_time {
            let mut lpmsg = MSG::default();
            if PeekMessageA(&mut lpmsg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&lpmsg);
                let _ = DispatchMessageA(&lpmsg);
            }
            std::thread::sleep(time::Duration::from_millis(10));
        }

        match UnhookWindowsHookEx(hhk) {
            Ok(()) => (),
            Err(e) => println!("Error unhooking: {e}"),
        }
        println!("Done.");
    }
}
