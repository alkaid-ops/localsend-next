use i_slint_backend_testing::{AccessibleRole, ElementQuery};
use localsend_gui_slint::{AppWindow, DeviceItem};
use slint::platform::{PointerEventButton, WindowEvent};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn device_tile_receives_physical_pointer_click() {
    i_slint_backend_testing::init_integration_test_with_mock_time();
    let ui = AppWindow::new().expect("create window");
    ui.window().set_size(slint::LogicalSize::new(1200.0, 900.0));
    ui.set_selected_tab("send".into());
    ui.set_devices(ModelRc::new(VecModel::from(vec![DeviceItem {
        alias: "Peer".into(),
        ip: "192.168.1.20".into(),
        fingerprint: "peer-fingerprint".into(),
        protocol: "HTTPS".into(),
        model: "Desktop".into(),
        device_type: "Desktop".into(),
        favorite: false,
        selected: false,
    }])));
    let clicked = Rc::new(RefCell::new(Vec::<String>::new()));
    ui.on_send_to_device({
        let clicked = clicked.clone();
        move |fingerprint| clicked.borrow_mut().push(fingerprint.to_string())
    });
    ui.show().expect("show window");
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));

    let tile = ElementQuery::from_root(&ui)
        .match_accessible_role(AccessibleRole::Button)
        .match_predicate(|element| {
            element
                .accessible_label()
                .is_some_and(|label| label.starts_with("Peer, 192.168.1.20"))
        })
        .find_first()
        .expect("device tile");
    tile.mock_single_click(PointerEventButton::Left);
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    assert_eq!(clicked.borrow().as_slice(), ["peer-fingerprint"]);

    let origin = tile.absolute_position();
    let size = tile.size();
    let text_position =
        slint::LogicalPosition::new(origin.x + size.width * 0.35, origin.y + size.height * 0.4);
    ui.window().dispatch_event(WindowEvent::PointerMoved {
        position: text_position,
    });
    ui.window().dispatch_event(WindowEvent::PointerPressed {
        position: text_position,
        button: PointerEventButton::Left,
    });
    ui.window().dispatch_event(WindowEvent::PointerReleased {
        position: text_position,
        button: PointerEventButton::Left,
    });
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(50));
    assert_eq!(
        clicked.borrow().as_slice(),
        ["peer-fingerprint", "peer-fingerprint"]
    );
}
