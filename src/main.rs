use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

const WH_KEYBOARD_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(13);

static mut ENABLED: bool = false;
static mut CTRL_PRESSED: bool = false;
static mut TILDE_PRESSED: bool = false;

fn main() {
    unsafe {
        let hook_id = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), HINSTANCE::default(), 0).unwrap();

        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            DispatchMessageA(&message);
        }
        
        if !hook_id.is_invalid() {
            UnhookWindowsHookEx(hook_id);
        }
    }
}

unsafe extern "system" fn hook_callback(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {  
    if ncode as u32 == HC_ACTION {
        let is_key_down = wparam.0 as u32 == WM_KEYDOWN;
        let is_key_up = wparam.0 as u32 == WM_KEYUP;
        
        let vk_code_inner = &*(lparam.0 as *mut u16) as &u16;
        
        if is_key_down || is_key_up {
            if *vk_code_inner == 223 {
                TILDE_PRESSED = is_key_down;
            } else if *vk_code_inner == 162 {
                CTRL_PRESSED = is_key_down;
            }
        }

        if TILDE_PRESSED && CTRL_PRESSED {
            ENABLED = !ENABLED;
        }

        println!("[code: {:?}] [enabled: {:?}] [tilde: {:?}] [ctrl: {:?}] [down: {:?}] [up: {:?}]", vk_code_inner, ENABLED, TILDE_PRESSED, CTRL_PRESSED, is_key_down, is_key_up);
        
        if ENABLED {
            return LRESULT(1);
        }
    }

    CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
}