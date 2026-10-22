#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

fn main() -> anyhow::Result<()> {
    localsend_platform::set_current_process_name("LocalSend");
    localsend_gui_slint::app::run_main()
}
