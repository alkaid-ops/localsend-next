pub mod app;
mod clipboard_payload;
pub mod daemon_worker;
mod runtime_strings;

pub mod view_model;

slint::include_modules!();

/// Translate a Rust-side message template and fill its `{}` placeholders:
/// `t!("无法打开文件：{}", error)`.
///
/// The template is translated first, so a language that orders the values
/// differently still reads correctly. Use this instead of `format!` for any
/// user-visible message with a value in it.
#[macro_export]
macro_rules! t {
    ($template:expr $(, $argument:expr)* $(,)?) => {{
        $crate::runtime_strings::translate_args(
            $template,
            &[$($crate::runtime_strings::template_arg(&$argument)),*],
        )
    }};
}

/// Language picker contents for the reference renderer. The app builds the same
/// list in `app::language_items`, but examples cannot reach a private helper.
pub fn reference_language_items(english: bool) -> Vec<(String, String)> {
    let mut items = vec![(
        "system".to_string(),
        (if english { "System" } else { "跟随系统" }).to_string(),
    )];
    items.extend(
        [
            ("ar", "العربية"),
            ("az", "Azərbaycanca"),
            ("be", "Беларуская"),
            ("bg", "Български"),
            ("bn", "বাংলা"),
            ("ca", "Català"),
            ("cs", "Čeština"),
            ("da", "Dansk"),
            ("de", "Deutsch"),
            ("el", "Ελληνικά"),
            ("en-US", "English"),
            ("en-IN", "English (India)"),
            ("es-ES", "Español"),
            ("et", "Eesti"),
            ("eu", "Euskara"),
            ("fa", "فارسی"),
            ("fi", "Suomi"),
            ("fil-PH", "Filipino"),
            ("fr", "Français"),
            ("ga", "Gaeilge"),
            ("gl", "Galego"),
            ("gu", "ગુજરાતી"),
            ("he", "עברית"),
            ("hi", "हिन्दी"),
            ("hu", "Magyar"),
            ("hy", "Հայերեն"),
            ("id", "Bahasa Indonesia"),
            ("it", "Italiano"),
            ("ja", "日本語"),
            ("km", "ខ្មែរ"),
            ("ko", "한국어"),
            ("ky", "Кыргызча"),
            ("lo", "ລາວ"),
            ("ml", "മലയാളം"),
            ("mn", "Монгол"),
            ("ms", "Bahasa Melayu"),
            ("ne", "नेपाली"),
            ("nl", "Nederlands"),
            ("pl", "Polski"),
            ("pt-BR", "Português (Brasil)"),
            ("pt-PT", "Português (Portugal)"),
            ("ro", "Română"),
            ("ru", "Русский"),
            ("si", "සිංහල"),
            ("sk", "Slovenčina"),
            ("sl", "Slovenščina"),
            ("sr", "Српски"),
            ("sr-Cyrl", "Српски (ћирилица)"),
            ("sv", "Svenska"),
            ("ta", "தமிழ்"),
            ("th", "ไทย"),
            ("tr", "Türkçe"),
            ("uk", "Українська"),
            ("ur", "اردو"),
            ("uz", "Oʻzbekcha"),
            ("vi", "Tiếng Việt"),
            ("zh-CN", "简体中文"),
            ("zh-HK", "繁體中文（香港）"),
            ("zh-TW", "繁體中文（台灣）"),
        ]
        .into_iter()
        .map(|(code, name)| (code.to_string(), name.to_string())),
    );
    items
}

/// Display name for a bundled tag, as the language picker shows it.
pub fn reference_language_name(tag: &str) -> String {
    reference_language_items(false)
        .into_iter()
        .find(|(code, _)| code == tag)
        .map(|(_, name)| name)
        .unwrap_or_else(|| "简体中文".to_string())
}
