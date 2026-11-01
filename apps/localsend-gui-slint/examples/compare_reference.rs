use anyhow::{Context, bail};
use png::{BitDepth, ColorType, Decoder, Encoder};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let expected = PathBuf::from(option(&args, "--expected").context("--expected is required")?);
    let actual = PathBuf::from(option(&args, "--actual").context("--actual is required")?);
    let tolerance = option(&args, "--tolerance")
        .unwrap_or_else(|| "4".to_string())
        .parse::<u8>()?;
    let tile_size = option(&args, "--tile-size")
        .unwrap_or_else(|| "64".to_string())
        .parse::<u32>()?;
    let top_tiles = option(&args, "--top-tiles")
        .unwrap_or_else(|| "20".to_string())
        .parse::<usize>()?;
    if tile_size == 0 {
        bail!("--tile-size must be greater than zero");
    }
    let diff_path = option(&args, "--diff").map(PathBuf::from);
    let crop = option(&args, "--crop");

    let mut expected = decode(&expected)?;
    let mut actual = decode(&actual)?;
    if expected.width != actual.width || expected.height != actual.height {
        bail!(
            "size mismatch: expected {}x{}, actual {}x{}",
            expected.width,
            expected.height,
            actual.width,
            actual.height
        );
    }
    if let Some(crop) = crop {
        let bounds = parse_crop(&crop, expected.width, expected.height)?;
        expected = expected.crop(bounds);
        actual = actual.crop(bounds);
    }

    let mut changed = 0_u64;
    let mut total_delta = 0_u64;
    let mut max_delta = 0_u8;
    let mut min_x = expected.width;
    let mut min_y = expected.height;
    let mut max_x = 0_u32;
    let mut max_y = 0_u32;
    let mut diff_pixels = vec![0_u8; expected.rgba.len()];
    let tile_columns = expected.width.div_ceil(tile_size) as usize;
    let tile_rows = expected.height.div_ceil(tile_size) as usize;
    let mut tile_changed = vec![0_u64; tile_columns * tile_rows];

    for y in 0..expected.height {
        for x in 0..expected.width {
            let offset = ((y * expected.width + x) * 4) as usize;
            let pixel_delta = (0..3)
                .map(|channel| {
                    expected.rgba[offset + channel].abs_diff(actual.rgba[offset + channel])
                })
                .max()
                .unwrap_or(0);
            total_delta += u64::from(pixel_delta);
            max_delta = max_delta.max(pixel_delta);
            if pixel_delta > tolerance {
                changed += 1;
                tile_changed[(y / tile_size) as usize * tile_columns + (x / tile_size) as usize] +=
                    1;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                diff_pixels[offset] = 255;
                diff_pixels[offset + 1] = 45;
                diff_pixels[offset + 2] = 45;
                diff_pixels[offset + 3] = 255;
            } else {
                let luminance = actual.rgba[offset] / 5;
                diff_pixels[offset] = luminance;
                diff_pixels[offset + 1] = luminance;
                diff_pixels[offset + 2] = luminance;
                diff_pixels[offset + 3] = 80;
            }
        }
    }

    let mut tiles = tile_changed
        .into_iter()
        .enumerate()
        .filter(|(_, changed)| *changed > 0)
        .map(|(index, changed)| {
            let x = (index % tile_columns) as u32 * tile_size;
            let y = (index / tile_columns) as u32 * tile_size;
            let width = tile_size.min(expected.width - x);
            let height = tile_size.min(expected.height - y);
            TileReport {
                x,
                y,
                width,
                height,
                changed_pixels: changed,
                changed_percent: changed as f64 * 100.0 / f64::from(width * height),
            }
        })
        .collect::<Vec<_>>();
    tiles.sort_by(|left, right| {
        right
            .changed_pixels
            .cmp(&left.changed_pixels)
            .then_with(|| right.changed_percent.total_cmp(&left.changed_percent))
    });
    tiles.truncate(top_tiles);

    let report = Report {
        width: expected.width,
        height: expected.height,
        tolerance,
        changed_pixels: changed,
        changed_percent: changed as f64 * 100.0
            / (u64::from(expected.width) * u64::from(expected.height)) as f64,
        mean_channel_delta: total_delta as f64
            / (u64::from(expected.width) * u64::from(expected.height) * 3) as f64,
        max_channel_delta: max_delta,
        changed_bounds: (changed > 0).then_some(Bounds {
            x: min_x,
            y: min_y,
            width: max_x - min_x + 1,
            height: max_y - min_y + 1,
        }),
        largest_tiles: tiles,
    };
    println!("{}", serde_json::to_string_pretty(&report)?);

    if let Some(path) = diff_path {
        write_rgba(&path, expected.width, expected.height, &diff_pixels)?;
    }
    Ok(())
}

#[derive(Serialize)]
struct Report {
    width: u32,
    height: u32,
    tolerance: u8,
    changed_pixels: u64,
    changed_percent: f64,
    mean_channel_delta: f64,
    max_channel_delta: u8,
    changed_bounds: Option<Bounds>,
    largest_tiles: Vec<TileReport>,
}

#[derive(Clone, Copy, Serialize)]
struct Bounds {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Serialize)]
struct TileReport {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    changed_pixels: u64,
    changed_percent: f64,
}

struct Image {
    width: u32,
    height: u32,
    rgba: Vec<u8>,
}

impl Image {
    fn crop(self, bounds: Bounds) -> Self {
        let mut rgba = Vec::with_capacity((bounds.width * bounds.height * 4) as usize);
        for y in bounds.y..bounds.y + bounds.height {
            let start = ((y * self.width + bounds.x) * 4) as usize;
            let end = start + (bounds.width * 4) as usize;
            rgba.extend_from_slice(&self.rgba[start..end]);
        }
        Self {
            width: bounds.width,
            height: bounds.height,
            rgba,
        }
    }
}

fn parse_crop(value: &str, width: u32, height: u32) -> anyhow::Result<Bounds> {
    let parts = value
        .split(',')
        .map(|part| part.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("--crop must be x,y,width,height; got {value}"))?;
    let [x, y, crop_width, crop_height] = parts.as_slice() else {
        bail!("--crop must have exactly four values: x,y,width,height");
    };
    if *crop_width == 0 || *crop_height == 0 {
        bail!("--crop width and height must be greater than zero");
    }
    if x + crop_width > width || y + crop_height > height {
        bail!("--crop {value} exceeds the {width}x{height} image");
    }
    Ok(Bounds {
        x: *x,
        y: *y,
        width: *crop_width,
        height: *crop_height,
    })
}

fn decode(path: &Path) -> anyhow::Result<Image> {
    let decoder = Decoder::new(BufReader::new(
        File::open(path).with_context(|| format!("opening {}", path.display()))?,
    ));
    let mut reader = decoder.read_info()?;
    let buffer_size = reader
        .output_buffer_size()
        .context("PNG output size is not available")?;
    let mut rgba = vec![0; buffer_size];
    let info = reader.next_frame(&mut rgba)?;
    if info.bit_depth != BitDepth::Eight {
        bail!("{} must be an 8-bit PNG", path.display());
    }
    let data = &rgba[..info.buffer_size()];
    let rgba = match info.color_type {
        ColorType::Rgba => data.to_vec(),
        ColorType::Rgb => data
            .chunks_exact(3)
            .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], 255])
            .collect(),
        other => bail!(
            "{} uses unsupported PNG color type {other:?}",
            path.display()
        ),
    };
    Ok(Image {
        width: info.width,
        height: info.height,
        rgba,
    })
}

fn write_rgba(path: &Path, width: u32, height: u32, rgba: &[u8]) -> anyhow::Result<()> {
    let mut encoder = Encoder::new(
        File::create(path).with_context(|| format!("creating {}", path.display()))?,
        width,
        height,
    );
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    encoder.write_header()?.write_image_data(rgba)?;
    Ok(())
}

fn option(args: &[String], key: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == key)
        .map(|pair| pair[1].clone())
}
