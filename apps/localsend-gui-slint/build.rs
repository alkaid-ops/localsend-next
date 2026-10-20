fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_debug_info(std::env::var("PROFILE").as_deref() != Ok("release"));
    slint_build::compile_with_config("ui/app-window.slint", config).expect("compile Slint UI");
}
