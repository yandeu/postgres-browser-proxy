use image::{DynamicImage, GenericImageView};
use regex::Regex;
use std::cmp;
use std::error::Error;
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

#[allow(dead_code)]
pub enum Input {
    File,
    Base64,
}

pub fn crop_image<T: Into<String>>(filepath: T) -> Result<String, Box<dyn Error>> {
    const TARGET_SIZE: u32 = 512;

    let filepath: String = filepath.into();

    let input_mode = Input::Base64;
    let output_mode = Output::Base64;
    let crop_mode = CropMode::Cover;

    let mut img: DynamicImage = match input_mode {
        Input::Base64 => {
            let re = Regex::new(r"data:image/([\w]+);base64,").unwrap();
            let filepath = re.replace(&filepath, "").to_string();
            let mut buffer = Vec::<u8>::new();

            base64::decode_engine_vec(
                filepath.trim(),
                &mut buffer,
                &base64::engine::DEFAULT_ENGINE,
            )
            .expect("decode to buffer");
            image::load_from_memory(&buffer).expect("should load from memory")
        }
        Input::File => {
            let binding = filepath.split('.').collect::<Vec<&str>>();
            let ext = binding.last().unwrap();
            let format = image::ImageFormat::from_extension(ext).unwrap();
            let file = File::open(filepath)?;
            let reader = BufReader::new(file);
            image::load(reader, format)?
        }
    };

    let (mut width, mut height) = img.dimensions();

    match crop_mode {
        CropMode::Cover => {
            let min = cmp::min(width, height);
            let ratio = (min as f32 / TARGET_SIZE as f32).round();

            // resize
            width = (width as f32 / ratio).round() as u32;
            height = (height as f32 / ratio).round() as u32;
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

    match output_mode {
        Output::File => {
            img.save_with_format("example.jpeg", format)?;
            Ok(String::from(""))
        }
        Output::Base64 => {
            let c = Cursor::new(Vec::new());
            let mut writer = BufWriter::new(c);

            img.write_to(&mut writer, format)
                .expect("should write to buffer");

            let img_cursor = writer.into_inner().unwrap();
            let img_u8q = img_cursor.into_inner();

            // data:image/jpeg;base64,HEX
            let base64 = base64::encode(img_u8q);

            Ok(format!("{}{}", "data:image/jpeg;base64,", base64))
        }
    }
}
