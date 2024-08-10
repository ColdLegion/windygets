// use std::{
//     collections::HashMap,
//     sync::{atomic::Ordering, mpsc::Receiver, RwLock}
// };

use windows::{
    core::*, Win32::Foundation::*,
    Win32::Graphics::Gdi::{
        ValidateRect, CreateSolidBrush, UpdateWindow, SetRect, FillRect, GetDC, ReleaseDC
    },
    Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::WindowsAndMessaging::*,
};

// use crate::{
//     WIDTH, HEIGHT, BG_COLOR,
// };

pub fn create_window(width: i32, height: i32, posx: i32, posy: i32, bg_color: u32) -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let window_class = w!("windygets");

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hbrBackground: CreateSolidBrush(COLORREF(bg_color)),
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);

        // let left : i32 = (GetSystemMetrics(SM_CXSCREEN) - &width) / 2;
        // let top : i32 = (GetSystemMetrics(SM_CYSCREEN) - &height) / 2;

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            // WS_EX_TOOLWINDOW | WS_EX_LAYERED,
            window_class,
            w!("Windygets"),
            WS_POPUP| WS_VISIBLE,
            posx,
            posy,
            width,
            height,
            None,
            None,
            instance,
            None,
        )?;

        _ = ShowWindow(hwnd, SW_SHOWNORMAL);
        _ = UpdateWindow(hwnd);

        let mut message = MSG::default();

        while GetMessageW(&mut message, None, 0, 0).into() {
            DispatchMessageW(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");

                // let h_black = CreateSolidBrush(COLORREF(0x00191919));
                // let hdc = GetDC(window);
            
                // let mut rect = RECT::default();
                // _ = SetRect(&mut rect, 0, 0, WIDTH, HEIGHT);
                // FillRect(hdc, &mut rect, h_black);
                // ReleaseDC(window, hdc);
            
                _ = ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
