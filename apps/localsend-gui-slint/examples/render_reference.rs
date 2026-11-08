use anyhow::{Context, bail};
use i_slint_backend_testing::ElementQuery;
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
    let print_metrics = args.iter().any(|argument| argument == "--metrics");
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
    ui.set_freeze_motion(true);
    slint::select_bundled_translation(&language)?;
    ui.set_selected_tab(page.clone().into());
    ui.set_requested_window_width(width as f32);
    ui.set_requested_window_height(height as f32);
    ui.global::<ThemeTokens>().set_dark(dark);
    ui.global::<ThemeTokens>().set_accent_index(accent);
    seed_common_state(&ui, english, &language);
    seed_dialog_state(&ui, &state, english);
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
    ui.set_settings_theme_code((if dark { "dark" } else { "light" }).into());
    match page.as_str() {
        "receive" => seed_receive(&ui, &state, english),
        "send" => seed_send(&ui, &state, english),
        "settings" => seed_settings(&ui, &language),
        _ => bail!("unknown page: {page}"),
    }
    set_window_size(&ui, width, height);
    ui.show()?;
    if print_metrics {
        print_accessible_metrics(&ui);
    }

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

fn print_accessible_metrics(ui: &AppWindow) {
    #[derive(serde::Serialize)]
    struct Metric {
        id: String,
        label: Option<String>,
        role: Option<String>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    let metrics = ElementQuery::from_root(ui)
        .find_all()
        .into_iter()
        .filter_map(|element| {
            let id = element.accessible_id().map(|id| id.to_string());
            let label = element.accessible_label().map(|label| label.to_string());
            if id.as_deref().unwrap_or("").is_empty() && label.as_deref().unwrap_or("").is_empty() {
                return None;
            }
            let position = element.absolute_position();
            let size = element.size();
            Some(Metric {
                id: id.unwrap_or_default(),
                label,
                role: element.accessible_role().map(|role| format!("{role:?}")),
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
            })
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::to_string_pretty(&metrics).expect("serialize accessibility metrics")
    );
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

fn seed_common_state(ui: &AppWindow, english: bool, language: &str) {
    ui.set_device_alias("Flutter Reference".into());
    ui.set_device_id("#80".into());
    ui.set_info_alias("Flutter Reference".into());
    ui.set_info_ip("192.168.1.42".into());
    ui.set_info_fingerprint("E4:7A:21:9C:88:BD".into());
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
    ui.set_settings_color("LocalSend Teal".into());
    ui.set_settings_language_code(language.into());
    ui.set_settings_language_name(localsend_gui_slint::reference_language_name(language).into());
    let language_items: Vec<LanguageItem> = localsend_gui_slint::reference_language_items(english)
        .into_iter()
        .map(|(code, name)| LanguageItem {
            code: code.into(),
            name: name.into(),
        })
        .collect();
    ui.set_language_items(ModelRc::new(VecModel::from(language_items)));
    ui.set_settings_device_type("Desktop".into());
    ui.set_settings_device_model("macOS".into());
    ui.set_settings_discovery_timeout("10".into());
    ui.set_settings_multicast_group("224.0.0.167".into());
    ui.set_settings_network_filter("192.168.1.0/24".into());
    ui.set_settings_encryption(true);
    ui.set_settings_require_pin(false);
    ui.set_settings_always_on_top(false);
    ui.set_save_window_position(true);
    ui.set_launch_on_login(false);
    ui.set_animations_enabled(true);
    ui.set_settings_auto_save_all(false);
    ui.set_settings_auto_save_favorites(true);
}

fn seed_receive(ui: &AppWindow, state: &str, english: bool) {
    if matches!(
        state,
        "history"
            | "info"
            | "manual"
            | "favorites"
            | "pin"
            | "text"
            | "troubleshoot"
            | "auto-save-help"
            | "language"
    ) {
        return;
    }
    if state == "completion" {
        ui.set_completion_dialog_open(true);
        ui.set_completion_file_name("official-256MiB.bin".into());
        ui.set_completion_file_path("/tmp/official-256MiB.bin".into());
        ui.set_completion_countdown(5);
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
                name: "产品评审说明.pdf".into(),
                size: "4.2 MiB · PDF".into(),
                selected: true,
            },
            RequestFileItem {
                file_id: "file-2".into(),
                name: "界面截图-2026-10-24.png".into(),
                size: "2.8 MiB · PNG".into(),
                selected: true,
            },
            RequestFileItem {
                file_id: "file-3".into(),
                name: "组件清单.md".into(),
                size: "18 KiB · Markdown".into(),
                selected: true,
            },
        ];
        ui.set_incoming_request_count(1);
        ui.set_incoming_request_id("request-1".into());
        ui.set_incoming_request_title("iPhone 15 Pro".into());
        ui.set_incoming_request_body(if english { "2 files" } else { "2 个文件" }.into());
        ui.set_incoming_files(ModelRc::new(VecModel::from(files)));
        ui.set_selected_request_ids(ModelRc::new(VecModel::from(vec![
            "file-1".into(),
            "file-2".into(),
            "file-3".into(),
        ])));
    }
    if state == "transferring" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "transfer-1".into(),
            peer: "iPhone 15 Pro".into(),
            summary: "3 个文件 · 共 7.0 MiB".into(),
            status: "传输中".into(),
            speed: "18.6 MB/s".into(),
            duration: "".into(),
            direction: "接收".into(),
            terminal: false,
            progress_percent: 64,
            file_count: 3,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(vec![TransferFileItem {
                name: "界面截图-2026-10-24.png".into(),
                size: "4.5 MiB / 7.0 MiB · 18.6 MB/s".into(),
                is_text: false,
                is_image: true,
                status: "64%".into(),
                detail: "正在传输".into(),
                progress_percent: 64,
                completed: false,
                in_progress: true,
            }])),
        }])));
        ui.set_active_transfer_count(1);
    }
    if state == "completed" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "receive-completed".into(),
            peer: "iPhone 15 Pro".into(),
            summary: "7.0 MiB".into(),
            status: "已完成".into(),
            speed: "".into(),
            duration: "".into(),
            direction: "接收".into(),
            terminal: true,
            progress_percent: 100,
            file_count: 3,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(vec![
                TransferFileItem {
                    name: "产品评审说明.pdf".into(),
                    size: "4.2 MiB".into(),
                    is_text: false,
                    is_image: false,
                    status: "已完成".into(),
                    detail: "已完成".into(),
                    progress_percent: 100,
                    completed: true,
                    in_progress: false,
                },
                TransferFileItem {
                    name: "界面截图-2026-10-24.png".into(),
                    size: "2.8 MiB".into(),
                    is_text: false,
                    is_image: true,
                    status: "已完成".into(),
                    detail: "已完成".into(),
                    progress_percent: 100,
                    completed: true,
                    in_progress: false,
                },
                TransferFileItem {
                    name: "组件清单.md".into(),
                    size: "18 KiB".into(),
                    is_text: false,
                    is_image: false,
                    status: "已完成".into(),
                    detail: "已完成".into(),
                    progress_percent: 100,
                    completed: true,
                    in_progress: false,
                },
            ])),
        }])));
        ui.set_active_transfer_count(1);
    }
}

fn seed_dialog_state(ui: &AppWindow, state: &str, _english: bool) {
    if state == "history" {
        ui.set_history_items(ModelRc::new(VecModel::from(vec![
            HistoryItem {
                id: "history-1".into(),
                name: "来自 iPhone 15 Pro · 3 个文件".into(),
                subtitle: "今天 10:24 · Downloads/LocalSend/产品评审说明.pdf".into(),
                path: "/tmp/presentation.pdf".into(),
                message: "今天 10:24 · 界面截图-2026-10-24.png、组件清单.md".into(),
                is_message: false,
            },
            HistoryItem {
                id: "history-2".into(),
                name: "发送到 Flutter Reference · 产品视觉规范.pdf".into(),
                subtitle: "昨天 18:42 · 4.0 MiB · 已完成".into(),
                path: "/tmp/visual-spec.pdf".into(),
                message: "".into(),
                is_message: false,
            },
            HistoryItem {
                id: "history-3".into(),
                name: "来自 Pixel 9 · 文本消息".into(),
                subtitle: "10 月 22 日 15:08 · 全文已展示".into(),
                path: "".into(),
                message: "本次评审需覆盖 14 个关键状态，并确认 Light/Dark、窄窗口和传输异常恢复。"
                    .into(),
                is_message: true,
            },
        ])));
        ui.set_history_dialog_open(true);
    }
    if state == "info" {
        ui.set_info_dialog_open(true);
    }
    if state == "manual" {
        ui.set_manual_dialog_open(true);
        ui.set_manual_target("192.168.1.42".into());
    }
    if state == "favorites" {
        ui.set_favorites(ModelRc::new(VecModel::from(vec![
            FavoriteItem {
                alias: "Flutter Reference".into(),
                fingerprint: "reference".into(),
            },
            FavoriteItem {
                alias: "iPhone 15 Pro".into(),
                fingerprint: "iphone".into(),
            },
        ])));
        seed_default_devices(ui);
        ui.set_favorites_dialog_open(true);
    }
    if state == "pin" {
        ui.set_pin_dialog_open(true);
        ui.set_pin_transfer_id("pin-test".into());
    }
    if state == "text-dialog" {
        ui.set_text_dialog_open(true);
        ui.set_text_message(
            "这是通过 LocalSend Next 发送的示例文本。它适用于快速分享地址、会议摘要或短链接。"
                .into(),
        );
    }
    if state == "troubleshoot" {
        ui.set_troubleshoot_dialog_open(true);
    }
    if state == "auto-save-help" {
        ui.set_auto_save_help_dialog_open(true);
    }
    if state == "language" {
        ui.set_language_dialog_open(true);
    }
    if state == "web-share" {
        ui.set_web_share_dialog_open(true);
        ui.set_web_share_running(true);
        ui.set_web_share_url("https://localsend.local/receive?pin=4821".into());
        ui.set_settings_encryption(true);
    }
}

fn seed_send(ui: &AppWindow, state: &str, english: bool) {
    if matches!(
        state,
        "default" | "dense" | "manual" | "favorites" | "text" | "text-dialog"
    ) {
        seed_default_devices(ui);
    }
    if state == "text" {
        seed_default_devices(ui);
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
        seed_default_devices(ui);
        ui.set_selected_files(ModelRc::new(VecModel::from(vec![
            SelectedFileItem {
                path: "/tmp/产品视觉规范.pdf".into(),
                name: "产品视觉规范.pdf".into(),
                is_directory: false,
                is_text: false,
                is_image: false,
                size: "4.0 MiB".into(),
            },
            SelectedFileItem {
                path: "/tmp/receive-default.png".into(),
                name: "receive-default.png".into(),
                is_directory: false,
                is_text: false,
                is_image: true,
                size: "2.4 MiB".into(),
            },
            SelectedFileItem {
                path: "/tmp/send-selected.png".into(),
                name: "send-selected.png".into(),
                is_directory: false,
                is_text: false,
                is_image: true,
                size: "2.3 MiB".into(),
            },
        ])));
        ui.set_has_selection(true);
        ui.set_selected_file_count(3);
        ui.set_selected_total_size("共 8.7 MiB".into());
        ui.set_selected_files_text(
            "产品视觉规范.pdf、receive-default.png、send-selected.png".into(),
        );
        ui.set_selected_target_label("Pixel 9".into());
    }
    if state == "dense" || state == "multi" {
        seed_dense_devices(ui);
        ui.set_selected_files(ModelRc::new(VecModel::from(vec![
            SelectedFileItem {
                path: "/tmp/产品视觉规范.pdf".into(),
                name: "产品视觉规范.pdf".into(),
                is_directory: false,
                is_text: false,
                is_image: false,
                size: "4.0 MiB".into(),
            },
            SelectedFileItem {
                path: "/tmp/receive-default.png".into(),
                name: "receive-default.png".into(),
                is_directory: false,
                is_text: false,
                is_image: true,
                size: "2.4 MiB".into(),
            },
            SelectedFileItem {
                path: "/tmp/send-selected.png".into(),
                name: "send-selected.png".into(),
                is_directory: false,
                is_text: false,
                is_image: true,
                size: "2.3 MiB".into(),
            },
        ])));
        ui.set_has_selection(true);
        ui.set_selected_file_count(3);
        ui.set_selected_total_size("共 8.7 MiB".into());
        ui.set_selected_files_text(
            "产品视觉规范.pdf、receive-default.png、send-selected.png".into(),
        );
        ui.set_selected_target_label(
            if english {
                "0 devices selected"
            } else {
                "已选择 0 台设备"
            }
            .into(),
        );
        ui.set_send_mode(if state == "multi" { 1 } else { 0 });
    }
    if state == "transferring" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "send-transfer-1".into(),
            peer: "Flutter Reference".into(),
            summary: "2 / 3 个文件 · 5.3 MiB / 8.7 MiB".into(),
            status: "传输中".into(),
            speed: "18.6 MB/s".into(),
            duration: "".into(),
            direction: "发送".into(),
            terminal: false,
            progress_percent: 58,
            file_count: 3,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(vec![
                TransferFileItem {
                    name: "产品视觉规范.pdf".into(),
                    size: "4.0 MiB".into(),
                    is_text: false,
                    is_image: false,
                    status: "58%".into(),
                    detail: "正在传输".into(),
                    progress_percent: 58,
                    completed: false,
                    in_progress: true,
                },
                TransferFileItem {
                    name: "receive-default.png".into(),
                    size: "2.4 MiB".into(),
                    is_text: false,
                    is_image: true,
                    status: "58%".into(),
                    detail: "正在传输".into(),
                    progress_percent: 58,
                    completed: false,
                    in_progress: true,
                },
                TransferFileItem {
                    name: "send-selected.png".into(),
                    size: "2.3 MiB".into(),
                    is_text: false,
                    is_image: true,
                    status: "待处理".into(),
                    detail: "等待中".into(),
                    progress_percent: 0,
                    completed: false,
                    in_progress: false,
                },
            ])),
        }])));
        ui.set_active_transfer_count(1);
        ui.set_sending_active(true);
    }
    if state == "completed" {
        ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "send-transfer-completed".into(),
            peer: "Flutter Reference".into(),
            summary: "8.7 MiB".into(),
            status: "已完成".into(),
            speed: "".into(),
            duration: "3.1 s".into(),
            direction: "发送".into(),
            terminal: true,
            progress_percent: 100,
            file_count: 3,
            completion_countdown: 3,
            files: ModelRc::new(VecModel::from(vec![
                TransferFileItem {
                    name: "产品视觉规范.pdf".into(),
                    size: "4.0 MiB".into(),
                    is_text: false,
                    is_image: false,
                    status: "已完成".into(),
                    detail: "已完成".into(),
                    progress_percent: 100,
                    completed: true,
                    in_progress: false,
                },
                TransferFileItem {
                    name: "receive-default.png".into(),
                    size: "2.4 MiB".into(),
                    is_text: false,
                    is_image: true,
                    status: "已完成".into(),
                    detail: "已完成".into(),
                    progress_percent: 100,
                    completed: true,
                    in_progress: false,
                },
                TransferFileItem {
                    name: "send-selected.png".into(),
                    size: "2.3 MiB".into(),
                    is_text: false,
                    is_image: true,
                    status: "已完成".into(),
                    detail: "已完成".into(),
                    progress_percent: 100,
                    completed: true,
                    in_progress: false,
                },
            ])),
        }])));
        ui.set_active_transfer_count(1);
        ui.set_sending_active(true);
    }
}

fn seed_default_devices(ui: &AppWindow) {
    ui.set_devices(ModelRc::new(VecModel::from(vec![
        DeviceItem {
            alias: "Flutter Reference".into(),
            ip: "macOS 15".into(),
            fingerprint: "reference".into(),
            protocol: "HTTP".into(),
            model: "MacBook Pro".into(),
            device_type: "Desktop".into(),
            favorite: true,
            selected: false,
        },
        DeviceItem {
            alias: "iPhone 15 Pro".into(),
            ip: "iOS 18.2".into(),
            fingerprint: "iphone".into(),
            protocol: "HTTPS".into(),
            model: "iPhone 15 Pro".into(),
            device_type: "Mobile".into(),
            favorite: true,
            selected: false,
        },
        DeviceItem {
            alias: "Studio Mac".into(),
            ip: "macOS 15".into(),
            fingerprint: "studio".into(),
            protocol: "HTTP".into(),
            model: "Mac Studio".into(),
            device_type: "Monitor".into(),
            favorite: false,
            selected: false,
        },
        DeviceItem {
            alias: "Pixel 9".into(),
            ip: "Android 16".into(),
            fingerprint: "pixel".into(),
            protocol: "HTTPS".into(),
            model: "Pixel 9".into(),
            device_type: "Mobile".into(),
            favorite: false,
            selected: false,
        },
    ])));
}

fn seed_dense_devices(ui: &AppWindow) {
    ui.set_devices(ModelRc::new(VecModel::from(vec![
        DeviceItem {
            alias: "Flutter Reference".into(),
            ip: "macOS 15".into(),
            fingerprint: "reference".into(),
            protocol: "HTTPS".into(),
            model: "MacBook Pro".into(),
            device_type: "Desktop".into(),
            favorite: true,
            selected: false,
        },
        DeviceItem {
            alias: "iPhone 15 Pro".into(),
            ip: "iOS 18.2".into(),
            fingerprint: "iphone".into(),
            protocol: "HTTPS".into(),
            model: "iPhone 15 Pro".into(),
            device_type: "Mobile".into(),
            favorite: true,
            selected: false,
        },
        DeviceItem {
            alias: "Studio Mac".into(),
            ip: "macOS 15".into(),
            fingerprint: "studio".into(),
            protocol: "HTTP".into(),
            model: "Mac Studio".into(),
            device_type: "Monitor".into(),
            favorite: false,
            selected: false,
        },
        DeviceItem {
            alias: "Pixel 9".into(),
            ip: "Android 16".into(),
            fingerprint: "pixel".into(),
            protocol: "HTTPS".into(),
            model: "Pixel 9".into(),
            device_type: "Mobile".into(),
            favorite: false,
            selected: false,
        },
        DeviceItem {
            alias: "Design Server".into(),
            ip: "Linux".into(),
            fingerprint: "design-server".into(),
            protocol: "HTTPS".into(),
            model: "Framework".into(),
            device_type: "Monitor".into(),
            favorite: false,
            selected: false,
        },
        DeviceItem {
            alias: "客厅 iPad".into(),
            ip: "iPadOS 19".into(),
            fingerprint: "living-room-ipad".into(),
            protocol: "HTTPS".into(),
            model: "iPad Air".into(),
            device_type: "Mobile".into(),
            favorite: false,
            selected: false,
        },
        DeviceItem {
            alias: "QA Windows".into(),
            ip: "Windows 11".into(),
            fingerprint: "qa-windows".into(),
            protocol: "HTTPS".into(),
            model: "ThinkPad".into(),
            device_type: "Desktop".into(),
            favorite: false,
            selected: false,
        },
        DeviceItem {
            alias: "Build Agent".into(),
            ip: "Ubuntu 26.04".into(),
            fingerprint: "build-agent".into(),
            protocol: "HTTP".into(),
            model: "NUC".into(),
            device_type: "Monitor".into(),
            favorite: false,
            selected: false,
        },
    ])));
}

fn seed_settings(ui: &AppWindow, _language: &str) {
    ui.set_settings_device_type("Desktop".into());
    ui.set_settings_device_model("MacBook Pro".into());
    ui.set_settings_advanced(false);
}
