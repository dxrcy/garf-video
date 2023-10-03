use image::imageops;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use imageproc::drawing::draw_filled_rect;
use imageproc::rect::Rect;
use std::{fs, path::Path};

fn main() {
    let dir = "/home/darcy/pics/geo/posts";
    let dir_out = "./temp";

    assert!(Path::new(dir).exists(), "cant find input dir");

    if Path::new(dir_out).exists() {
        fs::remove_dir_all(dir_out).expect("remove out dir");
    }
    fs::create_dir(dir_out).expect("create out dir");

    let ids = &["0500", "0501", "0502"];

    for id in ids {
        let folder = format!("{dir}/{id}");
        let folder_out = format!("{dir_out}/{id}");

        assert!(Path::new(&folder).exists(), "cant find input folder");
        if !Path::new(&folder_out).exists() {
            fs::create_dir(&folder_out).expect("create out folder ");
        }

        println!("{}", id);

        let esperanto = convert_image(
            image::open(format!("{folder}/esperanto.png")).expect("open esperanto image"),
        );
        let english = convert_image(
            image::open(format!("{folder}/english.png")).expect("open english image"),
        );

        let (width, height) = esperanto.dimensions();

        println!("    frame 0");
        let rect = Rect::at(width as i32 / 3, 0).of_size(width, height);
        draw_filled_rect(&esperanto.to_rgba8(), rect, WHITE)
            .save(format!("{folder_out}/frame_0.png"))
            .expect("save frame 0 (1 panel)");

        println!("    frame 1");
        let rect = Rect::at(width as i32 * 2 / 3, 0).of_size(width, height);
        draw_filled_rect(&esperanto.to_rgba8(), rect, WHITE)
            .save(format!("{folder_out}/frame_1.png"))
            .expect("save frame 1 (2 panels)");

        println!("    frame 2");
        esperanto
            .save(format!("{folder_out}/frame_2.png"))
            .expect("save frame 2 (3 panels)");

        println!("    frame 3");
        english
            .save(format!("{folder_out}/frame_3.png"))
            .expect("save frame 3 (english)");
    }
}

fn convert_image(image: DynamicImage) -> DynamicImage {
    let image = remove_padding_except_right(image);
    let image = make_unsquare(image);
    let image = remove_padding(image);
    let image = add_padding(image);
    image
}

/// White color for text and padding
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
/// Relative amount of extra white to add
const PADDING_AMOUNT: f32 = 0.009;
/// Minimum value of a color to be considered white.
/// Used to crop initial padding, which can be any amount
const MIN_WHITE_THRESHOLD: u8 = 100;

fn make_unsquare(image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();

    let mut long = ImageBuffer::from_pixel((width as f32 * 1.5) as u32, height / 2, WHITE);

    imageops::overlay(&mut long, &image.to_rgba8(), 0, 0);
    imageops::overlay(&mut long, &image.to_rgba8(), width as i64, 0);

    DynamicImage::ImageRgba8(long)
}

/// Add extra white padding to image
fn add_padding(image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();

    let padding = (width.min(height) as f32 * PADDING_AMOUNT) as u32;

    let mut padded = ImageBuffer::from_pixel(width + padding * 2, height + padding * 2, WHITE);

    imageops::overlay(&mut padded, &image, padding as i64, padding as i64);

    DynamicImage::ImageRgba8(padded)
}

/// Remove initial white padding from image
fn remove_padding(mut image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();

    let (mut min_x, mut min_y, mut max_x, mut max_y) = (width, height, 0, 0);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            if !is_white_enough(pixel) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    if min_x <= max_x && min_y <= max_y {
        image.crop(min_x, min_y, max_x - min_x + 1, max_y - min_y + 1)
    } else {
        image
    }
}

fn remove_padding_except_right(mut image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();

    let (mut min_x, mut min_y, mut max_y) = (width, height, 0);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            if !is_white_enough(pixel) {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    if min_x <= width && min_y <= max_y {
        image.crop(min_x, min_y, width - min_x + 1, max_y - min_y + 1)
    } else {
        image
    }
}

/// Returns if pixel value is considered white enough (MIN_WHITE_THRESHOLD)
fn is_white_enough(pixel: Rgba<u8>) -> bool {
    let Rgba([r, g, b, a]) = pixel;
    if a < 255 {
        return true;
    }
    r >= MIN_WHITE_THRESHOLD && g >= MIN_WHITE_THRESHOLD && b >= MIN_WHITE_THRESHOLD
}