use localsend_platform::GuiAction;

pub fn init(sender: std::sync::mpsc::Sender<GuiAction>) {
    platform::init(sender);
}

pub fn show(action: GuiAction, title: &str, body: &str) {
    platform::show(action, title, body);
}

pub fn clear_delivered() {
    platform::clear_delivered();
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
    use std::sync::{OnceLock, mpsc};

    const ACTION_PAYLOAD_KEY: &str = "localsend.gui_action";
    static ACTION_TX: OnceLock<mpsc::Sender<GuiAction>> = OnceLock::new();
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

    pub fn init(sender: mpsc::Sender<GuiAction>) {
        tracing::debug!("notification runtime init");
        let _ = ACTION_TX.set(sender);
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

    pub fn show(action: GuiAction, title: &str, body: &str) {
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

#[cfg(not(target_os = "macos"))]
mod platform {
    use super::GuiAction;

    pub fn init(_sender: std::sync::mpsc::Sender<GuiAction>) {}
    pub fn show(_action: GuiAction, _title: &str, _body: &str) {}
    pub fn clear_delivered() {}
}
