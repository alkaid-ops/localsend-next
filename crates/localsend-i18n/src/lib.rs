//! Translations for strings built in Rust rather than in the Slint markup:
//! status messages, history rows, tray menu items and notifications.
//!
//! The table is generated from the same `.po` catalogs the Slint UI bundles,
//! so every language covers both surfaces. Regenerate with
//! `node scripts/update-slint-translations.mjs`.

include!("lookup_generated.rs");

/// The msgid language: every source string is written in Chinese.
pub const DEFAULT_LANGUAGE: &str = "zh-CN";

/// Translate `message` for `language` ("zh-CN", "de", ...). Unknown messages
/// and the msgid language itself fall back to the source text.
pub fn translate(language: &str, message: &str) -> String {
    if language == DEFAULT_LANGUAGE || language.is_empty() {
        return message.to_string();
    }
    lookup(language, message).unwrap_or_else(|| message.to_string())
}

/// Translate a message template and fill its placeholders, so the values land
/// where the target language puts them. The msgid uses `{}` placeholders; a
/// translation may reorder them (`{1} … {0}`) or repeat them.
///
/// ```ignore
/// t(lang, "复制失败：{}", &[&error.to_string()]);
/// ```
pub fn t(language: &str, message: &str, args: &[&str]) -> String {
    let template = if language == DEFAULT_LANGUAGE || language.is_empty() {
        message.to_string()
    } else {
        lookup(language, message).unwrap_or_else(|| message.to_string())
    };
    fill(&template, args)
}

fn lookup(language: &str, message: &str) -> Option<String> {
    let tag = resolve_tag(language);
    TABLE
        .iter()
        .find(|(entry, _)| *entry == tag)
        .and_then(|(_, entries)| {
            entries
                .iter()
                .find(|(msgid, _)| *msgid == message)
                .map(|(_, translated)| (*translated).to_string())
        })
}

/// Replace `{}` (positional, in order) or `{0}`/`{1}` (explicit) placeholders.
fn fill(template: &str, args: &[&str]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut next = 0usize;
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let Some(close) = after.find('}') else {
            out.push_str(&rest[open..]);
            return out;
        };
        let index = after[..close].trim();
        let value = if index.is_empty() {
            let value = args.get(next).copied().unwrap_or("");
            next += 1;
            value
        } else {
            index
                .parse::<usize>()
                .ok()
                .and_then(|position| args.get(position).copied())
                .unwrap_or("")
        };
        out.push_str(value);
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Resolve any locale spelling ("de", "de-DE", "zh_CN", "pt_BR") to a bundled
/// tag, matching the exact tag first and then the language part.
pub fn resolve_tag(language: &str) -> &'static str {
    let wanted = language.replace('_', "-");
    if let Some((tag, _)) = TABLE.iter().find(|(tag, _)| *tag == wanted) {
        return tag;
    }
    let base = wanted
        .split(['-', '@'])
        .next()
        .unwrap_or_default()
        .to_lowercase();
    TABLE
        .iter()
        .find(|(tag, _)| tag.split('-').next() == Some(base.as_str()))
        .map(|(tag, _)| *tag)
        .unwrap_or("en-US")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_count(tag: &str) -> usize {
        TABLE
            .iter()
            .find(|(entry, _)| *entry == tag)
            .map(|(_, entries)| entries.len())
            .unwrap_or(0)
    }

    #[test]
    fn resolves_locale_spellings_to_bundled_tags() {
        assert_eq!(resolve_tag("de"), "de");
        assert_eq!(resolve_tag("de-DE"), "de");
        assert_eq!(resolve_tag("pt_BR"), "pt-BR");
        assert_eq!(resolve_tag("zh-CN"), "zh-CN");
        assert_eq!(resolve_tag("xx-YY"), "en-US");
    }

    #[test]
    fn translates_known_messages_and_keeps_unknown_ones() {
        assert_ne!(translate("de", "打开主界面"), "打开主界面");
        assert_eq!(translate("zh-CN", "打开主界面"), "打开主界面");
        assert_eq!(
            translate("de", "this message is not in the catalog"),
            "this message is not in the catalog"
        );
    }

    #[test]
    fn templates_fill_placeholders_in_translation_order() {
        let filled = t("de", "复制失败：{}", &["boom"]);
        assert!(filled.contains("boom"), "{filled}");
        assert!(!filled.contains("{}"), "{filled}");

        // Explicit positions and repeated placeholders both work.
        assert_eq!(fill("{} and {0}", &["x"]), "x and x");
        assert_eq!(fill("{1} before {0}", &["a", "b"]), "b before a");
    }

    #[test]
    fn every_language_covers_the_same_msgids() {
        let reference = entry_count("en-US");
        assert!(reference > 0);
        for (tag, entries) in TABLE {
            assert_eq!(
                entries.len(),
                reference,
                "{tag} has a different msgid count"
            );
            assert!(
                entries
                    .iter()
                    .all(|(id, value)| !id.is_empty() && !value.is_empty()),
                "{tag} has an empty msgid or translation"
            );
        }
    }
}
