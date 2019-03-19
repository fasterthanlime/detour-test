#![allow(non_upper_case_globals, non_snake_case)]

#[macro_use]
extern crate lazy_static;
extern crate detour;
use detour::RawDetour;
use std::sync::Mutex;
use widestring::U16CString;

fn main() {
    unsafe {
        lazy_static::initialize(&MyMessageBoxHook);

        let text = U16CString::from_str("The original text").unwrap();
        let caption = U16CString::from_str("The original caption").unwrap();
        MessageBoxW(0, text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}

// here be dragons:

#[link(name = "user32")]
extern "C" {
    pub fn MessageBoxW(hwnd: i32, text: *const u16, caption: *const u16, typ: u32) -> ();
}

const MB_OK: u32 = 0x00000000;

unsafe extern "C" fn MyMessageBoxW(
    hwnd: i32,
    text: *const u16,
    caption: *const u16,
    typ: u32,
) -> () {
    let mut origText = U16CString::from_ptr_str(text).to_string().unwrap();
    origText += "... or is it?";
    let hookedText = U16CString::from_str(origText).unwrap();

    MyMessageBoxHook_original(hwnd, hookedText.as_ptr(), caption, typ);
}

lazy_static! {
    static ref MyMessageBoxHook: Mutex<RawDetour> = unsafe {
        let mut detour =
            RawDetour::new(MessageBoxW as *const (), MyMessageBoxW as *const ()).unwrap();
        detour.enable().unwrap();
        Mutex::new(detour)
    };
    static ref MyMessageBoxHook_original: unsafe extern "C" fn(hwnd: i32, text: *const u16, caption: *const u16, typ: u32) -> () =
        unsafe { std::mem::transmute(MyMessageBoxHook.lock().unwrap().trampoline()) };
}
