use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::{
    core::{GUID, HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::{ERROR_SUCCESS, HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR},
        NetworkManagement::WiFi::{
            WlanCloseHandle, WlanEnumInterfaces, WlanFreeMemory, WlanGetProfile,
            WlanGetProfileList, WlanOpenHandle, WLAN_API_VERSION_2_0, WLAN_INTERFACE_INFO,
            WLAN_INTERFACE_INFO_LIST, WLAN_PROFILE_GET_PLAINTEXT_KEY, WLAN_PROFILE_INFO,
            WLAN_PROFILE_INFO_LIST,
        },
    },
};

fn main() {
    let wlan_handle = open_wlan_handle(WLAN_API_VERSION_2_0).expect("Failed to open wlan handle");

    let interface_ptr = match enum_wlan_interfaces(wlan_handle) {
        Ok(interfaces) => interfaces,
        Err(e) => {
            eprintln!("Failed to enumerate wlan interfaces: {}", e);
            unsafe { WlanCloseHandle(wlan_handle, None) };
            // Exit the program with a non-zero status code because it broke
            std::process::exit(1);
        }
    };
}

fn open_wlan_handle(api_version: u32) -> Result<HANDLE, windows::core::Error> {
    let mut negotiated_version = 0;
    let mut wlan_handle = INVALID_HANDLE_VALUE;

    let result =
        unsafe { WlanOpenHandle(api_version, None, &mut negotiated_version, &mut wlan_handle) };

    WIN32_ERROR(result).ok()?;

    Ok(wlan_handle)
}

fn enum_wlan_interfaces(
    handle: HANDLE,
) -> Result<*mut WLAN_INTERFACE_INFO_LIST, windows::core::Error> {
    let mut interface_ptr = std::ptr::null_mut();
    let result = unsafe { WlanEnumInterfaces(handle, None, &mut interface_ptr) };

    WIN32_ERROR(result).ok()?;

    Ok(interface_ptr)
}

fn grab_interface_profiles(
    handle: HANDLE,
    interface_guid: &GUID,
) -> Result<*const WLAN_PROFILE_INFO_LIST, windows::core::Error> {
    let mut wlan_profiles_ptr = std::ptr::null_mut();

    let result =
        unsafe { WlanGetProfileList(handle, interface_guid, None, &mut wlan_profiles_ptr) };

    WIN32_ERROR(result).ok()?;

    Ok(wlan_profiles_ptr)
}

fn parse_utf16_slice(string_slice: &[u16]) -> Option<OsString> {
    let null_index = string_slice.iter().position(|&c| c == 0)?;
    Some(OsString::from_wide(&string_slice[..null_index]))
}
