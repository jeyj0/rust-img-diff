use image::io::Reader;

fn generate_diff(image_a: image::DynamicImage, image_b: image::DynamicImage) -> image::RgbaImage {
    let img_a = image_a.to_rgba();
    let img_b = image_b.to_rgba();

    if img_a.width() != img_b.width() || img_a.height() != img_b.height() {
        panic!("Images have different sizes");
    }

    let img_diff = image::DynamicImage::new_rgba8(img_a.width(), img_a.height()).to_rgba();

    let pixel_iter_a = img_a.enumerate_pixels();
    let pixel_iter_b = img_b.enumerate_pixels();
    let pixel_iter_diff = img_diff.enumerate_pixels();

    // iterate over all pixels and store the diff in pixel_iter_diff
}

fn main() {
    let image_a = Reader::open("./example_a.png").expect("Error reading file a").decode().expect("Error decoding file a");
    let image_b = Reader::open("./example_b.png").expect("Error reading file b").decode().expect("Error decoding file b");

    let image_diff: image::RgbaImage = generate_diff(image_a, image_b);

    image_diff.save("./example_diff.png").expect("Error saving diff");
}
