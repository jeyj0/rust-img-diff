use image::io::Reader;

fn generate_diff(image_a: image::DynamicImage, image_b: image::DynamicImage) -> (u32, u32, Vec<u8>) {
    let img_a = image_a.to_rgba();
    let img_b = image_b.to_rgba();

    if img_a.width() != img_b.width() || img_a.height() != img_b.height() {
        panic!("Images have different sizes");
    }

    let mut values: Vec<u8> = vec!();

    img_a.pixels().zip(img_b.pixels()).for_each(|(pixel_a, pixel_b)| {
        values.push(255);
        values.push(0);
        values.push(0);
        if pixel_a != pixel_b {
            values.push(255);
        } else {
            values.push(0);
        }
    });

    return (img_a.width(), img_a.height(), values);
}

fn main() {
    let image_a = Reader::open("./example_a.png").expect("Error reading file a").decode().expect("Error decoding file a");
    let image_b = Reader::open("./example_b.png").expect("Error reading file b").decode().expect("Error decoding file b");

    let (width, height, buffer) = generate_diff(image_a, image_b);

    image::save_buffer_with_format("./example_diff.png", buffer.as_slice(), width, height, image::RGBA(8), image::ImageFormat::PNG).expect("Error saving diff");
}

