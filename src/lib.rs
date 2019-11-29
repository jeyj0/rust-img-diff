pub fn generate_diff(image_a: image::DynamicImage, image_b: image::DynamicImage) -> (u32, u32, Vec<u8>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::io::Reader;


    #[test]
    fn test_generate_diff() {
        // given
        let image_a = Reader::open("./assets/example_a.png").expect("Error reading file a").decode().expect("Error decoding file a");
        let image_b = Reader::open("./assets/example_b.png").expect("Error reading file b").decode().expect("Error decoding file b");
        let image_diff_wanted = Reader::open("./assets/example_diff.png").expect("Error reading wanted diff file").decode().expect("Error decoding wanted diff file").to_rgba();
        let image_diff_wanted_buffer = image_diff_wanted.clone().into_raw();

        // when
        let (width, height, buffer) = generate_diff(image_a, image_b);

        // then
        assert_eq!(width, image_diff_wanted.width());
        assert_eq!(height, image_diff_wanted.height());
        assert_eq!(buffer, image_diff_wanted_buffer);
    }
}
