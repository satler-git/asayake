pub mod hwnd;
pub mod shuffle;
pub mod win_icons;
use crate::img;

use windows::Win32::Foundation::HWND;

/// (base64, color)
/// use komorebi_client::Window::hwnd for get hwnd
pub fn extract_icon_base64_color_from_hwnd(hwnd_i: HWND) -> (String, u8) {
    let icon = self::hwnd::get_icon_from_hwnd(hwnd_i.into()).unwrap();
    (
        img::convert_img_base64(&icon),
        img::find_most_used_color(icon),
    )
}
