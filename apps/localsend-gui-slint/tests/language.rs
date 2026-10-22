use i_slint_backend_testing::{AccessibleRole, ElementQuery};
use localsend_gui_slint::{AppWindow, TransferItem};
use slint::{ComponentHandle, ModelRc, VecModel};

fn has_accessible_label(ui: &AppWindow, role: AccessibleRole, label: &'static str) -> bool {
    ElementQuery::from_root(ui)
        .match_accessible_role(role)
        .match_predicate(|element| element.accessible_label().as_deref() == Some(label))
        .find_first()
        .is_some()
}

fn has_label(ui: &AppWindow, label: &'static str) -> bool {
    ElementQuery::from_root(ui)
        .match_predicate(|element| element.accessible_label().as_deref() == Some(label))
        .find_first()
        .is_some()
}

fn has_label_contains(ui: &AppWindow, needle: &'static str) -> bool {
    ElementQuery::from_root(ui)
        .match_predicate(move |element| {
            element
                .accessible_label()
                .is_some_and(|label| label.contains(needle))
        })
        .find_first()
        .is_some()
}

#[test]
fn bundled_translation_switches_navigation_at_runtime() {
    i_slint_backend_testing::init_integration_test_with_mock_time();
    let ui = AppWindow::new().expect("create window");
    ui.show().expect("show window");
    slint::select_bundled_translation("zh-CN").expect("select Chinese bundle");
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    assert!(has_accessible_label(&ui, AccessibleRole::Button, "接收"));
    assert!(has_accessible_label(&ui, AccessibleRole::Button, "发送"));
    assert!(has_accessible_label(&ui, AccessibleRole::Button, "设置"));

    slint::select_bundled_translation("en-US").expect("select English bundle");
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    assert!(has_accessible_label(&ui, AccessibleRole::Button, "Receive"));
    assert!(has_accessible_label(&ui, AccessibleRole::Button, "Send"));
    assert!(has_accessible_label(
        &ui,
        AccessibleRole::Button,
        "Settings"
    ));

    ui.set_selected_tab("send".into());
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["File", "Folder", "Text", "Clipboard"] {
        assert!(has_label(&ui, label), "missing English label {label}");
    }
    ui.set_transfer_items(ModelRc::new(VecModel::from(vec![TransferItem {
        transfer_id: "test-transfer".into(),
        peer: "Peer".into(),
        summary: "1 KiB".into(),
        status: "等待中".into(),
        direction: "发送".into(),
        terminal: false,
        progress_percent: 0,
        file_count: 1,
        completion_countdown: 0,
        files: ModelRc::new(VecModel::from(
            Vec::<localsend_gui_slint::TransferFileItem>::new(),
        )),
    }])));
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    assert!(has_label_contains(&ui, "Waiting"));

    ui.set_selected_tab("settings".into());
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Theme", "Color", "Language"] {
        assert!(has_label(&ui, label), "missing English label {label}");
    }

    ui.set_pin_dialog_open(true);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Enter receiver PIN", "PIN value", "Retry"] {
        assert!(
            has_label(&ui, label),
            "missing English dialog label {label}"
        );
    }
    ui.set_pin_dialog_open(false);

    ui.set_setting_dialog_open(true);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Edit setting", "Setting value", "Save"] {
        assert!(
            has_label(&ui, label),
            "missing English dialog label {label}"
        );
    }
    ui.set_setting_dialog_open(false);

    ui.set_web_share_dialog_open(true);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Receive via link", "Encryption", "Auto accept requests"] {
        assert!(
            has_label(&ui, label),
            "missing English dialog label {label}"
        );
    }
    ui.set_web_share_dialog_open(false);

    ui.set_completion_dialog_open(true);
    ui.set_completion_countdown(0);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Open file", "Show in file manager", "Close"] {
        assert!(
            has_label(&ui, label),
            "missing English dialog label {label}"
        );
    }
}
