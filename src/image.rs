use image::GenericImageView;
use std::cmp;
use std::io::{BufWriter, Cursor};
use std::{fs::File, io::BufReader};

// see: https://www.cssmine.com/content/dist/images/original/background-size-cover-contain.svg
#[allow(dead_code)]
pub enum CropMode {
    Cover,
    Contain,
}

#[allow(dead_code)]
pub enum Output {
    File,
    Base64,
}

pub fn crop_image() {
    const TARGET_SIZE: u32 = 256;

    let output = Output::File;
    let crop_mode = CropMode::Cover;

    let filepath = "example/img/golden-retriever.jpg";
    let binding = filepath.split(".").collect::<Vec<&str>>();
    let ext = binding.last().unwrap();

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let format = image::ImageFormat::from_extension(ext).unwrap();
    let mut img = image::load(reader, format).unwrap();
    let (mut width, mut height) = img.dimensions();

    match crop_mode {
        CropMode::Cover => {
            let min = cmp::min(width, height);
            let ratio = (min as f32 / TARGET_SIZE as f32) as f32;

            // resize
            width = (width as f32 / ratio as f32).round() as u32;
            height = (height as f32 / ratio as f32).round() as u32;
            img = img.resize(width, height, image::imageops::FilterType::Nearest);

            // crop
            if width > height {
                let offset = (width - TARGET_SIZE) / 2;
                img = img.crop(offset, 0, TARGET_SIZE, TARGET_SIZE);
            } else {
                let offset = (width - TARGET_SIZE) / 2;
                img = img.crop(0, offset, TARGET_SIZE, TARGET_SIZE);
            }
        }
        CropMode::Contain => {
            img = img.resize(
                TARGET_SIZE,
                TARGET_SIZE,
                image::imageops::FilterType::Nearest,
            );
        }
    };

    let format = image::ImageFormat::from_extension("jpg").unwrap();

    match output {
        Output::File => {
            img.save_with_format("golden-retriever.jpeg", format)
                .unwrap();
        }
        Output::Base64 => {
            let c = Cursor::new(Vec::new());
            let mut writer = BufWriter::new(c);

            img.write_to(&mut writer, format).unwrap();
            let img_buf = writer.buffer();

            // data:image/jpeg;base64,HEX
            let base64 = base64::encode(img_buf);
            println!("base64.len() {}", base64.len());

            let _data = format!("{}{}", "data:image/jpeg;base64,", base64);
        }
    }
}
