use localsend_platform::GuiAction;
use std::sync::OnceLock;

static ACTION_TX: OnceLock<std::sync::mpsc::Sender<GuiAction>> = OnceLock::new();

pub fn init(sender: std::sync::mpsc::Sender<GuiAction>) {
    let _ = ACTION_TX.set(sender);
    platform::init();
}

pub fn show(action: GuiAction, title: &str, body: &str, action_label: &str) {
    platform::show(action, title, body, action_label);
}

pub fn clear_delivered() {
    platform::clear_delivered();
}

fn dispatch_action(action: GuiAction) {
    if let Some(sender) = ACTION_TX.get() {
        let _ = sender.send(action);
    }
}

#[cfg(test)]
mod tests {
    use super::GuiAction;
    use std::path::PathBuf;

    fn round_trip(action: GuiAction) -> GuiAction {
        let payload = serde_json::to_vec(&action).expect("serialize GUI action");
        serde_json::from_slice(&payload).expect("deserialize GUI action")
    }

    #[test]
    fn notification_actions_round_trip_message_payload() {
        let action = GuiAction::OpenMessage {
            sender: "sender".to_string(),
            device: "device".to_string(),
            text: "hello\nworld".to_string(),
        };
        assert_eq!(round_trip(action.clone()), action);
    }

    #[test]
    fn notification_actions_round_trip_folder_payload() {
        let action = GuiAction::OpenReceiveFolder {
            path: PathBuf::from("/tmp/received"),
        };
        assert_eq!(round_trip(action.clone()), action);
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::GuiAction;
    use block2::StackBlock;
    use objc2::rc::Retained;
    use objc2::{MainThreadOnly, define_class, msg_send};
    use objc2_foundation::{MainThreadMarker, NSDictionary, NSObject, NSObjectProtocol, NSString};
    use objc2_user_notifications::{
        UNAuthorizationOptions, UNMutableNotificationContent, UNNotification,
        UNNotificationPresentationOptions, UNNotificationRequest, UNNotificationResponse,
        UNNotificationSound, UNUserNotificationCenter, UNUserNotificationCenterDelegate,
    };
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicU64, Ordering};

    const ACTION_PAYLOAD_KEY: &str = "localsend.gui_action";
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);

    thread_local! {
        static RUNTIME: RefCell<Option<NotificationRuntime>> = const { RefCell::new(None) };
    }

    struct NotificationRuntime {
        center: Retained<UNUserNotificationCenter>,
        _delegate: Retained<NotificationDelegate>,
    }

    fn authorization_options() -> UNAuthorizationOptions {
        UNAuthorizationOptions::Alert
            | UNAuthorizationOptions::Sound
            | UNAuthorizationOptions::Badge
    }

    fn foreground_presentation_options() -> UNNotificationPresentationOptions {
        UNNotificationPresentationOptions::Banner
            | UNNotificationPresentationOptions::List
            | UNNotificationPresentationOptions::Sound
    }

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "LocalSendTrayNotificationDelegate"]
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
                completion_handler.call((foreground_presentation_options(),));
            }

            #[unsafe(method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:))]
            fn did_receive(
                &self,
                _center: &UNUserNotificationCenter,
                response: &UNNotificationResponse,
                completion_handler: &block2::DynBlock<dyn Fn()>,
            ) {
                let request = response.notification().request();
                let content = request.content();
                let user_info = content.userInfo();
                let key = NSString::from_str(ACTION_PAYLOAD_KEY);
                let payload = unsafe { user_info.objectForKey_unchecked(&key) }
                    .and_then(|value| value.downcast_ref::<NSString>())
                    .map(ToString::to_string);
                let action = payload.and_then(|payload| serde_json::from_str(&payload).ok());
                if let Some(action) = action {
                    super::dispatch_action(action);
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

    pub fn init() {
        tracing::debug!("notification runtime init");
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
            let completion = StackBlock::new(
                |granted: objc2::runtime::Bool, error: *mut objc2_foundation::NSError| {
                    tracing::debug!(
                        granted = granted.as_bool(),
                        error = !error.is_null(),
                        "notification authorization response"
                    );
                },
            );
            center.requestAuthorizationWithOptions_completionHandler(
                authorization_options(),
                &completion,
            );
            *runtime.borrow_mut() = Some(NotificationRuntime {
                center,
                _delegate: delegate,
            });
        });
    }

    pub fn show(action: GuiAction, title: &str, body: &str, _action_label: &str) {
        tracing::debug!(title, "submitting desktop notification");
        let Some(payload) = serde_json::to_string(&action).ok() else {
            return;
        };
        let identifier = format!("localsend-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed));
        RUNTIME.with(|runtime| {
            let runtime = runtime.borrow();
            let Some(runtime) = runtime.as_ref() else {
                return;
            };
            let content = UNMutableNotificationContent::new();
            content.setTitle(&NSString::from_str(title));
            content.setBody(&NSString::from_str(body));
            let sound = UNNotificationSound::defaultSound();
            content.setSound(Some(&sound));
            let key = NSString::from_str(ACTION_PAYLOAD_KEY);
            let value = NSString::from_str(&payload);
            let user_info = NSDictionary::from_slices(&[&*key], &[&*value]);
            let user_info: Retained<NSDictionary> = unsafe { Retained::cast_unchecked(user_info) };
            unsafe { content.setUserInfo(&user_info) };
            let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
                &NSString::from_str(&identifier),
                &content,
                None,
            );
            let completion = StackBlock::new(|error: *mut objc2_foundation::NSError| {
                if error.is_null() {
                    tracing::debug!("desktop notification submitted");
                } else {
                    tracing::warn!("desktop notification submission failed");
                }
            });
            runtime
                .center
                .addNotificationRequest_withCompletionHandler(&request, Some(&completion));
        });
    }

    pub fn clear_delivered() {
        RUNTIME.with(|runtime| {
            if let Some(runtime) = runtime.borrow().as_ref() {
                runtime.center.removeAllDeliveredNotifications();
            }
        });
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn notifications_request_and_present_with_sound_enabled() {
            assert!(authorization_options().contains(UNAuthorizationOptions::Sound));
            assert!(
                foreground_presentation_options()
                    .contains(UNNotificationPresentationOptions::Sound)
            );
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod platform {
    use super::{GuiAction, dispatch_action};
    use notify_rust::{Notification, NotificationResponse};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, OnceLock};
    use std::thread;
    use tokio::sync::oneshot;

    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    static ACTIVE: OnceLock<Mutex<HashMap<u64, oneshot::Sender<()>>>> = OnceLock::new();

    fn active_notifications() -> &'static Mutex<HashMap<u64, oneshot::Sender<()>>> {
        ACTIVE.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn response_opens_notification(response: &NotificationResponse) -> bool {
        match response {
            NotificationResponse::Default => true,
            NotificationResponse::Action(action) => action == "open",
            _ => false,
        }
    }

    fn build_linux_notification(title: &str, body: &str, action_label: &str) -> Notification {
        let mut notification = Notification::new();
        notification
            .appname("LocalSend")
            .summary(title)
            .body(body)
            .action("open", action_label)
            .sound_name("message-new-instant");
        notification
    }

    pub fn init() {
        tracing::debug!("notification runtime init");
    }

    pub fn show(action: GuiAction, title: &str, body: &str, action_label: &str) {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let (cancel, cancelled) = oneshot::channel();
        active_notifications()
            .lock()
            .expect("notification registry poisoned")
            .insert(id, cancel);

        let title = title.to_string();
        let body = body.to_string();
        let action_label = action_label.to_string();
        let spawn = thread::Builder::new()
            .name("localsend-linux-notification".to_string())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        tracing::warn!(%error, "could not create notification runtime");
                        active_notifications()
                            .lock()
                            .expect("notification registry poisoned")
                            .remove(&id);
                        return;
                    }
                };

                let result = runtime.block_on(async {
                    let notification = build_linux_notification(&title, &body, &action_label);
                    let handle = notification
                        .show_async()
                        .await
                        .map_err(|error| error.to_string())?;
                    tokio::select! {
                        _ = handle.wait_for_action_async(|response: &NotificationResponse| {
                            if response_opens_notification(response) {
                                dispatch_action(action.clone());
                            }
                        }) => {}
                        _ = cancelled => handle.close_async().await,
                    }
                    Ok::<(), String>(())
                });
                if let Err(error) = result {
                    tracing::warn!(%error, "could not show Linux notification");
                }
                active_notifications()
                    .lock()
                    .expect("notification registry poisoned")
                    .remove(&id);
            });
        if let Err(error) = spawn {
            tracing::warn!(%error, "could not spawn Linux notification thread");
            active_notifications()
                .lock()
                .expect("notification registry poisoned")
                .remove(&id);
        }
    }

    pub fn clear_delivered() {
        let senders = {
            let mut active = active_notifications()
                .lock()
                .expect("notification registry poisoned");
            active.drain().map(|(_, sender)| sender).collect::<Vec<_>>()
        };
        for sender in senders {
            let _ = sender.send(());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use notify_rust::CloseReason;

        #[test]
        fn default_and_open_actions_are_accepted() {
            assert!(response_opens_notification(&NotificationResponse::Default));
            assert!(response_opens_notification(&NotificationResponse::Action(
                "open".to_string()
            )));
            assert!(!response_opens_notification(&NotificationResponse::Closed(
                CloseReason::Dismissed
            )));
        }

        #[test]
        fn notification_builder_uses_message_sound_and_open_action() {
            let notification = build_linux_notification("Title", "Body", "Open");
            assert_eq!(notification.summary, "Title");
            assert_eq!(notification.body, "Body");
            assert_eq!(notification.actions, vec!["open", "Open"]);
            assert!(notification.hints.contains(&notify_rust::Hint::SoundName(
                "message-new-instant".to_string()
            )));
        }
    }
}

#[cfg(windows)]
mod platform {
    use super::{GuiAction, dispatch_action};
    use localsend_platform::{NOTIFICATION_APP_ID, clear_notification_history};
    use notify_rust::{Notification, NotificationResponse};
    use std::thread;

    fn response_opens_notification(response: &NotificationResponse) -> bool {
        match response {
            NotificationResponse::Default => true,
            NotificationResponse::Action(action) => action == "open",
            _ => false,
        }
    }

    pub fn init() {
        if let Err(error) = localsend_platform::ensure_notification_identity() {
            tracing::warn!(%error, "could not register Windows notification identity");
        }
    }

    pub fn show(action: GuiAction, title: &str, body: &str, action_label: &str) {
        let mut notification = Notification::new();
        notification
            .appname("LocalSend")
            .app_id(NOTIFICATION_APP_ID)
            .summary(title)
            .body(body)
            .action("open", action_label)
            .sound_name("Default");
        let handle = match notification.show() {
            Ok(handle) => handle,
            Err(error) => {
                tracing::warn!(%error, "could not show Windows notification");
                return;
            }
        };
        let spawned = thread::Builder::new()
            .name("localsend-windows-notification".to_string())
            .spawn(move || {
                let result = handle.wait_for_response(|response: &NotificationResponse| {
                    if response_opens_notification(response) {
                        dispatch_action(action.clone());
                    }
                });
                if let Err(error) = result {
                    tracing::warn!(%error, "Windows notification response failed");
                }
            });
        if let Err(error) = spawned {
            tracing::warn!(%error, "could not spawn Windows notification thread");
        }
    }

    pub fn clear_delivered() {
        if let Err(error) = clear_notification_history() {
            tracing::warn!(%error, "could not clear Windows notification history");
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use notify_rust::CloseReason;

        #[test]
        fn default_and_open_actions_are_accepted() {
            assert!(response_opens_notification(&NotificationResponse::Default));
            assert!(response_opens_notification(&NotificationResponse::Action(
                "open".to_string()
            )));
            assert!(!response_opens_notification(&NotificationResponse::Closed(
                CloseReason::Dismissed
            )));
        }
    }
}

#[cfg(not(any(target_os = "macos", unix, windows)))]
mod platform {
    use super::GuiAction;

    pub fn init() {}
    pub fn show(_action: GuiAction, _title: &str, _body: &str, _action_label: &str) {}
    pub fn clear_delivered() {}
}
