fn main() {
    for path in source_files(std::path::Path::new("ui")) {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!("cargo:rerun-if-changed=translations");
    let config = slint_build::CompilerConfiguration::new()
        .with_debug_info(std::env::var("PROFILE").as_deref() != Ok("release"))
        .with_bundled_translations("translations")
        .with_default_translation_context(slint_build::DefaultTranslationContext::None);
    slint_build::compile_with_config("ui/app-window.slint", config).expect("compile Slint UI");
    #[cfg(windows)]
    embed_windows_resources();
}

fn source_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(root) else {
        return files;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(source_files(&path));
        } else {
            files.push(path);
        }
    }
    files
}

#[cfg(windows)]
fn embed_windows_resources() {
    let icon =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("packaging/windows/app.ico");
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon(icon.to_string_lossy().as_ref());
    resource.set("FileDescription", "LocalSend");
    resource.set("ProductName", "LocalSend");
    resource.set("OriginalFilename", "LocalSend.exe");
    resource.set("FileVersion", "0.1.0.0");
    resource.set("ProductVersion", "0.1.0.0");
    resource.compile().expect("embed Windows resources");
}
