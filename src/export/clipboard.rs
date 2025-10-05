use std::{fs, mem, ptr};
use winapi::shared::minwindef::{HGLOBAL, UINT};
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::um::winnt::HANDLE;
use widestring::U16CString;
use image;

pub fn copy_image(path: &str) -> bool {
    let data = match fs::read(path) { Ok(d) => d, Err(_) => return false };
    let img = match image::load_from_memory(&data) { Ok(i) => i.to_rgba8(), Err(_) => return false };
    let (w, h) = img.dimensions();
    let stride = w * 4;
    let bmp_size = (stride * h) as usize;

    unsafe {
        let header = BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: w as i32,
            biHeight: -(h as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: bmp_size as u32,
            biXPelsPerMeter: 2835,
            biYPelsPerMeter: 2835,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        let hglobal: HGLOBAL = GlobalAlloc(GHND, (mem::size_of::<BITMAPINFOHEADER>() + bmp_size) as usize);
        if hglobal.is_null() { return false; }

        let dest = GlobalLock(hglobal) as *mut u8;
        if dest.is_null() { GlobalFree(hglobal); return false; }

        ptr::copy_nonoverlapping(&header as *const _ as *const u8, dest, mem::size_of::<BITMAPINFOHEADER>());
        let pixels_ptr = dest.add(mem::size_of::<BITMAPINFOHEADER>());
        ptr::copy_nonoverlapping(img.as_ptr(), pixels_ptr, bmp_size);
        GlobalUnlock(hglobal);

        if OpenClipboard(ptr::null_mut()) == 0 { GlobalFree(hglobal); return false; }
        EmptyClipboard();
        SetClipboardData(CF_DIB, hglobal);

        let cf_png: UINT = RegisterClipboardFormatW(U16CString::from_str("PNG").unwrap().as_ptr());
        let h_png = GlobalAlloc(GHND, data.len());
        if !h_png.is_null() {
            let ptr_png = GlobalLock(h_png) as *mut u8;
            ptr::copy_nonoverlapping(data.as_ptr(), ptr_png, data.len());
            GlobalUnlock(h_png);
            SetClipboardData(cf_png, h_png);
        }

        let cf_fname: UINT = RegisterClipboardFormatW(U16CString::from_str("FileNameW").unwrap().as_ptr());
        let fname = U16CString::from_str(path).unwrap();
        let bytes = (fname.len() + 1) * 2;
        let h_name = GlobalAlloc(GHND, bytes);
        if !h_name.is_null() {
            let ptr_name = GlobalLock(h_name) as *mut u16;
            ptr::copy_nonoverlapping(fname.as_ptr(), ptr_name, fname.len());
            *ptr_name.add(fname.len()) = 0;
            GlobalUnlock(h_name);
            SetClipboardData(cf_fname, h_name);
        }

        CloseClipboard();
        true
    }
}
