use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub struct GuiPidGuard {
    path: PathBuf,
    pid: u32,
}

impl GuiPidGuard {
    pub fn acquire() -> io::Result<Self> {
        acquire_pid_guard(gui_pid_path())
    }
}

impl Drop for GuiPidGuard {
    fn drop(&mut self) {
        if read_gui_pid() == Some(self.pid) {
            let _ = fs::remove_file(&self.path);
        }
    }
}

pub fn gui_pid_path() -> PathBuf {
    config_dir().join("gui.pid")
}

pub fn read_gui_pid() -> Option<u32> {
    fs::read_to_string(gui_pid_path()).ok()?.trim().parse().ok()
}

pub fn terminate_gui_process() -> bool {
    terminate_pid_file(gui_pid_path(), process_is_gui)
}

pub fn gui_process_is_running() -> bool {
    read_gui_pid().is_some_and(process_is_gui)
}

pub struct TrayPidGuard {
    path: PathBuf,
    pid: u32,
}

impl TrayPidGuard {
    pub fn acquire() -> io::Result<Self> {
        acquire_pid_guard(tray_pid_path())
    }
}

impl Drop for TrayPidGuard {
    fn drop(&mut self) {
        if read_tray_pid() == Some(self.pid) {
            let _ = fs::remove_file(&self.path);
        }
    }
}

pub fn tray_pid_path() -> PathBuf {
    config_dir().join("tray.pid")
}

pub fn read_tray_pid() -> Option<u32> {
    fs::read_to_string(tray_pid_path())
        .ok()?
        .trim()
        .parse()
        .ok()
}

pub fn terminate_tray_process() -> bool {
    terminate_pid_file(tray_pid_path(), process_is_tray)
}

fn acquire_pid_guard<T>(path: PathBuf) -> io::Result<T>
where
    T: FromPidGuard,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let pid = std::process::id();
    let temporary = path.with_extension(format!("tmp-{pid}"));
    fs::write(&temporary, pid.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&temporary, fs::Permissions::from_mode(0o600))?;
    }
    fs::rename(temporary, &path)?;
    Ok(T::from_pid_guard(path, pid))
}

trait FromPidGuard {
    fn from_pid_guard(path: PathBuf, pid: u32) -> Self;
}

impl FromPidGuard for GuiPidGuard {
    fn from_pid_guard(path: PathBuf, pid: u32) -> Self {
        Self { path, pid }
    }
}

impl FromPidGuard for TrayPidGuard {
    fn from_pid_guard(path: PathBuf, pid: u32) -> Self {
        Self { path, pid }
    }
}

fn terminate_pid_file(path: PathBuf, matches: fn(u32) -> bool) -> bool {
    let Some(pid) = fs::read_to_string(&path)
        .ok()
        .and_then(|value| value.trim().parse().ok())
    else {
        return false;
    };
    if pid == std::process::id() || !matches(pid) {
        let _ = fs::remove_file(path);
        return false;
    }

    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid as libc::pid_t, libc::SIGTERM);
        }
        for _ in 0..30 {
            if !process_exists(pid) {
                let _ = fs::remove_file(path);
                return true;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        unsafe {
            libc::kill(pid as libc::pid_t, libc::SIGKILL);
        }
        let _ = fs::remove_file(path);
        return true;
    }

    #[cfg(windows)]
    {
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .status();
        let _ = fs::remove_file(path);
        return true;
    }

    #[allow(unreachable_code)]
    false
}

fn config_dir() -> PathBuf {
    std::env::var_os("LOCALSEND_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
                .map(|dirs| dirs.config_dir().to_path_buf())
        })
        .unwrap_or_else(|| std::env::temp_dir().join("localsend-rs"))
}

#[cfg(unix)]
fn process_exists(pid: u32) -> bool {
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

#[cfg(unix)]
fn process_is_gui(pid: u32) -> bool {
    let Ok(output) = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
    else {
        return false;
    };
    if !output.status.success() {
        return false;
    }
    let command = String::from_utf8_lossy(&output.stdout);
    command.contains("localsend-gui-slint")
        || command.contains("localsend-gui-egui")
        || command.contains("/LocalSend --window")
        || command.contains("/LocalSend.app/Contents/MacOS/LocalSend")
}

#[cfg(unix)]
fn process_is_tray(pid: u32) -> bool {
    let Ok(output) = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
    else {
        return false;
    };
    output.status.success()
        && ["localsend-tray", "LocalSendTray"]
            .iter()
            .any(|name| String::from_utf8_lossy(&output.stdout).contains(name))
}

#[cfg(windows)]
fn process_is_gui(pid: u32) -> bool {
    windows_image_name(pid).is_some_and(|name| name.eq_ignore_ascii_case("LocalSend.exe"))
}

#[cfg(windows)]
fn process_is_tray(pid: u32) -> bool {
    windows_image_name(pid).is_some_and(|name| name.eq_ignore_ascii_case("localsend-tray.exe"))
}

#[cfg(windows)]
fn windows_image_name(pid: u32) -> Option<String> {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&output.stdout);
    line.split(',')
        .next()
        .map(|name| name.trim().trim_matches('"').to_string())
        .filter(|name| !name.is_empty() && name != "INFO:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pid_guard_round_trips_and_removes_its_own_pid_file() {
        let root = std::env::temp_dir().join(format!(
            "localsend-gui-pid-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let previous = std::env::var_os("LOCALSEND_CONFIG_DIR");
        unsafe {
            std::env::set_var("LOCALSEND_CONFIG_DIR", &root);
        }
        {
            let guard = GuiPidGuard::acquire().unwrap();
            assert_eq!(read_gui_pid(), Some(std::process::id()));
            drop(guard);
        }
        assert_eq!(read_gui_pid(), None);
        {
            let guard = TrayPidGuard::acquire().unwrap();
            assert_eq!(read_tray_pid(), Some(std::process::id()));
            drop(guard);
        }
        assert_eq!(read_tray_pid(), None);
        unsafe {
            if let Some(previous) = previous {
                std::env::set_var("LOCALSEND_CONFIG_DIR", previous);
            } else {
                std::env::remove_var("LOCALSEND_CONFIG_DIR");
            }
        }
        let _ = fs::remove_dir_all(root);
    }
}
