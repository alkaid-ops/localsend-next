//! The interface language selected for this GUI instance, shared by the Slint
//! catalog selection and the Rust-side message lookups in `localsend-i18n`.

thread_local! {
    static CURRENT_LANGUAGE: std::cell::Cell<Option<&'static str>> = const { std::cell::Cell::new(None) };
}

pub(crate) fn current_language() -> &'static str {
    CURRENT_LANGUAGE.with(|language| language.get().unwrap_or(localsend_i18n::DEFAULT_LANGUAGE))
}

pub(crate) fn set_current_language(tag: &'static str) {
    CURRENT_LANGUAGE.with(|language| language.set(Some(tag)));
}

/// Translate a template for the active language and fill its placeholders.
pub(crate) fn translate_args(template: &str, args: &[String]) -> String {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    localsend_i18n::t(current_language(), template, &args)
}

/// Render one macro argument with `Display`, which covers strings, numbers and
/// error types alike.
pub(crate) fn template_arg<T: std::fmt::Display>(value: &T) -> String {
    value.to_string()
}
