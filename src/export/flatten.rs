use crate::export::clipboard;
use crate::brand;
use crate::util::show_toast;
use chrono::Local;
use std::{fs, path::PathBuf};
use image::{ImageBuffer, Rgba};
use log::info;

/// Flatten and export the captured RGBA buffer to a temporary file,
/// then copy it to the clipboard.
pub fn export_and_copy(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Option<PathBuf> {
    // Ensure capture directory exists
    let dir = brand::user_capture_dir();
    std::fs::create_dir_all(&dir).ok()?;

    // Generate a friendly timestamped filename
    let ts = Local::now().format("Screenshot %Y-%m-%d %H-%M-%S").to_string();
    let mut path = dir;
    path.push(format!("{}.png", ts));

    // Save to disk
    match img.save(&path) {
        Ok(_) => info!("Saved screenshot to {}", path.display()),
        Err(e) => {
            log::error!("Failed to save screenshot: {}", e);
            return None;
        }
    }

    // Copy to clipboard
    if clipboard::copy_image(path.to_str().unwrap()) {
        info!("Copied {} to clipboard", path.display());
    } else {
        log::warn!("Clipboard copy failed");
    }

    Some(path)
}

if clipboard::copy_image(path.to_str().unwrap()) {
    show_toast("Screenshot copied", "You can paste in Word or Outlook now.");
}
