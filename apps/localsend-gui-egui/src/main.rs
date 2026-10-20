use eframe::egui::{self, Context, FontData, FontDefinitions, FontFamily, ViewportBuilder};
use localsend_rpc::{DaemonClient, SendFileRequest, ServiceCommand, ServiceSnapshot};
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tab {
    Receive,
    Send,
    Settings,
}

struct DesktopApp {
    runtime: Runtime,
    client: DaemonClient,
    tab: Tab,
    send_path: String,
    message: String,
}

impl DesktopApp {
    fn new(context: &eframe::CreationContext<'_>) -> Self {
        if let Ok(font) = std::env::var("LOCALSEND_BENCH_FONT") {
            if font != "none" {
                install_cjk_font(&context.egui_ctx, Some(&font));
            }
        } else {
            install_cjk_font(&context.egui_ctx, None);
        }
        let runtime = Runtime::new().expect("Tokio runtime");
        let client = runtime
            .block_on(DaemonClient::connect_interactive())
            .expect("connect to localsendd");
        Self {
            runtime,
            client,
            tab: Tab::Receive,
            send_path: String::new(),
            message: "Ready".to_string(),
        }
    }

    fn snapshot(&self) -> Option<ServiceSnapshot> {
        self.client.snapshot()
    }

    fn toggle_server(&mut self, running: bool) {
        let command = if running {
            ServiceCommand::StopServer
        } else {
            ServiceCommand::StartServer
        };
        self.message = match self.runtime.block_on(self.client.command(command)) {
            Ok(_) => "Server command submitted".to_string(),
            Err(error) => error,
        };
    }

    fn send_path(&mut self) {
        let Some(device) = self
            .snapshot()
            .and_then(|snapshot| snapshot.devices.first().cloned())
        else {
            self.message = "No nearby device".to_string();
            return;
        };
        let command = ServiceCommand::SendFiles {
            target_fingerprint: Some(device.device.fingerprint),
            target: None,
            files: vec![SendFileRequest {
                path: self.send_path.clone().into(),
                file_id: None,
                sha256: None,
                metadata: None,
            }],
            pin: None,
        };
        self.message = match self.runtime.block_on(self.client.command(command)) {
            Ok(_) => "Send command submitted".to_string(),
            Err(error) => error,
        };
    }
}

impl eframe::App for DesktopApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let snapshot = self.snapshot();
        let running = snapshot
            .as_ref()
            .map(|snapshot| snapshot.server_running)
            .unwrap_or(false);
        let mut toggle_server = false;
        let mut send_path = false;

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Tab::Receive, "接收");
            ui.selectable_value(&mut self.tab, Tab::Send, "发送");
            ui.selectable_value(&mut self.tab, Tab::Settings, "设置");
            ui.separator();
            if ui.button("启动 / 停止").clicked() {
                toggle_server = true;
            }
            ui.label(self.message.as_str());
        });
        ui.separator();

        match self.tab {
            Tab::Receive => {
                ui.heading("接收");
                if let Some(snapshot) = &snapshot {
                    ui.label(format!(
                        "服务器：{}，端口：{}",
                        if snapshot.server_running {
                            "运行中"
                        } else {
                            "已停止"
                        },
                        snapshot.server_port.unwrap_or_default()
                    ));
                    ui.label(format!("活动传输：{}", snapshot.active_transfers.len()));
                    ui.label(format!("最近历史：{}", snapshot.history.len()));
                } else {
                    ui.label("等待 daemon snapshot");
                }
            }
            Tab::Send => {
                ui.heading("发送");
                if let Some(snapshot) = &snapshot {
                    ui.label(format!("附近设备：{}", snapshot.devices.len()));
                    for device in snapshot.devices.iter().take(8) {
                        ui.label(format!(
                            "{} · {}",
                            device.device.alias,
                            device.device.ip.as_deref().unwrap_or("?")
                        ));
                    }
                }
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.send_path);
                    if ui.button("发送路径").clicked() {
                        send_path = true;
                    }
                });
            }
            Tab::Settings => {
                ui.heading("设置");
                if let Some(snapshot) = &snapshot {
                    let settings = &snapshot.settings;
                    ui.label(format!("设备名称：{}", settings.alias));
                    ui.label(format!("端口：{}", settings.port));
                    ui.label(format!("协议：{}", settings.protocol));
                    ui.label(format!("PIN：{}", settings.pin.as_deref().unwrap_or("无")));
                    ui.label(format!(
                        "接收目录：{}",
                        settings
                            .destination_dir
                            .as_deref()
                            .map(Path::display)
                            .map(|path| path.to_string())
                            .unwrap_or_else(|| "默认".to_string())
                    ));
                }
            }
        }

        if toggle_server {
            self.toggle_server(running);
        }
        if send_path {
            self.send_path();
        }
        ui.ctx().request_repaint_after(Duration::from_millis(500));
    }
}

fn install_cjk_font(context: &Context, requested: Option<&str>) {
    let candidates = [
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
        r"C:\Windows\Fonts\msyh.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
    ];
    let paths: Vec<&str> = requested
        .map(|path| vec![path])
        .unwrap_or_else(|| candidates.to_vec());
    for candidate in paths {
        let Ok(bytes) = fs::read(candidate) else {
            continue;
        };
        let mut fonts = FontDefinitions::default();
        fonts
            .font_data
            .insert("cjk".to_string(), FontData::from_owned(bytes).into());
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .insert(0, "cjk".to_string());
        }
        context.set_fonts(fonts);
        return;
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("LocalSend PC")
            .with_inner_size([980.0, 700.0])
            .with_min_inner_size([760.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "LocalSend PC",
        options,
        Box::new(|context| Ok(Box::new(DesktopApp::new(context)))),
    )
}
