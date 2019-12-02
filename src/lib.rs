pub fn generate_diff_image(image_a: image::DynamicImage, image_b: image::DynamicImage) -> (u32, u32, Vec<u8>) {
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

pub fn save_diff_to_disk(image_a: image::DynamicImage, image_b: image::DynamicImage, output_path: &str) -> Result<(u32, u32, Vec<u8>), std::io::Error> {
    let (width, height, buffer) = generate_diff_image(image_a, image_b);

    match image::save_buffer(output_path, buffer.as_slice(), width, height, image::ColorType::RGBA(8)) {
        Ok(_) => Ok((width, height, buffer)),
        Err(e) => Err(e)
    }
}

fn save_diff_to_disk_and_panic(image_a: image::DynamicImage, image_b: image::DynamicImage, output_path: &str) {
    match save_diff_to_disk(image_a, image_b, output_path) {
        Ok(_) => panic!("Images are not equal. Diff saved to {}", output_path),
        Err(_) => panic!("Images are not equal. Could not save diff to disk.")
    }
}

pub fn assert_eq_or_save_diff(image_a: image::DynamicImage, image_b: image::DynamicImage, output_path: &str) {
    let img_a = image_a.to_rgba();
    let img_b = image_b.to_rgba();

    if img_a.dimensions() != img_b.dimensions() {
        save_diff_to_disk_and_panic(image_a, image_b, output_path);
        return;
    }

    let pixels_a = img_a.into_raw();
    let pixels_b = img_b.into_raw();
    let mut pixels_b_iter = pixels_b.iter();

    for a in pixels_a {
        if *pixels_b_iter.next().unwrap() != a {
            save_diff_to_disk_and_panic(image_a, image_b, output_path);
            return;
        }
    }
}

pub fn assert_eq(expected_image_path: &str, given_image: image::DynamicImage, diff_output_path: &str) {
    match Reader::open(expected_image_path) {
        Some(expected_image) => assert_eq_or_save_diff(expected_image, given_image, diff_output_path),
        None => given_image.save(expected_image_path)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::io::Reader;

    #[test]
    fn test_generate_diff_image() {
        // given
        let image_a = Reader::open("./assets/example_a.png").expect("Error reading file a").decode().expect("Error decoding file a");
        let image_b = Reader::open("./assets/example_b.png").expect("Error reading file b").decode().expect("Error decoding file b");
        let image_diff_wanted = Reader::open("./assets/example_diff.png").expect("Error reading wanted diff file").decode().expect("Error decoding wanted diff file").to_rgba();
        let image_diff_wanted_buffer = image_diff_wanted.clone().into_raw();

        // when
        let (width, height, buffer) = generate_diff_image(image_a, image_b);

        // then
        assert_eq!(width, image_diff_wanted.width());
        assert_eq!(height, image_diff_wanted.height());
        assert_eq!(buffer, image_diff_wanted_buffer);
    }

    #[test]
    fn test_assert_eq_or_save_diff() {
        // given
        let image_a = Reader::open("./assets/example_a.png").expect("Error reading file").decode().expect("Error decoding file");

        // when
        assert_eq_or_save_diff(image_a.clone(), image_a, "./test-output/test_assert_eq_or_save_diff__shouldnt_exist.png");

        // then
        // assert_eq... would panic, in which case this test fails
    }

    #[test]
    #[should_panic(expected = "Images are not equal. Diff saved to ./test-output/test_assert_eq_or_save_diff_should_panic.png")]
    fn test_assert_eq_or_save_diff_should_panic() {
        // given
        let image_a = Reader::open("./assets/example_a.png").expect("Error reading file a").decode().expect("Error decoding file a");
        let image_b = Reader::open("./assets/example_b.png").expect("Error reading file b").decode().expect("Error decoding file b");

        // when
        assert_eq_or_save_diff(image_a, image_b, "./test-output/test_assert_eq_or_save_diff_should_panic.png");
    }

    #[test]
    #[should_panic(expected = "Images are not equal. Could not save diff to disk.")]
    fn test_assert_eq_or_save_diff_should_panic_when_saving_fails() {
        // given
        let image_a = Reader::open("./assets/example_a.png").expect("Error reading file a").decode().expect("Error decoding file a");
        let image_b = Reader::open("./assets/example_b.png").expect("Error reading file b").decode().expect("Error decoding file b");

        // when
        assert_eq_or_save_diff(image_a, image_b, "./test-output/non-existent-folder/file.png");
    }
}
