use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const ACTION_PAYLOAD_KEY: &str = "localsend.action";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum NotificationAction {
    Message {
        sender: String,
        device: String,
        text: String,
    },
    OpenReceiveFolder(PathBuf),
}

fn encode_action(action: &NotificationAction) -> Option<String> {
    serde_json::to_string(action).ok()
}

fn decode_action(payload: &str) -> Option<NotificationAction> {
    serde_json::from_str(payload).ok()
}

pub fn init(sender: std::sync::mpsc::Sender<NotificationAction>) {
    platform::init(sender);
}

pub fn show(action: NotificationAction, title: &str, body: &str) -> bool {
    platform::show(action, title, body)
}

#[cfg(target_os = "macos")]
mod platform {
    use super::{ACTION_PAYLOAD_KEY, NotificationAction};
    use block2::StackBlock;
    use objc2::rc::Retained;
    use objc2::{MainThreadOnly, define_class, msg_send};
    use objc2_foundation::{MainThreadMarker, NSDictionary, NSObject, NSObjectProtocol, NSString};
    use objc2_user_notifications::{
        UNAuthorizationOptions, UNMutableNotificationContent, UNNotification,
        UNNotificationPresentationOptions, UNNotificationRequest, UNNotificationResponse,
        UNUserNotificationCenter, UNUserNotificationCenterDelegate,
    };
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, OnceLock, mpsc};

    static ACTIONS: OnceLock<Mutex<HashMap<String, NotificationAction>>> = OnceLock::new();
    static ACTION_TX: OnceLock<mpsc::Sender<NotificationAction>> = OnceLock::new();
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);

    thread_local! {
        static RUNTIME: RefCell<Option<NotificationRuntime>> = const { RefCell::new(None) };
    }

    struct NotificationRuntime {
        center: Retained<UNUserNotificationCenter>,
        _delegate: Retained<NotificationDelegate>,
    }

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "LocalSendUserNotificationDelegate"]
        struct NotificationDelegate;

        unsafe impl NSObjectProtocol for NotificationDelegate {}

        unsafe impl UNUserNotificationCenterDelegate for NotificationDelegate {
            #[unsafe(method(userNotificationCenter:willPresentNotification:withCompletionHandler:))]
            fn will_present(
                &self,
                _center: &UNUserNotificationCenter,
                _notification: &UNNotification,
                completion_handler: &block2::DynBlock<dyn Fn(UNNotificationPresentationOptions)>,
            ) {
                completion_handler.call((UNNotificationPresentationOptions::Banner
                    | UNNotificationPresentationOptions::List,));
            }

            #[unsafe(method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:))]
            fn did_receive(
                &self,
                _center: &UNUserNotificationCenter,
                response: &UNNotificationResponse,
                completion_handler: &block2::DynBlock<dyn Fn()>,
            ) {
                let request = response.notification().request();
                let identifier = request.identifier().to_string();
                let action = actions()
                    .lock()
                    .ok()
                    .and_then(|mut actions| actions.remove(&identifier))
                    .or_else(|| {
                        let key = NSString::from_str(ACTION_PAYLOAD_KEY);
                        let user_info = request.content().userInfo();
                        let payload = unsafe { user_info.objectForKey_unchecked(&key) }
                            .and_then(|value| value.downcast_ref::<NSString>())
                            .map(ToString::to_string)?;
                        super::decode_action(&payload)
                    });
                if let Some(action) = action
                    && let Some(sender) = ACTION_TX.get()
                {
                    let _ = sender.send(action);
                }
                completion_handler.call(());
            }
        }
    );

    impl NotificationDelegate {
        fn new(mtm: MainThreadMarker) -> Retained<Self> {
            let this = Self::alloc(mtm).set_ivars(());
            unsafe { msg_send![super(this), init] }
        }
    }

    pub fn init(sender: mpsc::Sender<NotificationAction>) {
        let _ = ACTION_TX.set(sender);
        let _ = actions();
        RUNTIME.with(|runtime| {
            if runtime.borrow().is_some() {
                return;
            }
            let Some(mtm) = MainThreadMarker::new() else {
                return;
            };
            let delegate = NotificationDelegate::new(mtm);
            let center = UNUserNotificationCenter::currentNotificationCenter();
            center.setDelegate(Some(objc2::runtime::ProtocolObject::from_ref(&*delegate)));
            let authorization = UNAuthorizationOptions::Alert
                | UNAuthorizationOptions::Sound
                | UNAuthorizationOptions::Badge;
            let completion = StackBlock::new(
                |_granted: objc2::runtime::Bool, _error: *mut objc2_foundation::NSError| {},
            );
            center.requestAuthorizationWithOptions_completionHandler(authorization, &completion);
            *runtime.borrow_mut() = Some(NotificationRuntime {
                center,
                _delegate: delegate,
            });
        });
    }

    pub fn show(action: NotificationAction, title: &str, body: &str) -> bool {
        show_inner(action, title, body).is_some()
    }

    fn show_inner(action: NotificationAction, title: &str, body: &str) -> Option<String> {
        let identifier = format!("localsend-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
        let payload = super::encode_action(&action)?;
        actions().lock().ok()?.insert(identifier.clone(), action);

        RUNTIME.with(|runtime| {
            let runtime = runtime.borrow();
            let runtime = runtime.as_ref()?;
            let content = UNMutableNotificationContent::new();
            content.setTitle(&NSString::from_str(title));
            content.setBody(&NSString::from_str(body));
            let key = NSString::from_str(ACTION_PAYLOAD_KEY);
            let value = NSString::from_str(&payload);
            let user_info = NSDictionary::from_slices(&[&*key], &[&*value]);
            let user_info: Retained<NSDictionary> = unsafe { Retained::cast_unchecked(user_info) };
            // SAFETY: the dictionary contains only NSString keys and values.
            unsafe { content.setUserInfo(&user_info) };
            let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
                &NSString::from_str(&identifier),
                &content,
                None,
            );
            runtime
                .center
                .addNotificationRequest_withCompletionHandler(&request, None);
            Some(identifier.clone())
        })
    }

    fn actions() -> &'static Mutex<HashMap<String, NotificationAction>> {
        ACTIONS.get_or_init(|| Mutex::new(HashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::{NotificationAction, decode_action, encode_action};
    use std::path::PathBuf;

    #[test]
    fn notification_message_action_round_trips() {
        let action = NotificationAction::Message {
            sender: "Cool Melon".into(),
            device: "MacBook Pro".into(),
            text: "line one\nline two".into(),
        };
        let encoded = encode_action(&action).expect("message action should encode");
        let decoded = decode_action(&encoded).expect("message action should decode");
        assert!(matches!(
            decoded,
            NotificationAction::Message { sender, device, text }
                if sender == "Cool Melon" && device == "MacBook Pro" && text == "line one\nline two"
        ));
    }

    #[test]
    fn notification_folder_action_round_trips() {
        let action = NotificationAction::OpenReceiveFolder(PathBuf::from("/tmp/LocalSend"));
        let encoded = encode_action(&action).expect("folder action should encode");
        let decoded = decode_action(&encoded).expect("folder action should decode");
        assert!(matches!(
            decoded,
            NotificationAction::OpenReceiveFolder(path) if path == std::path::Path::new("/tmp/LocalSend")
        ));
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    use super::NotificationAction;

    pub fn init(_sender: std::sync::mpsc::Sender<NotificationAction>) {}

    pub fn show(_action: NotificationAction, _title: &str, _body: &str) -> bool {
        false
    }
}
