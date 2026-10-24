use anyhow::{Context, bail};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let page = option(&args, "--page", "receive");
    let state = option(&args, "--state", "default");
    let width = number(&args, "--width", 1024_u32)?;
    let height = number(&args, "--height", 768_u32)?;
    let dark = option(&args, "--theme", "light") == "dark";
    let language = option(&args, "--language", "zh-CN");
    let english = language.starts_with("en");
    let headless = args.iter().any(|argument| argument == "--headless");
    if headless {
        slint::platform::set_platform(Box::new(i_slint_backend_testing::TestingBackend::new(
            i_slint_backend_testing::TestingBackendOptions {
                mock_time: false,
                threading: true,
                renderer_name: Some("software".into()),
            },
        )))?;
    }
    let accent = match option(&args, "--accent", "teal").as_str() {
        "blue" => 1,
        "green" => 2,
        _ => 0,
    };
    let output = PathBuf::from(option(&args, "--output", "screenshot.png"));

    let ui = AppWindow::new()?;
    slint::select_bundled_translation(if english { "en-US" } else { "zh-CN" })?;
    ui.set_selected_tab(page.clone().into());
    ui.set_requested_window_width(width as f32);
    ui.set_requested_window_height(height as f32);
    ui.global::<ThemeTokens>().set_dark(dark);
    ui.global::<ThemeTokens>().set_accent_index(accent);
    seed_common_state(&ui, english);
    ui.set_settings_theme(
        if english {
            if dark { "Dark" } else { "Light" }
        } else if dark {
            "深色"
        } else {
            "浅色"
        }
        .into(),
    );
    match page.as_str() {
        "receive" => seed_receive(&ui, &state, english),
        "send" => seed_send(&ui, &state, english),
        "settings" => seed_settings(&ui),
        _ => bail!("unknown page: {page}"),
    }
    set_window_size(&ui, width, height);
    ui.show()?;

    let error = Rc::new(RefCell::new(None::<String>));
    let weak = ui.as_weak();
    let error_for_timer = error.clone();
    let timer = Box::leak(Box::new(slint::Timer::default()));
    timer.start(
        slint::TimerMode::SingleShot,
        Duration::from_millis(150),
        move || {
            let Some(ui) = weak.upgrade() else {
                *error_for_timer.borrow_mut() = Some("window was destroyed before sizing".into());
                let _ = slint::quit_event_loop();
                return;
            };
            set_window_size(&ui, width, height);
            let capture_weak = ui.as_weak();
            let capture_error = error_for_timer.clone();
            let capture_output = output.clone();
            let capture_timer = Box::leak(Box::new(slint::Timer::default()));
            capture_timer.start(
                slint::TimerMode::SingleShot,
                Duration::from_millis(350),
                move || {
                    let Some(ui) = capture_weak.upgrade() else {
                        *capture_error.borrow_mut() =
                            Some("window was destroyed before capture".into());
                        let _ = slint::quit_event_loop();
                        return;
                    };
                    ui.set_settings_scroll_y(0.0);
                    let settle_weak = ui.as_weak();
                    let settle_error = capture_error.clone();
                    let settle_output = capture_output.clone();
                    let settle_timer = Box::leak(Box::new(slint::Timer::default()));
                    settle_timer.start(
                        slint::TimerMode::SingleShot,
                        Duration::from_millis(100),
                        move || {
                            let result = settle_weak
                                .upgrade()
                                .context("window was destroyed while settling")
                                .and_then(|ui| {
                                    write_snapshot(
                                        &ui,
                                        &settle_output,
                                        headless.then_some((width, height)),
                                    )
                                });
                            if let Err(error) = result {
                                *settle_error.borrow_mut() = Some(error.to_string());
                            }
                            let _ = slint::quit_event_loop();
                        },
                    );
                },
            );
        },
    );
    ui.run()?;
    if let Some(error) = error.borrow().as_ref() {
        bail!("screenshot capture failed: {error}");
    }
    Ok(())
}

fn write_snapshot(
    ui: &AppWindow,
    output: &Path,
    crop_to: Option<(u32, u32)>,
) -> anyhow::Result<()> {
    let image = ui.window().take_snapshot()?;
    eprintln!(
        "capture {}x{}; viewport={}x{}",
        image.width(),
        image.height(),
        ui.get_viewport_width(),
        ui.get_viewport_height()
    );
    let (width, height) = crop_to.unwrap_or((image.width(), image.height()));
    if width > image.width() || height > image.height() {
        bail!(
            "captured image is {}x{}, smaller than requested crop {}x{}",
            image.width(),
            image.height(),
            width,
            height
        );
    }
    let pixels = if width == image.width() && height == image.height() {
        image.as_bytes().to_vec()
    } else {
        let source = image.as_bytes();
        let source_stride = image.width() as usize * 4;
        let target_stride = width as usize * 4;
        let mut cropped = vec![0; target_stride * height as usize];
        for row in 0..height as usize {
            let start = row * source_stride;
            cropped[row * target_stride..(row + 1) * target_stride]
                .copy_from_slice(&source[start..start + target_stride]);
        }
        cropped
    };
    let mut encoder = png::Encoder::new(
        File::create(output).with_context(|| format!("creating {}", output.display()))?,
        width,
        height,
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()?.write_image_data(&pixels)?;
    Ok(())
}

fn set_window_size(ui: &AppWindow, width: u32, height: u32) {
    // The Slint testing backend treats 800x600 as its unset window size and
    // otherwise falls back to the component's preferred size. Rendering one
    // pixel larger and cropping preserves the requested layout and dimensions.
    if width == 800 && height == 600 {
        ui.window().set_size(slint::LogicalSize::new(801.0, 601.0));
        return;
    }
    ui.window()
        .set_size(slint::LogicalSize::new(width as f32, height as f32));
}

fn option(args: &[String], key: &str, default: &str) -> String {
    args.windows(2)
        .find(|pair| pair[0] == key)
        .map(|pair| pair[1].clone())
        .unwrap_or_else(|| default.to_string())
}

fn number<T: std::str::FromStr>(args: &[String], key: &str, default: T) -> anyhow::Result<T>
where
    T::Err: std::error::Error + Send + Sync + 'static,
{
    let Some(value) = args.windows(2).find(|pair| pair[0] == key) else {
        return Ok(default);
    };
    Ok(value[1].parse()?)
}

fn seed_common_state(ui: &AppWindow, english: bool) {
    ui.set_device_alias("Flutter Reference".into());
    ui.set_device_id("#80".into());
    ui.set_info_alias("Flutter Reference".into());
    ui.set_info_ip("192.0.2.80".into());
    ui.set_info_port("53317".into());
    ui.set_server_status(
        if english {
            "Server running"
        } else {
            "服务器运行中"
        }
        .into(),
    );
    ui.set_server_running(true);
    ui.set_settings_theme(if english { "Light" } else { "浅色" }.into());
    ui.set_settings_color("LocalSend".into());
    ui.set_settings_language(if english { "English" } else { "简体中文" }.into());
    ui.set_settings_device_type("Desktop".into());
    ui.set_settings_device_model("macOS".into());
    ui.set_settings_discovery_timeout("10".into());
    ui.set_settings_multicast_group("224.0.0.167".into());
    ui.set_settings_network_filter(
        if english {
            "All interfaces"
        } else {
            "全部接口"
        }
        .into(),
    );
    ui.set_settings_encryption(true);
    ui.set_settings_require_pin(false);
    ui.set_settings_always_on_top(false);
    ui.set_save_window_position(true);
    ui.set_launch_on_login(false);
    ui.set_animations_enabled(true);
    ui.set_settings_auto_save_all(true);
    ui.set_settings_auto_save_favorites(false);
}

fn seed_receive(ui: &AppWindow, state: &str, english: bool) {
    if state == "completion" {
        ui.set_completion_dialog_open(true);
        ui.set_completion_file_name("official-256MiB.bin".into());
        ui.set_completion_file_path("/tmp/official-256MiB.bin".into());
        ui.set_completion_countdown(5);
    }
    if state == "web-share" {
        ui.set_web_share_dialog_open(true);
        ui.set_web_share_running(true);
        ui.set_web_share_url("http://192.0.2.80:53317/".into());
        ui.set_settings_encryption(false);
        ui.set_settings_send_auto_accept(false);
        ui.set_settings_pin_enabled(false);
    }
    if state == "message" {
        ui.set_message_received_open(true);
        ui.set_message_received_sender("Pixel 9".into());
        ui.set_message_received_text(
            if english {
                "This is a text message received through LocalSend.\nIt supports multiple lines and can be copied directly."
            } else {
                "这是通过 LocalSend 收到的文本消息。\n支持多行内容，并可以直接复制。"
            }
            .into(),
        );
    }
    if state == "request" {
        let files = vec![
            RequestFileItem {
                file_id: "file-1".into(),
                name: "presentation.pdf".into(),
                size: "4.2 MiB".into(),
                selected: true,
            },
            RequestFileItem {
                file_id: "file-2".into(),
                name: "photos".into(),
                size: "18 files".into(),
                selected: false,
            },
        ];
        ui.set_incoming_request_count(1);
        ui.set_incoming_request_id("request-1".into());
        ui.set_incoming_request_title(
            if english {
                "iPhone wants to send files"
            } else {
                "iPhone 想发送文件"
            }
            .into(),
        );
        ui.set_incoming_request_body(if english { "2 files" } else { "2 个文件" }.into());
        ui.set_incoming_files(ModelRc::new(VecModel::from(files)));
        ui.set_selected_request_ids(ModelRc::new(VecModel::from(vec!["file-1".into()])));
    }
    if state == "transferring" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "transfer-1".into(),
            peer: "iPhone".into(),
            summary: "8.0 MiB / 24.0 MiB".into(),
            status: "传输中".into(),
            direction: "接收".into(),
            terminal: false,
            progress_percent: 33,
            file_count: 2,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(Vec::<TransferFileItem>::new())),
        }])));
        ui.set_active_transfer_count(1);
    }
}

fn seed_send(ui: &AppWindow, state: &str, english: bool) {
    if state == "text" {
        ui.set_selected_files(ModelRc::new(VecModel::from(vec![SelectedFileItem {
            path: "".into(),
            name: if english {
                "Text message"
            } else {
                "文本消息"
            }
            .into(),
            is_directory: false,
            is_text: true,
            is_image: false,
            size: "1 B".into(),
        }])));
        ui.set_has_selection(true);
        ui.set_selected_file_count(1);
        ui.set_selected_total_size("1 B".into());
        ui.set_selected_files_text(
            if english {
                "Text message"
            } else {
                "文本消息"
            }
            .into(),
        );
        ui.set_selected_target_label("Pixel 9".into());
    }
    if state == "selected" {
        ui.set_selected_files(ModelRc::new(VecModel::from(vec![SelectedFileItem {
            path: "/tmp/report.pdf".into(),
            name: "report.pdf".into(),
            is_directory: false,
            is_text: false,
            is_image: false,
            size: "2.4 MiB".into(),
        }])));
        ui.set_has_selection(true);
        ui.set_selected_file_count(1);
        ui.set_selected_total_size("2.4 MiB".into());
        ui.set_selected_files_text("report.pdf".into());
        ui.set_selected_target_label("Pixel 9".into());
    }
    if state == "multiple" {
        ui.set_selected_files(ModelRc::new(VecModel::from(vec![
            SelectedFileItem {
                path: "/tmp/report.pdf".into(),
                name: "report.pdf".into(),
                is_directory: false,
                is_text: false,
                is_image: false,
                size: "2.4 MiB".into(),
            },
            SelectedFileItem {
                path: "/tmp/Photos".into(),
                name: "Photos".into(),
                is_directory: true,
                is_text: false,
                is_image: false,
                size: if english { "Folder" } else { "文件夹" }.into(),
            },
        ])));
        ui.set_has_selection(true);
        ui.set_selected_file_count(2);
        ui.set_selected_total_size("7.8 MiB".into());
        ui.set_selected_files_text("report.pdf · Photos".into());
        ui.set_selected_target_label(
            if english {
                "2 devices selected"
            } else {
                "已选择 2 台设备"
            }
            .into(),
        );
        ui.set_devices(ModelRc::new(VecModel::from(vec![
            DeviceItem {
                alias: "Pixel 9".into(),
                ip: "192.0.2.81".into(),
                fingerprint: "fingerprint-1".into(),
                protocol: "HTTPS".into(),
                model: "Pixel 9".into(),
                device_type: "Mobile".into(),
                favorite: true,
                selected: true,
            },
            DeviceItem {
                alias: "Windows PC".into(),
                ip: "192.0.2.82".into(),
                fingerprint: "fingerprint-2".into(),
                protocol: "HTTP".into(),
                model: "Desktop".into(),
                device_type: "Desktop".into(),
                favorite: false,
                selected: true,
            },
        ])));
        ui.set_send_mode(1);
    }
    if state == "transferring" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "send-transfer-1".into(),
            peer: "iPhone".into(),
            summary: "8.0 MiB / 24.0 MiB".into(),
            status: "传输中".into(),
            direction: "发送".into(),
            terminal: false,
            progress_percent: 33,
            file_count: 2,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(vec![
                TransferFileItem {
                    name: "SCR-20260927-tlhj.jpeg".into(),
                    size: "1.1 MB".into(),
                    is_text: false,
                },
                TransferFileItem {
                    name: "SCR-20260927-tlel.jpeg".into(),
                    size: "1.5 MB".into(),
                    is_text: false,
                },
            ])),
        }])));
        ui.set_active_transfer_count(1);
        ui.set_sending_active(true);
    }
    if state == "completed" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "send-transfer-completed".into(),
            peer: "iPhone".into(),
            summary: "2.6 MB / 2.6 MB".into(),
            status: "已完成".into(),
            direction: "发送".into(),
            terminal: true,
            progress_percent: 100,
            file_count: 2,
            completion_countdown: 3,
            files: ModelRc::new(VecModel::from(vec![
                TransferFileItem {
                    name: "SCR-20260927-tlhj.jpeg".into(),
                    size: "1.1 MB".into(),
                    is_text: false,
                },
                TransferFileItem {
                    name: "SCR-20260927-tlel.jpeg".into(),
                    size: "1.5 MB".into(),
                    is_text: false,
                },
            ])),
        }])));
        ui.set_active_transfer_count(1);
        ui.set_sending_active(true);
    }
}

fn seed_settings(ui: &AppWindow) {
    ui.set_settings_device_type("Desktop".into());
    ui.set_settings_device_model("MacBook Air".into());
    ui.set_settings_advanced(false);
}
