pub mod gui_actions;
pub mod gui_process;
pub mod launch;
#[cfg(target_os = "macos")]
pub mod macos_dialog_services;
pub mod notification_identity;
pub mod single_instance;

pub use gui_actions::{
    GuiAction, GuiControlServer, gui_action_queue_path, queue_gui_action, send_gui_control_action,
    take_queued_gui_actions,
};
pub use gui_process::{
    GuiPidGuard, TrayPidGuard, gui_pid_path, gui_process_is_running, read_gui_pid, read_tray_pid,
    terminate_gui_process, terminate_tray_process, tray_pid_path,
};
pub use launch::apply_launch_on_login;
#[cfg(target_os = "macos")]
pub use macos_dialog_services::with_native_file_dialog_cleanup;
#[cfg(not(target_os = "macos"))]
pub fn with_native_file_dialog_cleanup<T>(operation: impl FnOnce() -> T) -> T {
    operation()
}
pub use notification_identity::{
    NOTIFICATION_APP_ID, clear_notification_history, ensure_notification_identity,
};
pub use single_instance::{InstanceLock, scoped_lock_name};

pub fn set_current_process_name(name: &str) {
    #[cfg(target_os = "macos")]
    {
        use objc2_foundation::{NSProcessInfo, NSString};
        use std::ffi::CString;

        if let Ok(name) = CString::new(name) {
            unsafe {
                libc::setprogname(name.as_ptr());
            }
        }
        NSProcessInfo::processInfo().setProcessName(&NSString::from_str(name));
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        use std::ffi::CString;
        if let Ok(name) = CString::new(name) {
            unsafe {
                libc::prctl(libc::PR_SET_NAME, name.as_ptr());
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = name;
    }
}

pub fn activate_current_application_window() {
    #[cfg(target_os = "macos")]
    {
        use objc2_app_kit::NSApplication;
        use objc2_foundation::MainThreadMarker;

        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        let app = NSApplication::sharedApplication(mtm);
        app.activate();
        if let Some(window) = app.mainWindow().or_else(|| app.windows().iter().next()) {
            window.setAlphaValue(1.0);
            window.makeKeyAndOrderFront(None);
        }
    }
}

pub fn local_ipv4_addresses() -> std::io::Result<Vec<std::net::Ipv4Addr>> {
    let mut addresses = if_addrs::get_if_addrs()?
        .into_iter()
        .filter(|interface| !interface.is_loopback())
        .filter_map(|interface| match interface.ip() {
            std::net::IpAddr::V4(address) => Some(address),
            std::net::IpAddr::V6(_) => None,
        })
        .collect::<Vec<_>>();
    addresses.sort();
    addresses.dedup();
    Ok(addresses)
}
