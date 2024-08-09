use windows::{
    core::*, Win32::Foundation::*,
    Win32::Graphics::Gdi::{
        ValidateRect, CreateSolidBrush, UpdateWindow, SetRect, FillRect, GetDC, ReleaseDC
    },
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

const WIDTH : i32 = 800;
const HEIGHT : i32 = 600;

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let left : i32;
        let top : i32;

        left = (GetSystemMetrics(SM_CXSCREEN) - WIDTH) / 2;
        top = (GetSystemMetrics(SM_CYSCREEN) - HEIGHT) / 2;
        
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("This is a sample window"),
            WS_POPUP| WS_VISIBLE,
            left,
            top,
            WIDTH,
            HEIGHT,
            None,
            None,
            instance,
            None,
        )?;

        let _ = ShowWindow(hwnd, SW_SHOWNORMAL);
        let _ = UpdateWindow(hwnd);

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}


extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");

                let h_black = CreateSolidBrush(COLORREF(0x00191919));
                let hdc = GetDC(window);
                
                let mut rect = RECT::default();
                let _ = SetRect(&mut rect, 0, 0, WIDTH, HEIGHT);
                FillRect(hdc, &mut rect, h_black);
                ReleaseDC(window, hdc);
                
                _ = ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
