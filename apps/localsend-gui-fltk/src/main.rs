use fltk::button::Button;
use fltk::frame::Frame;
use fltk::group::{Group, Tabs};
use fltk::input::Input;
use fltk::prelude::*;
use fltk::window::Window;
use localsend_rpc::{DaemonClient, SendFileRequest, ServiceCommand};
use std::rc::Rc;
use tokio::runtime::Runtime;

fn main() -> anyhow::Result<()> {
    let runtime = Rc::new(Runtime::new()?);
    let client = runtime.block_on(DaemonClient::connect_interactive())?;

    let app = fltk::app::App::default();
    fltk::app::set_scheme(fltk::app::Scheme::Gtk);

    let mut window = Window::default()
        .with_size(980, 700)
        .with_label("LocalSend PC");
    window.set_color(fltk::enums::Color::White);

    let tabs = Tabs::default().with_pos(0, 0).with_size(980, 700);
    tabs.begin();

    let receive = Group::default().with_label("接收");
    receive.begin();
    Frame::default()
        .with_pos(32, 64)
        .with_size(260, 40)
        .with_label("LocalSend PC");
    let mut status = Frame::default()
        .with_pos(32, 118)
        .with_size(880, 80)
        .with_label("正在连接 daemon...");
    status.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);
    let mut toggle = Button::default()
        .with_pos(32, 220)
        .with_size(140, 36)
        .with_label("启动 / 停止");
    let mut refresh = Button::default()
        .with_pos(188, 220)
        .with_size(160, 36)
        .with_label("刷新状态");
    receive.end();

    let send = Group::default().with_label("发送");
    send.begin();
    Frame::default()
        .with_pos(32, 64)
        .with_size(260, 40)
        .with_label("附近设备");
    let mut devices = Frame::default()
        .with_pos(32, 110)
        .with_size(880, 220)
        .with_label("暂无设备");
    devices.set_align(
        fltk::enums::Align::Left | fltk::enums::Align::Inside | fltk::enums::Align::Wrap,
    );
    let mut path = Input::default()
        .with_pos(32, 360)
        .with_size(700, 36)
        .with_label("");
    path.set_label("文件或目录路径");
    let mut submit = Button::default()
        .with_pos(744, 360)
        .with_size(140, 36)
        .with_label("发送路径");
    send.end();

    let settings = Group::default().with_label("设置");
    settings.begin();
    Frame::default()
        .with_pos(32, 64)
        .with_size(300, 40)
        .with_label("基础设置");
    let mut settings_text = Frame::default()
        .with_pos(32, 116)
        .with_size(880, 360)
        .with_label("正在读取设置...");
    settings_text.set_align(
        fltk::enums::Align::Left | fltk::enums::Align::Inside | fltk::enums::Align::Wrap,
    );
    settings.end();

    tabs.end();
    window.end();
    window.make_resizable(true);
    window.show();

    let timer_client = client.clone();
    let mut timer_status = status.clone();
    fltk::app::add_timeout3(0.5, move |handle| {
        if let Some(snapshot) = timer_client.snapshot() {
            timer_status.set_label(&format!(
                "服务器：{}    端口：{}\n活动传输：{}    历史：{}",
                if snapshot.server_running {
                    "运行中"
                } else {
                    "已停止"
                },
                snapshot.server_port.unwrap_or_default(),
                snapshot.active_transfers.len(),
                snapshot.history.len()
            ));
            devices.set_label(
                &snapshot
                    .devices
                    .iter()
                    .map(|device| {
                        format!(
                            "{} · {}",
                            device.device.alias,
                            device.device.ip.as_deref().unwrap_or("?")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
            let current = &snapshot.settings;
            settings_text.set_label(&format!(
                "设备名称：{}\n端口：{}\n协议：{}\nPIN：{}\n接收目录：{}",
                current.alias,
                current.port,
                current.protocol,
                current.pin.as_deref().unwrap_or("无"),
                current
                    .destination_dir
                    .as_deref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "默认".to_string())
            ));
        }
        fltk::app::repeat_timeout3(0.5, handle);
    });

    let toggle_runtime = runtime.clone();
    let toggle_client = client.clone();
    let mut toggle_state = status.clone();
    toggle.set_callback(move |_| {
        let running = toggle_client
            .snapshot()
            .map(|snapshot| snapshot.server_running)
            .unwrap_or(false);
        let command = if running {
            ServiceCommand::StopServer
        } else {
            ServiceCommand::StartServer
        };
        match toggle_runtime.block_on(toggle_client.command(command)) {
            Ok(_) => toggle_state.set_label("服务器命令已提交"),
            Err(error) => toggle_state.set_label(&error),
        }
    });

    let refresh_client = client.clone();
    refresh.set_callback(move |_| {
        if let Some(snapshot) = refresh_client.snapshot() {
            status.set_label(&format!(
                "服务器：{}    端口：{}",
                if snapshot.server_running {
                    "运行中"
                } else {
                    "已停止"
                },
                snapshot.server_port.unwrap_or_default()
            ));
        }
    });

    let submit_runtime = runtime.clone();
    let submit_client = client.clone();
    submit.set_callback(move |_| {
        let Some(device) = submit_client
            .snapshot()
            .and_then(|snapshot| snapshot.devices.first().cloned())
        else {
            return;
        };
        let command = ServiceCommand::SendFiles {
            target_fingerprint: Some(device.device.fingerprint),
            target: None,
            files: vec![SendFileRequest {
                path: path.value().into(),
                file_id: None,
                sha256: None,
                metadata: None,
            }],
            pin: None,
        };
        let _ = submit_runtime.block_on(submit_client.command(command));
    });

    app.run()?;
    Ok(())
}
