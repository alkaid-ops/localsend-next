use std::collections::HashSet;
use std::io;
use std::time::Duration;

const DIALOG_SERVICE_NAMES: [&str; 2] = [
    "com.apple.appkit.xpc.openAndSavePanelService",
    "QuickLookUIService",
];
const EXIT_GRACE_PERIOD: Duration = Duration::from_millis(500);
const TERM_TIMEOUT: Duration = Duration::from_secs(1);
const POLL_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Debug, Default, PartialEq, Eq)]
struct DialogServiceCleanupReport {
    new_pids: Vec<u32>,
    terminated: Vec<u32>,
    forced: Vec<u32>,
}

trait DialogServiceBackend {
    fn scan(&self) -> io::Result<HashSet<u32>>;
    fn signal(&self, pid: u32, signal: i32) -> io::Result<()>;
    fn sleep(&self, duration: Duration);
}

#[derive(Default)]
struct SystemDialogServiceBackend;

impl DialogServiceBackend for SystemDialogServiceBackend {
    fn scan(&self) -> io::Result<HashSet<u32>> {
        scan_dialog_service_pids(|| {
            std::process::Command::new("/bin/ps")
                .args(["-x", "-o", "pid=,command="])
                .output()
        })
    }

    fn signal(&self, pid: u32, signal: i32) -> io::Result<()> {
        #[cfg(unix)]
        {
            if unsafe { libc::kill(pid as libc::pid_t, signal) } == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
        #[cfg(not(unix))]
        {
            let _ = (pid, signal);
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "dialog service cleanup is only supported on Unix",
            ))
        }
    }

    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
}

/// Owns the lifetime of system file-panel services created by one native dialog.
///
/// The AppKit and Quick Look XPC services are parented by launchd, so they are
/// not child processes of LocalSend. This guard records the services that
/// existed before the dialog and only reaps PIDs that appeared afterwards.
pub fn with_native_file_dialog_cleanup<T>(operation: impl FnOnce() -> T) -> T {
    #[cfg(target_os = "macos")]
    {
        let guard = DialogServiceGuard::begin();
        let result = operation();
        drop(guard);
        result
    }
    #[cfg(not(target_os = "macos"))]
    {
        operation()
    }
}

struct DialogServiceGuard {
    baseline: Option<HashSet<u32>>,
}

impl DialogServiceGuard {
    fn begin() -> Self {
        let baseline = SystemDialogServiceBackend.scan().ok();
        if baseline.is_none() {
            tracing::debug!("could not snapshot macOS file-dialog service PIDs");
        }
        Self { baseline }
    }
}

impl Drop for DialogServiceGuard {
    fn drop(&mut self) {
        let Some(baseline) = self.baseline.take() else {
            return;
        };
        match cleanup_new_services(&SystemDialogServiceBackend, &baseline) {
            Ok(report) if !report.new_pids.is_empty() => {
                tracing::debug!(
                    new_pids = ?report.new_pids,
                    terminated = ?report.terminated,
                    forced = ?report.forced,
                    "cleaned up macOS file-dialog services created by this dialog"
                );
            }
            Ok(_) => {}
            Err(error) => {
                tracing::warn!(%error, "failed to clean up macOS file-dialog services");
            }
        }
    }
}

fn cleanup_new_services<B: DialogServiceBackend>(
    backend: &B,
    baseline: &HashSet<u32>,
) -> io::Result<DialogServiceCleanupReport> {
    cleanup_new_services_with_timeout(backend, baseline, TERM_TIMEOUT)
}

fn cleanup_new_services_with_timeout<B: DialogServiceBackend>(
    backend: &B,
    baseline: &HashSet<u32>,
    term_timeout: Duration,
) -> io::Result<DialogServiceCleanupReport> {
    backend.sleep(EXIT_GRACE_PERIOD);
    let after = backend.scan()?;
    let mut new_pids = after.difference(baseline).copied().collect::<Vec<_>>();
    new_pids.sort_unstable();
    if new_pids.is_empty() {
        return Ok(DialogServiceCleanupReport::default());
    }

    let mut report = DialogServiceCleanupReport {
        new_pids: new_pids.clone(),
        ..DialogServiceCleanupReport::default()
    };
    for pid in &new_pids {
        match backend.signal(*pid, libc_signal_term()) {
            Ok(()) => report.terminated.push(*pid),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                tracing::debug!(pid, %error, "failed to terminate dialog service");
            }
        }
    }

    let deadline = std::time::Instant::now() + term_timeout;
    loop {
        let running = backend.scan()?;
        new_pids.retain(|pid| running.contains(pid));
        if new_pids.is_empty() || std::time::Instant::now() >= deadline {
            break;
        }
        backend.sleep(POLL_INTERVAL);
    }

    for pid in new_pids {
        match backend.signal(pid, libc_signal_kill()) {
            Ok(()) => report.forced.push(pid),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                tracing::debug!(pid, %error, "failed to kill lingering dialog service");
            }
        }
    }
    Ok(report)
}

fn scan_dialog_service_pids(
    run: impl FnOnce() -> io::Result<std::process::Output>,
) -> io::Result<HashSet<u32>> {
    let output = run()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "ps failed with status {}",
            output.status
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let (pid, command) = line.split_once(char::is_whitespace)?;
            let pid = pid.parse().ok()?;
            DIALOG_SERVICE_NAMES
                .iter()
                .any(|name| command.contains(name))
                .then_some(pid)
        })
        .collect())
}

#[cfg(unix)]
fn libc_signal_term() -> i32 {
    libc::SIGTERM
}

#[cfg(unix)]
fn libc_signal_kill() -> i32 {
    libc::SIGKILL
}

#[cfg(not(unix))]
fn libc_signal_term() -> i32 {
    0
}

#[cfg(not(unix))]
fn libc_signal_kill() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    #[derive(Default)]
    struct FakeBackend {
        scans: RefCell<VecDeque<io::Result<HashSet<u32>>>>,
        signals: RefCell<Vec<(u32, i32)>>,
    }

    impl DialogServiceBackend for FakeBackend {
        fn scan(&self) -> io::Result<HashSet<u32>> {
            self.scans
                .borrow_mut()
                .pop_front()
                .unwrap_or_else(|| Ok(HashSet::new()))
        }

        fn signal(&self, pid: u32, signal: i32) -> io::Result<()> {
            self.signals.borrow_mut().push((pid, signal));
            Ok(())
        }

        fn sleep(&self, _duration: Duration) {}
    }

    #[test]
    fn cleanup_only_targets_pids_created_after_the_baseline() {
        let backend = FakeBackend::default();
        backend
            .scans
            .borrow_mut()
            .push_back(Ok(HashSet::from([10, 30])));
        backend
            .scans
            .borrow_mut()
            .push_back(Ok(HashSet::from([30])));
        let report = cleanup_new_services(&backend, &HashSet::from([10])).unwrap();
        assert_eq!(report.new_pids, vec![30]);
        assert_eq!(report.terminated, vec![30]);
        assert!(report.forced.is_empty());
        assert_eq!(backend.signals.borrow().as_slice(), [(30, 15)]);
    }

    #[test]
    fn cleanup_escalates_only_the_same_lingering_pid() {
        let backend = FakeBackend::default();
        backend
            .scans
            .borrow_mut()
            .push_back(Ok(HashSet::from([20])));
        backend
            .scans
            .borrow_mut()
            .push_back(Ok(HashSet::from([20])));
        let report =
            cleanup_new_services_with_timeout(&backend, &HashSet::from([10]), Duration::ZERO)
                .unwrap();
        assert_eq!(report.new_pids, vec![20]);
        assert_eq!(report.terminated, vec![20]);
        assert_eq!(report.forced, vec![20]);
        assert_eq!(backend.signals.borrow().as_slice(), [(20, 15), (20, 9)]);
    }

    #[test]
    fn scan_failure_is_reported_without_signaling_processes() {
        let backend = FakeBackend::default();
        backend
            .scans
            .borrow_mut()
            .push_back(Err(io::Error::other("process inspection unavailable")));
        let error = cleanup_new_services(&backend, &HashSet::from([10])).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Other);
        assert!(backend.signals.borrow().is_empty());
    }

    #[test]
    fn ps_parser_matches_only_supported_dialog_services() {
        let output = std::process::Output {
            status: std::process::Command::new("/usr/bin/true")
                .status()
                .unwrap(),
            stdout: b"12 /System/QuickLookUIService\n13 unrelated\n14 /com.apple.appkit.xpc.openAndSavePanelService\n".to_vec(),
            stderr: Vec::new(),
        };
        let pids = scan_dialog_service_pids(|| Ok(output)).unwrap();
        assert_eq!(pids, HashSet::from([12, 14]));
    }
}
