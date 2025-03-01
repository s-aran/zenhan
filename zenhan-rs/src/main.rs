use std::env;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::Ime::ImmGetDefaultIMEWnd,
        WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL},
    },
};

static IMC_GETOPENSTATUS: WPARAM = WPARAM(5);
static IMC_SETOPENSTATUS: WPARAM = WPARAM(6);

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    let h_wnd: HWND = unsafe { GetForegroundWindow() };
    if h_wnd == HWND::default() {
        return ();
    }

    let h_ime: HWND = unsafe { ImmGetDefaultIMEWnd(h_wnd) };
    if h_ime == HWND::default() {
        return ();
    }

    let stat: LRESULT;
    if argc < 2 {
        stat = unsafe {
            SendMessageW(
                h_ime,
                WM_IME_CONTROL,
                Some(IMC_GETOPENSTATUS),
                Some(LPARAM::default()),
            )
        };
    } else {
        let mode: i32 = match argv.get(1).unwrap_or(&"0".to_string()).parse() {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{}", e.to_string(),);
                return ();
            }
        };
        unsafe {
            SendMessageW(
                h_ime,
                WM_IME_CONTROL,
                Some(IMC_SETOPENSTATUS),
                Some(LPARAM(mode as isize)),
            )
        };
        stat = LRESULT(mode as isize);
    }

    println!("{}", stat.0);

    ()
}
