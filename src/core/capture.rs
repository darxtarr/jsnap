use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::um::winnt::HANDLE;
use std::ptr;
use image::{ImageBuffer, Rgba};

/// Capture the entire desktop to RGBA image buffer.
pub fn capture_fullscreen() -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    unsafe {
        let hdc_screen = GetDC(ptr::null_mut());
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm = CreateCompatibleBitmap(hdc_screen, width, height);
        SelectObject(hdc_mem, hbm as HANDLE);
        BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);
        let mut buffer = vec![0u8; (width * height * 4) as usize];
        GetBitmapBits(hbm, buffer.len() as i32, buffer.as_mut_ptr() as *mut _);
        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, buffer)?;
        DeleteObject(hbm as HANDLE);
        DeleteDC(hdc_mem);
        ReleaseDC(ptr::null_mut(), hdc_screen);
        Some(img)
    }
}
