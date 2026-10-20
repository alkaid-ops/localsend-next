use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Size, Task, Theme};
use localsend_rpc::{DaemonClient, SendFileRequest, ServiceCommand, ServiceSnapshot};
use tokio::runtime::Runtime;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tab {
    Receive,
    Send,
    Settings,
}

#[derive(Debug, Clone)]
enum Message {
    SelectTab(Tab),
    ToggleServer,
    PathChanged(String),
    SendPath,
}

struct DesktopApp {
    runtime: Runtime,
    client: DaemonClient,
    tab: Tab,
    send_path: String,
    status: String,
    snapshot: Option<ServiceSnapshot>,
}

impl DesktopApp {
    fn new() -> (Self, Task<Message>) {
        let runtime = Runtime::new().expect("Tokio runtime");
        let client = runtime
            .block_on(DaemonClient::connect_interactive())
            .expect("connect to localsendd");
        let snapshot = client.snapshot();
        (
            Self {
                runtime,
                client,
                tab: Tab::Receive,
                send_path: String::new(),
                status: "Ready".to_string(),
                snapshot,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectTab(tab) => self.tab = tab,
            Message::ToggleServer => {
                let running = self
                    .snapshot
                    .as_ref()
                    .map(|snapshot| snapshot.server_running)
                    .unwrap_or(false);
                let command = if running {
                    ServiceCommand::StopServer
                } else {
                    ServiceCommand::StartServer
                };
                self.status = match self.runtime.block_on(self.client.command(command)) {
                    Ok(_) => "Server command submitted".to_string(),
                    Err(error) => error,
                };
                self.snapshot = self.client.snapshot();
            }
            Message::PathChanged(path) => self.send_path = path,
            Message::SendPath => {
                let Some(device) = self
                    .snapshot
                    .as_ref()
                    .and_then(|snapshot| snapshot.devices.first().cloned())
                else {
                    self.status = "No nearby device".to_string();
                    return Task::none();
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
                self.status = match self.runtime.block_on(self.client.command(command)) {
                    Ok(_) => "Send command submitted".to_string(),
                    Err(error) => error,
                };
                self.snapshot = self.client.snapshot();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let tabs = row![
            button("接收").on_press(Message::SelectTab(Tab::Receive)),
            button("发送").on_press(Message::SelectTab(Tab::Send)),
            button("设置").on_press(Message::SelectTab(Tab::Settings)),
            button("启动 / 停止").on_press(Message::ToggleServer),
            text(self.status.as_str()),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let content: Element<'_, Message> = match self.tab {
            Tab::Receive => {
                let mut body = column![text("接收").size(28)].spacing(10);
                if let Some(snapshot) = &self.snapshot {
                    body = body.push(text(format!(
                        "服务器：{}，端口：{}",
                        if snapshot.server_running {
                            "运行中"
                        } else {
                            "已停止"
                        },
                        snapshot.server_port.unwrap_or_default()
                    )));
                    body = body.push(text(format!(
                        "活动传输：{}，最近历史：{}",
                        snapshot.active_transfers.len(),
                        snapshot.history.len()
                    )));
                }
                body.into()
            }
            Tab::Send => {
                let mut body = column![text("发送").size(28)].spacing(10);
                if let Some(snapshot) = &self.snapshot {
                    body = body.push(text(format!("附近设备：{}", snapshot.devices.len())));
                    for device in snapshot.devices.iter().take(8) {
                        body = body.push(text(format!(
                            "{} · {}",
                            device.device.alias,
                            device.device.ip.as_deref().unwrap_or("?")
                        )));
                    }
                }
                body = body.push(
                    row![
                        text_input("文件或目录路径", self.send_path.as_str())
                            .on_input(Message::PathChanged)
                            .width(Length::Fill),
                        button("发送路径").on_press(Message::SendPath),
                    ]
                    .spacing(8),
                );
                body.into()
            }
            Tab::Settings => {
                let mut body = column![text("设置").size(28)].spacing(10);
                if let Some(snapshot) = &self.snapshot {
                    let settings = &snapshot.settings;
                    body = body.push(text(format!("设备名称：{}", settings.alias)));
                    body = body.push(text(format!("端口：{}", settings.port)));
                    body = body.push(text(format!("协议：{}", settings.protocol)));
                    body = body.push(text(format!(
                        "PIN：{}",
                        settings.pin.as_deref().unwrap_or("无")
                    )));
                }
                body.into()
            }
        };

        container(column![tabs, scrollable(content)].spacing(16).padding(16))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(DesktopApp::new, DesktopApp::update, DesktopApp::view)
        .title("LocalSend PC")
        .theme(Theme::Dark)
        .window(iced::window::Settings {
            size: Size::new(980.0, 700.0),
            ..Default::default()
        })
        .run()
}
