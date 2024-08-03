pub mod hwnd;
pub mod shuffle;
pub mod win_icons;
use crate::img;

use windows::Win32::Foundation::HWND;

/// use komorebi_client::Window::hwnd for get hwnd
impl TryFrom<HWND> for super::WindowForSend {
    type Error = anyhow::Error;

    fn try_from(value: HWND) -> Result<Self, Self::Error> {
        let icon = self::hwnd::get_icon_from_hwnd(value.into())?;
        Ok(super::WindowForSend {
            icon: super::Icon {
                base64_icon: (img::convert_img_base64(&icon)?),
            },
            accent_color: img::find_most_used_color(&icon)?,
        })
    }
}
