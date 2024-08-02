use anyhow::Result;

use image::DynamicImage;

use windows::core::PWSTR;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::QueryFullProcessImageNameW;
use windows::Win32::System::Threading::PROCESS_NAME_WIN32;
use windows::Win32::System::Threading::PROCESS_QUERY_LIMITED_INFORMATION;
use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;

use super::win_icons::get_images_from_exe;

/// hwndからアイコンを取得
pub fn get_icon_from_hwnd(hwnd: HWND) -> Result<DynamicImage> {
    let mut process_id: u32 = 0;

    unsafe {
        GetWindowThreadProcessId(hwnd, Option::from(std::ptr::addr_of_mut!(process_id)));
    }

    // handleを開く
    let handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION,
            false,
            process_id.clone().into(),
        )
    }?;

    let mut len = 260_u32;
    let mut path: Vec<u16> = vec![0; len as usize];
    let text_ptr = path.as_mut_ptr();

    unsafe { QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, PWSTR(text_ptr), &mut len) }?;

    let process_exe_path = String::from_utf16(&path[..len as usize])?;

    // handleのクローズ
    unsafe { CloseHandle(handle) }?;

    let images = get_images_from_exe(&process_exe_path)?;
    Ok(images[0].clone().into())
}
