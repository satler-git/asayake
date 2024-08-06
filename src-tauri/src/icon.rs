pub mod hwnd;
pub mod shuffle;
pub mod win_icons;
use crate::img;

use image::{Pixel as _, Rgba};
use windows::Win32::Foundation::HWND;

fn cast_rgbau8_to_u32(rgba: &Rgba<u8>) -> u32 {
    let rgb = rgba.to_rgb();
    ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | (rgb[2] as u32)
}

/// use komorebi_client::Window::hwnd for get hwnd
impl TryFrom<HWND> for super::WindowForSend {
    type Error = anyhow::Error;

    fn try_from(value: HWND) -> Result<Self, Self::Error> {
        let icon = self::hwnd::get_icon_from_hwnd(value.into())?;
        Ok(super::WindowForSend {
            icon: super::Icon {
                base64_icon: (img::convert_img_base64(&icon)?),
            },
            accent_color: cast_rgbau8_to_u32(&img::find_most_used_color(&icon).unwrap()),
        })
    }
}
