pub const NOTIFICATION_APP_ID: &str = "org.localsend.pc.prototype";

#[cfg(not(windows))]
pub fn ensure_notification_identity() -> Result<(), String> {
    Ok(())
}

#[cfg(not(windows))]
pub fn clear_notification_history() -> Result<(), String> {
    Ok(())
}

#[cfg(windows)]
pub fn ensure_notification_identity() -> Result<(), String> {
    use windows::Win32::Storage::EnhancedStorage::PKEY_AppUserModel_ID;
    use windows::Win32::System::Com::IPersistFile;
    use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
    use windows::Win32::System::Com::{
        CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
        CoUninitialize,
    };
    use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
    use windows::Win32::UI::Shell::{
        IShellLinkW, SetCurrentProcessExplicitAppUserModelID, ShellLink,
    };
    use windows::core::{HSTRING, Interface};

    unsafe {
        SetCurrentProcessExplicitAppUserModelID(&HSTRING::from(NOTIFICATION_APP_ID))
            .map_err(|error| error.to_string())?;
    }

    let registration = windows_registry::CURRENT_USER
        .create(format!(
            r"Software\Classes\AppUserModelId\{NOTIFICATION_APP_ID}"
        ))
        .map_err(|error| error.to_string())?;
    registration
        .set_string("DisplayName", "LocalSend")
        .map_err(|error| error.to_string())?;

    let target = gui_executable_for_notifications()?;
    registration
        .set_string("IconUri", target.display().to_string())
        .map_err(|error| error.to_string())?;

    let shortcut = start_menu_shortcut_path()?;
    if let Some(parent) = shortcut.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let com_result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    let should_uninitialize = com_result.is_ok();
    let result = (|| {
        let shell_link: IShellLinkW =
            unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
                .map_err(|error| error.to_string())?;
        let target_path = target;
        let target = HSTRING::from(target_path.display().to_string());
        unsafe {
            shell_link
                .SetPath(&target)
                .map_err(|error| error.to_string())?;
            if let Some(parent) = target_path.parent() {
                shell_link
                    .SetWorkingDirectory(&HSTRING::from(parent.display().to_string()))
                    .map_err(|error| error.to_string())?;
            }
            shell_link
                .SetIconLocation(&target, 0)
                .map_err(|error| error.to_string())?;
        }

        let property_store: IPropertyStore =
            shell_link.cast().map_err(|error| error.to_string())?;
        let app_id = PROPVARIANT::from(NOTIFICATION_APP_ID);
        unsafe {
            property_store
                .SetValue(&PKEY_AppUserModel_ID, &app_id)
                .map_err(|error| error.to_string())?;
            property_store.Commit().map_err(|error| error.to_string())?;
        }

        let persist: IPersistFile = shell_link.cast().map_err(|error| error.to_string())?;
        unsafe {
            persist
                .Save(&HSTRING::from(shortcut.display().to_string()), true)
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    })();

    if should_uninitialize {
        unsafe { CoUninitialize() };
    }
    result
}

#[cfg(windows)]
pub fn clear_notification_history() -> Result<(), String> {
    use windows::UI::Notifications::ToastNotificationManager;
    use windows::core::HSTRING;

    let history = ToastNotificationManager::History().map_err(|error| error.to_string())?;
    history
        .ClearWithId(&HSTRING::from(NOTIFICATION_APP_ID))
        .map_err(|error| error.to_string())
}

#[cfg(windows)]
fn gui_executable_for_notifications() -> Result<std::path::PathBuf, String> {
    let current = std::env::current_exe().map_err(|error| error.to_string())?;
    let parent = current
        .parent()
        .ok_or_else(|| "tray executable has no parent directory".to_string())?;
    ["LocalSend.exe", "localsend-gui-slint.exe"]
        .into_iter()
        .map(|name| parent.join(name))
        .find(|path| path.is_file())
        .ok_or_else(|| "LocalSend GUI executable was not found for notifications".to_string())
}

#[cfg(windows)]
fn start_menu_shortcut_path() -> Result<std::path::PathBuf, String> {
    let app_data = std::env::var_os("APPDATA").ok_or_else(|| "APPDATA is not set".to_string())?;
    Ok(std::path::PathBuf::from(app_data)
        .join("Microsoft/Windows/Start Menu/Programs/LocalSend.lnk"))
}

#[cfg(test)]
mod tests {
    use super::NOTIFICATION_APP_ID;

    #[test]
    fn notification_identity_is_stable() {
        assert_eq!(NOTIFICATION_APP_ID, "org.localsend.pc.prototype");
    }
}
