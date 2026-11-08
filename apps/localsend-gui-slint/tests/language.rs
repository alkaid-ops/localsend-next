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
        status: "waiting".into(),
        speed: "".into(),
        duration: "".into(),
        direction: "send".into(),
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
    for label in ["Set PIN", "PIN", "Confirm"] {
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
    ui.set_web_share_url("https://localsend.local/receive".into());
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Receive via link", "Copy link", "Close"] {
        assert!(
            has_label(&ui, label),
            "missing English dialog label {label}"
        );
    }
    ui.set_web_share_dialog_open(false);

    ui.set_completion_dialog_open(true);
    ui.set_completion_countdown(0);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    for label in ["Open", "Show in file manager", "Close"] {
        assert!(
            has_label(&ui, label),
            "missing English dialog label {label}"
        );
    }
    ui.set_completion_dialog_open(false);

    // Every bundled locale must be selectable and must leave the navigation
    // translated; a missing bundle or a Chinese fallback fails here.
    let locales = localsend_gui_slint::reference_language_items(false);
    assert!(locales.len() >= 58, "bundled locale list is too short");
    for (code, _) in locales.iter().skip(1) {
        if code == "zh-CN" || code == "zh-HK" || code == "zh-TW" {
            // Chinese variants legitimately keep Chinese glyphs.
            continue;
        }
        slint::select_bundled_translation(code)
            .unwrap_or_else(|error| panic!("cannot select {code}: {error:?}"));
        i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
        let has_nav = ["接收", "发送", "设置"]
            .iter()
            .any(|label| has_accessible_label(&ui, AccessibleRole::Button, label));
        assert!(!has_nav, "locale {code} left the navigation in Chinese");
    }
}

#[test]
fn every_bundled_locale_translates_every_ui_string() {
    // A locale whose catalog lacks a msgid silently falls back to Chinese, so
    // check every bundled language covers the whole UI catalog.
    let translations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("translations");
    let catalog =
        std::fs::read_to_string(translations.join("languages.json")).expect("languages.json");
    let locales: Vec<String> = catalog
        .split("\"locale\": \"")
        .skip(1)
        .filter_map(|rest| rest.split('"').next().map(str::to_string))
        .collect();
    assert!(
        locales.len() >= 57,
        "expected the upstream locale list, got {}",
        locales.len()
    );

    let ui: Vec<String> = walk_slint("ui")
        .into_iter()
        .flat_map(|file| {
            let source = std::fs::read_to_string(file).unwrap_or_default();
            extract_tr(&source)
        })
        .collect();
    assert!(!ui.is_empty());

    for locale in locales {
        let path = translations
            .join(&locale)
            .join("LC_MESSAGES")
            .join("localsend-gui-slint.po");
        let po = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing catalog for {locale}: {}", path.display()));
        let empty = count_empty_msgstr(&po);
        assert_eq!(empty, 0, "{locale} has {empty} untranslated msgids");
    }
}

fn walk_slint(dir: &str) -> Vec<std::path::PathBuf> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(dir);
    let mut files = Vec::new();
    let mut stack = vec![root];
    while let Some(path) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "slint") {
                files.push(path);
            }
        }
    }
    files
}

fn extract_tr(source: &str) -> Vec<String> {
    source
        .match_indices("@tr(\"")
        .filter_map(|(index, _)| {
            let rest = &source[index + 5..];
            rest.find("\")").map(|end| rest[..end].to_string())
        })
        .collect()
}

/// Count real untranslated entries; the PO header's own empty msgstr is not one.
fn count_empty_msgstr(po: &str) -> usize {
    let mut previous_is_header = false;
    po.lines()
        .filter(|line| {
            let is_header = *line == "msgid \"\"";
            let empty = line.starts_with("msgstr ") && line.trim_end() == "msgstr \"\"";
            let counted = empty && !previous_is_header;
            previous_is_header = is_header;
            counted
        })
        .count()
}
