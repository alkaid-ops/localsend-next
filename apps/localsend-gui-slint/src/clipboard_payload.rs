use arboard::ImageData;
use chrono::{DateTime, Local};
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

const CACHE_SUBDIR: &str = "clipboard";
const IMAGE_PREFIX: &str = "clipboard_";
const IMAGE_MAX_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);
static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ClipboardPayload {
    Files(Vec<PathBuf>),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ClipboardError {
    Unavailable,
    Empty,
    CacheUnavailable,
    ImageEncode(String),
    ImageWrite(String),
}

impl ClipboardError {
    pub(crate) fn user_message(&self, english: bool) -> &'static str {
        match (self, english) {
            (Self::Unavailable, true) => "Cannot access the clipboard",
            (Self::Unavailable, false) => "无法访问剪贴板",
            (Self::Empty, true) => "No file, folder, image, or text in clipboard",
            (Self::Empty, false) => "剪贴板中没有可发送的文件、文件夹、图片或文本",
            (Self::CacheUnavailable, true) => "Cannot locate the clipboard image cache",
            (Self::CacheUnavailable, false) => "无法定位剪贴板图片缓存目录",
            (Self::ImageEncode(_), true) => "Cannot convert the clipboard image to PNG",
            (Self::ImageEncode(_), false) => "无法将剪贴板图片转换为 PNG",
            (Self::ImageWrite(_), true) => "Cannot save the clipboard image",
            (Self::ImageWrite(_), false) => "无法保存剪贴板图片",
        }
    }
}

trait ClipboardReader {
    fn read_text(&mut self) -> Option<String>;
    fn read_image(&mut self) -> Option<ImageData<'static>>;
    fn read_files(&mut self) -> Vec<PathBuf>;
}

struct ArboardReader(arboard::Clipboard);

impl ClipboardReader for ArboardReader {
    fn read_text(&mut self) -> Option<String> {
        self.0.get_text().ok()
    }

    fn read_image(&mut self) -> Option<ImageData<'static>> {
        self.0.get_image().ok()
    }

    fn read_files(&mut self) -> Vec<PathBuf> {
        self.0.get().file_list().unwrap_or_default()
    }
}

pub(crate) fn read_clipboard() -> Result<ClipboardPayload, ClipboardError> {
    let clipboard = arboard::Clipboard::new().map_err(|_| ClipboardError::Unavailable)?;
    let cache = clipboard_cache_dir().ok_or(ClipboardError::CacheUnavailable)?;
    read_with(ArboardReader(clipboard), &cache, Local::now())
}

fn read_with(
    mut clipboard: impl ClipboardReader,
    cache_dir: &Path,
    now: DateTime<Local>,
) -> Result<ClipboardPayload, ClipboardError> {
    if let Some(text) = clipboard.read_text()
        && !text.trim().is_empty()
    {
        return Ok(ClipboardPayload::Text(text));
    }

    if let Some(image) = clipboard.read_image() {
        return write_image_png(image, cache_dir, now)
            .map(|path| ClipboardPayload::Files(vec![path]));
    }

    let files = clipboard.read_files();
    if !files.is_empty() {
        return Ok(ClipboardPayload::Files(files));
    }

    Err(ClipboardError::Empty)
}

fn clipboard_cache_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("org", "LocalSend", "localsend-rs")
        .map(|paths| paths.cache_dir().join(CACHE_SUBDIR))
}

fn write_image_png(
    image: ImageData<'static>,
    directory: &Path,
    now: DateTime<Local>,
) -> Result<PathBuf, ClipboardError> {
    let width = u32::try_from(image.width)
        .map_err(|_| ClipboardError::ImageEncode("image width overflows u32".to_string()))?;
    let height = u32::try_from(image.height)
        .map_err(|_| ClipboardError::ImageEncode("image height overflows u32".to_string()))?;
    let expected_len = image
        .width
        .checked_mul(image.height)
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or_else(|| ClipboardError::ImageEncode("image dimensions overflow".to_string()))?;
    if width == 0 || height == 0 || image.bytes.len() != expected_len {
        return Err(ClipboardError::ImageEncode(
            "clipboard image is not valid RGBA data".to_string(),
        ));
    }

    fs::create_dir_all(directory).map_err(|error| ClipboardError::ImageWrite(error.to_string()))?;
    let destination = unique_image_path(directory, now)?;
    let temporary = directory.join(format!(
        ".{}.{}.{}.tmp",
        destination
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("clipboard"),
        std::process::id(),
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));

    let write_result = (|| {
        let file = File::create(&temporary)
            .map_err(|error| ClipboardError::ImageWrite(error.to_string()))?;
        let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|error| ClipboardError::ImageEncode(error.to_string()))?;
        writer
            .write_image_data(image.bytes.as_ref())
            .map_err(|error| ClipboardError::ImageEncode(error.to_string()))?;
        writer
            .finish()
            .map_err(|error| ClipboardError::ImageWrite(error.to_string()))?;
        fs::rename(&temporary, &destination)
            .map_err(|error| ClipboardError::ImageWrite(error.to_string()))
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    write_result.map(|()| destination)
}

fn unique_image_path(directory: &Path, now: DateTime<Local>) -> Result<PathBuf, ClipboardError> {
    let stem = format!("{IMAGE_PREFIX}{}", now.format("%Y-%m-%d_%H-%M"));
    for suffix in 0..1000 {
        let name = if suffix == 0 {
            format!("{stem}.png")
        } else {
            format!("{stem}_{suffix}.png")
        };
        let path = directory.join(name);
        if !path.exists() {
            return Ok(path);
        }
    }
    Err(ClipboardError::ImageWrite(
        "too many clipboard images in the same minute".to_string(),
    ))
}

pub(crate) fn cleanup_old_images() {
    let Some(directory) = clipboard_cache_dir() else {
        return;
    };
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        let path = entry.path();
        let is_clipboard_png = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(IMAGE_PREFIX) && name.ends_with(".png"));
        if !is_clipboard_png {
            continue;
        }
        let Ok(modified) = entry.metadata().and_then(|metadata| metadata.modified()) else {
            continue;
        };
        if now
            .duration_since(modified)
            .is_ok_and(|age| age > IMAGE_MAX_AGE)
        {
            let _ = fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::borrow::Cow;

    struct FakeClipboard {
        text: Option<String>,
        image: Option<ImageData<'static>>,
        files: Vec<PathBuf>,
    }

    impl ClipboardReader for FakeClipboard {
        fn read_text(&mut self) -> Option<String> {
            self.text.clone()
        }

        fn read_image(&mut self) -> Option<ImageData<'static>> {
            self.image.take()
        }

        fn read_files(&mut self) -> Vec<PathBuf> {
            self.files.clone()
        }
    }

    fn test_image() -> ImageData<'static> {
        ImageData {
            width: 2,
            height: 1,
            bytes: Cow::Owned(vec![255, 0, 0, 255, 0, 255, 0, 128]),
        }
    }

    fn fixed_time() -> DateTime<Local> {
        Local
            .with_ymd_and_hms(2026, 10, 24, 12, 34, 0)
            .single()
            .expect("local time")
    }

    #[test]
    fn text_has_priority_over_image_and_files() {
        let directory = tempfile::tempdir().expect("tempdir");
        let payload = read_with(
            FakeClipboard {
                text: Some("hello".to_string()),
                image: Some(test_image()),
                files: vec![PathBuf::from("/tmp/file")],
            },
            directory.path(),
            fixed_time(),
        )
        .expect("clipboard payload");
        assert_eq!(payload, ClipboardPayload::Text("hello".to_string()));
        assert_eq!(fs::read_dir(directory.path()).expect("entries").count(), 0);
    }

    #[test]
    fn image_has_priority_over_files_and_is_png() {
        let directory = tempfile::tempdir().expect("tempdir");
        let payload = read_with(
            FakeClipboard {
                text: None,
                image: Some(test_image()),
                files: vec![PathBuf::from("/tmp/file")],
            },
            directory.path(),
            fixed_time(),
        )
        .expect("clipboard payload");
        let ClipboardPayload::Files(files) = payload else {
            panic!("expected image file");
        };
        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].file_name().and_then(|name| name.to_str()),
            Some("clipboard_2026-10-24_12-34.png")
        );
        let bytes = fs::read(&files[0]).expect("png bytes");
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
        let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
        let mut reader = decoder.read_info().expect("png reader");
        let mut output = vec![0; reader.output_buffer_size().expect("buffer size")];
        let info = reader.next_frame(&mut output).expect("png frame");
        assert_eq!((info.width, info.height), (2, 1));
        assert_eq!(info.color_type, png::ColorType::Rgba);
        assert_eq!(&output[..8], &[255, 0, 0, 255, 0, 255, 0, 128]);
    }

    #[test]
    fn files_are_preserved_when_text_and_image_are_absent() {
        let directory = tempfile::tempdir().expect("tempdir");
        let files = vec![PathBuf::from("first.txt"), PathBuf::from("folder")];
        let payload = read_with(
            FakeClipboard {
                text: Some("   ".to_string()),
                image: None,
                files: files.clone(),
            },
            directory.path(),
            fixed_time(),
        )
        .expect("clipboard payload");
        assert_eq!(payload, ClipboardPayload::Files(files));
    }

    #[test]
    fn path_like_text_is_still_text() {
        let directory = tempfile::tempdir().expect("tempdir");
        let payload = read_with(
            FakeClipboard {
                text: Some("/tmp/a.png".to_string()),
                image: None,
                files: vec![PathBuf::from("/tmp/a.png")],
            },
            directory.path(),
            fixed_time(),
        )
        .expect("clipboard payload");
        assert_eq!(payload, ClipboardPayload::Text("/tmp/a.png".to_string()));
    }

    #[test]
    fn empty_clipboard_is_reported() {
        let directory = tempfile::tempdir().expect("tempdir");
        let error = read_with(
            FakeClipboard {
                text: None,
                image: None,
                files: Vec::new(),
            },
            directory.path(),
            fixed_time(),
        )
        .expect_err("empty clipboard");
        assert_eq!(error, ClipboardError::Empty);
    }

    #[test]
    fn repeated_images_receive_unique_names() {
        let directory = tempfile::tempdir().expect("tempdir");
        let first = write_image_png(test_image(), directory.path(), fixed_time()).expect("first");
        let second = write_image_png(test_image(), directory.path(), fixed_time()).expect("second");
        assert_ne!(first, second);
        assert_eq!(
            second.file_name().and_then(|name| name.to_str()),
            Some("clipboard_2026-10-24_12-34_1.png")
        );
    }
}
