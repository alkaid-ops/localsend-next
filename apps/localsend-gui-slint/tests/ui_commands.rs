use i_slint_backend_testing::{ElementHandle, ElementQuery};
use localsend_gui_slint::app::{AppOptions, build_app};
use localsend_gui_slint::daemon_worker::UiCommand;
use localsend_gui_slint::{AppWindow, DeviceItem, TransferFileItem, TransferItem};
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::path::PathBuf;
use std::sync::Once;
use std::sync::mpsc as std_mpsc;
use std::time::Duration;

static INIT_BACKEND: Once = Once::new();

struct TestApp {
    ui: AppWindow,
    commands: tokio::sync::mpsc::UnboundedReceiver<UiCommand>,
    _worker_events: std_mpsc::Sender<localsend_gui_slint::daemon_worker::WorkerEvent>,
}

fn app_with_inputs(files: Vec<PathBuf>, text: Option<String>) -> TestApp {
    INIT_BACKEND.call_once(i_slint_backend_testing::init_integration_test_with_mock_time);
    let (command_tx, commands) = tokio::sync::mpsc::unbounded_channel();
    let (event_tx, event_rx) = std_mpsc::channel();
    let mut options = AppOptions::for_test("send");
    options.initial_files = files;
    options.initial_text = text;
    let ui = build_app(command_tx, event_rx, options).expect("build app");
    ui.window().set_size(slint::LogicalSize::new(1200.0, 900.0));
    ui.set_devices(ModelRc::new(VecModel::from(vec![DeviceItem {
        alias: "Pointer Peer".into(),
        ip: "192.168.1.20".into(),
        fingerprint: "pointer-peer".into(),
        protocol: "HTTPS".into(),
        model: "Desktop".into(),
        device_type: "Desktop".into(),
        favorite: false,
        selected: false,
    }])));
    ui.show().expect("show app");
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
    TestApp {
        ui,
        commands,
        _worker_events: event_tx,
    }
}

fn element_by_id(ui: &AppWindow, id: &str) -> ElementHandle {
    let label = id.to_string();
    let needle = id.to_string();
    ElementQuery::from_root(ui)
        .match_predicate(move |element| element.accessible_id().as_deref() == Some(needle.as_str()))
        .find_first()
        .unwrap_or_else(|| panic!("missing accessible-id {label}"))
}

fn click_position(ui: &AppWindow, position: slint::LogicalPosition) {
    ui.window()
        .dispatch_event(WindowEvent::PointerMoved { position });
    ui.window().dispatch_event(WindowEvent::PointerPressed {
        position,
        button: PointerEventButton::Left,
    });
    ui.window().dispatch_event(WindowEvent::PointerReleased {
        position,
        button: PointerEventButton::Left,
    });
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
}

fn next_command(receiver: &mut tokio::sync::mpsc::UnboundedReceiver<UiCommand>) -> UiCommand {
    receiver.try_recv().expect("expected UiCommand")
}

#[test]
fn gui_pointer_and_action_command_matrix() {
    device_card_pointer_regions_emit_send_text();
    device_card_emits_send_files_with_selected_path();
    device_action_buttons_do_not_cross_fire();
    send_progress_buttons_dismiss_transfer();
    send_progress_cancel_button_cancels_active_transfer();
    send_progress_advanced_button_opens_diagnostics();
}

fn send_progress_buttons_dismiss_transfer() {
    for id in ["send-progress-back", "send-progress-finish"] {
        let mut app = app_with_inputs(Vec::new(), None);
        app.ui
            .set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
                transfer_id: "failed-transfer".into(),
                peer: "Pointer Peer".into(),
                summary: "0 B / 1 B".into(),
                status: "失败".into(),
                direction: "发送".into(),
                terminal: true,
                progress_percent: 0,
                file_count: 1,
                completion_countdown: 3,
                files: ModelRc::new(VecModel::from(vec![TransferFileItem {
                    name: "message.txt".into(),
                    size: "1 B".into(),
                    is_text: true,
                }])),
            }])));
        app.ui.set_sending_active(true);
        i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
        element_by_id(&app.ui, id).mock_single_click(PointerEventButton::Left);
        i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
        assert!(matches!(
            next_command(&mut app.commands),
            UiCommand::DismissTransfer(transfer_id) if transfer_id == "failed-transfer"
        ));
    }
}

fn send_progress_cancel_button_cancels_active_transfer() {
    let mut app = app_with_inputs(Vec::new(), None);
    app.ui
        .set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "active-transfer".into(),
            peer: "Pointer Peer".into(),
            summary: "0 B / 1 B".into(),
            status: "传输中".into(),
            direction: "发送".into(),
            terminal: false,
            progress_percent: 0,
            file_count: 1,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(vec![TransferFileItem {
                name: "sample.txt".into(),
                size: "1 B".into(),
                is_text: false,
            }])),
        }])));
    app.ui.set_sending_active(true);
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
    element_by_id(&app.ui, "send-progress-finish").mock_single_click(PointerEventButton::Left);
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
    assert!(matches!(
        next_command(&mut app.commands),
        UiCommand::CancelTransfer(transfer_id) if transfer_id == "active-transfer"
    ));
}

fn send_progress_advanced_button_opens_diagnostics() {
    let app = app_with_inputs(Vec::new(), None);
    app.ui
        .set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
            transfer_id: "failed-transfer".into(),
            peer: "Pointer Peer".into(),
            summary: "0 B / 1 B".into(),
            status: "失败".into(),
            direction: "发送".into(),
            terminal: true,
            progress_percent: 0,
            file_count: 1,
            completion_countdown: 0,
            files: ModelRc::new(VecModel::from(Vec::<TransferFileItem>::new())),
        }])));
    app.ui.set_sending_active(true);
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
    element_by_id(&app.ui, "send-progress-advanced").mock_single_click(PointerEventButton::Left);
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
    assert!(app.ui.get_troubleshoot_dialog_open());
}

fn device_card_pointer_regions_emit_send_text() {
    for fraction in [0.12_f32, 0.35, 0.68] {
        let mut app = app_with_inputs(Vec::new(), Some("pointer text".to_string()));
        assert_eq!(app.ui.get_selected_file_count(), 1);
        let selected = app.ui.get_selected_files();
        assert_eq!(selected.row_count(), 1);
        assert!(selected.row_data(0).expect("text row").is_text);
        let card = element_by_id(&app.ui, "device-card-pointer-peer");
        let origin = card.absolute_position();
        let size = card.size();
        click_position(
            &app.ui,
            slint::LogicalPosition::new(
                origin.x + size.width * fraction,
                origin.y + size.height / 2.0,
            ),
        );
        match next_command(&mut app.commands) {
            UiCommand::SendText { fingerprint, text } => {
                assert_eq!(fingerprint, "pointer-peer");
                assert_eq!(text, "pointer text");
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }
}

fn device_card_emits_send_files_with_selected_path() {
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("pointer.txt");
    std::fs::write(&path, b"pointer file").expect("write file");
    let mut app = app_with_inputs(vec![path.clone()], None);
    element_by_id(&app.ui, "device-card-pointer-peer").mock_single_click(PointerEventButton::Left);
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(50));
    match next_command(&mut app.commands) {
        UiCommand::SendFiles { fingerprint, paths } => {
            assert_eq!(fingerprint, "pointer-peer");
            assert_eq!(paths, vec![path]);
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

fn device_action_buttons_do_not_cross_fire() {
    let directory = tempfile::tempdir().expect("tempdir");
    let path = directory.path().join("pointer.txt");
    std::fs::write(&path, b"pointer file").expect("write file");

    let mut info_app = app_with_inputs(vec![path.clone()], None);
    element_by_id(&info_app.ui, "device-info-pointer-peer")
        .mock_single_click(PointerEventButton::Left);
    assert!(info_app.commands.try_recv().is_err());

    let mut favorite_app = app_with_inputs(vec![path.clone()], None);
    element_by_id(&favorite_app.ui, "device-favorite-pointer-peer")
        .mock_single_click(PointerEventButton::Left);
    assert!(matches!(
        next_command(&mut favorite_app.commands),
        UiCommand::ToggleFavorite(fingerprint) if fingerprint == "pointer-peer"
    ));

    let mut send_app = app_with_inputs(vec![path.clone()], None);
    element_by_id(&send_app.ui, "device-send-pointer-peer")
        .mock_single_click(PointerEventButton::Left);
    assert!(matches!(
        next_command(&mut send_app.commands),
        UiCommand::SendFiles { fingerprint, paths }
            if fingerprint == "pointer-peer" && paths == vec![path]
    ));
}
