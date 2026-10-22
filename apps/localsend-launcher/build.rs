fn main() {
    #[cfg(windows)]
    embed_windows_resources();
}

#[cfg(windows)]
fn embed_windows_resources() {
    let icon = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../localsend-gui-slint/packaging/windows/app.ico");
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon(icon.to_string_lossy().as_ref());
    resource.set("FileDescription", "LocalSend Launcher");
    resource.set("ProductName", "LocalSend");
    resource.set("OriginalFilename", "localsend-launcher.exe");
    resource.set("FileVersion", "0.1.0.0");
    resource.set("ProductVersion", "0.1.0.0");
    resource.compile().expect("embed Windows resources");
}
