pub mod gui_process;
pub mod launch;
pub mod macos_dialog_services;
pub mod single_instance;

pub use gui_process::{
    GuiPidGuard, TrayPidGuard, gui_pid_path, read_gui_pid, read_tray_pid, terminate_gui_process,
    terminate_tray_process, tray_pid_path,
};
pub use launch::apply_launch_on_login;
pub use macos_dialog_services::with_native_file_dialog_cleanup;
pub use single_instance::{InstanceLock, scoped_lock_name};

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
