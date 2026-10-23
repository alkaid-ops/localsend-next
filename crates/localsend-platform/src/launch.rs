#[cfg(any(target_os = "macos", windows))]
use std::process::Command;

#[cfg(target_os = "macos")]
const MACOS_LABEL: &str = "org.localsend.pc.prototype";

#[cfg(target_os = "linux")]
const LINUX_LABEL: &str = "org.localsend.pc.desktop";

#[cfg(target_os = "windows")]
const WINDOWS_VALUE: &str = "LocalSendPC";

pub fn apply_launch_on_login(enabled: bool) -> Result<(), String> {
    platform_apply(enabled)
}

#[cfg(target_os = "macos")]
fn platform_apply(enabled: bool) -> Result<(), String> {
    use std::fs;
    use std::path::PathBuf;
    use std::process::Stdio;

    let home = std::env::var_os("HOME").ok_or_else(|| "HOME is not set".to_string())?;
    let launch_agents = PathBuf::from(home).join("Library/LaunchAgents");
    let plist_path = launch_agents.join(format!("{MACOS_LABEL}.plist"));
    let uid = Command::new("id")
        .arg("-u")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|uid| uid.trim().to_string())
        .filter(|uid| !uid.is_empty())
        .unwrap_or_else(|| "501".to_string());
    let target = format!("gui/{uid}");

    if !enabled {
        let _ = Command::new("launchctl")
            .args(["bootout", &format!("{target}/{MACOS_LABEL}")])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        if plist_path.exists() {
            fs::remove_file(&plist_path).map_err(|error| error.to_string())?;
        }
        return Ok(());
    }

    fs::create_dir_all(&launch_agents).map_err(|error| error.to_string())?;
    let (program, mut arguments) = macos_program_and_arguments();
    arguments.push("--minimized".to_string());
    let mut program_arguments = String::new();
    for argument in std::iter::once(&program).chain(arguments.iter()) {
        program_arguments.push_str(&format!(
            "        <string>{}</string>\n",
            xml_escape(argument)
        ));
    }
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{MACOS_LABEL}</string>
    <key>ProgramArguments</key>
    <array>
{program_arguments}    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>ProcessType</key>
    <string>Interactive</string>
</dict>
</plist>
"#
    );
    fs::write(&plist_path, plist).map_err(|error| error.to_string())?;
    let plist = plist_path.to_string_lossy().to_string();
    let status = Command::new("launchctl")
        .args(["bootstrap", &target, &plist])
        .status()
        .map_err(|error| error.to_string())?;
    if !status.success() {
        let _ = Command::new("launchctl")
            .args(["load", "-w", &plist])
            .status();
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn macos_program_and_arguments() -> (String, Vec<String>) {
    let current = std::env::current_exe().ok();
    let bundle = current
        .as_ref()
        .and_then(|path| {
            path.ancestors().find(|ancestor| {
                ancestor
                    .extension()
                    .is_some_and(|extension| extension == "app")
            })
        })
        .map(|path| path.to_path_buf());
    if let Some(bundle) = bundle {
        (
            "/usr/bin/open".to_string(),
            vec![
                "-a".to_string(),
                bundle.to_string_lossy().to_string(),
                "--args".to_string(),
            ],
        )
    } else {
        (
            current
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|| "LocalSend".to_string()),
            Vec::new(),
        )
    }
}

#[cfg(target_os = "macos")]
fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(target_os = "linux")]
fn platform_apply(enabled: bool) -> Result<(), String> {
    use std::fs;
    use std::path::PathBuf;

    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .ok_or_else(|| "neither XDG_CONFIG_HOME nor HOME is set".to_string())?
        .join("autostart");
    let path = base.join(LINUX_LABEL);
    if !enabled {
        if path.exists() {
            fs::remove_file(path).map_err(|error| error.to_string())?;
        }
        return Ok(());
    }
    fs::create_dir_all(&base).map_err(|error| error.to_string())?;
    let executable = sibling_executable(&["localsend-launcher"])
        .ok_or_else(|| "localsend-launcher was not found next to the GUI".to_string())?;
    let contents = format!(
        "[Desktop Entry]\nType=Application\nName=LocalSend PC\nExec={} --background\nTryExec={}\nTerminal=false\nX-GNOME-Autostart-enabled=true\n",
        shell_quote(&executable.to_string_lossy()),
        shell_quote(&executable.to_string_lossy())
    );
    fs::write(path, contents).map_err(|error| error.to_string())
}

#[cfg(target_os = "linux")]
fn shell_quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg(target_os = "windows")]
fn platform_apply(enabled: bool) -> Result<(), String> {
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
    let status = if enabled {
        let executable = sibling_executable(&["localsend-tray.exe"])
            .or_else(|| sibling_executable(&["localsend-tray"]))
            .ok_or_else(|| "localsend-tray was not found next to the GUI".to_string())?;
        let value = format!("\"{}\" --minimized", executable.display());
        Command::new("reg")
            .args([
                "add",
                key,
                "/v",
                WINDOWS_VALUE,
                "/t",
                "REG_SZ",
                "/d",
                &value,
                "/f",
            ])
            .status()
    } else {
        Command::new("reg")
            .args(["delete", key, "/v", WINDOWS_VALUE, "/f"])
            .status()
    }
    .map_err(|error| error.to_string())?;
    if status.success() || !enabled {
        Ok(())
    } else {
        Err(format!("reg command failed with {status}"))
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn sibling_executable(names: &[&str]) -> Option<std::path::PathBuf> {
    let current = std::env::current_exe().ok()?;
    let parent = current.parent()?;
    names
        .iter()
        .map(|name| parent.join(name))
        .find(|path| path.is_file())
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn platform_apply(_enabled: bool) -> Result<(), String> {
    Ok(())
}
