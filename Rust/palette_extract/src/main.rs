use anyhow::anyhow;
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use std::{ffi::OsStr, path::Path, time::SystemTime};

use palette_extractor::{get_args, k_means};

fn main() -> Result<(), anyhow::Error> {
    let (path, k) = get_args()?;

    let mut image = image::open(&path)?;

    let centroids = k_means::k_cluster(&image, k)?;

    let rect_height = image.height() / k as u32;
    let rect_width = image.width() / k as u32;

    for (i, color) in centroids.iter().enumerate() {
        let rect = Rect::at(0, i as i32 * rect_height as i32).of_size(rect_height, rect_width);

        draw_filled_rect_mut(&mut image, rect, color.center);
    }

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let extension = Path::new(&path)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| anyhow!("File doesn't have an valid extension"))?;

    let filename = format!("{}_{}.{}", path, time, extension);

    image.save(filename)?;

    Ok(())
}
