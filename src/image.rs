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

// 6224
pub fn crop_image(filepath: String) -> Result<String, Box<dyn Error>> {
    const TARGET_SIZE: u32 = 256;

    let input_mode = Input::Base64;
    let output_mode = Output::Base64;
    let crop_mode = CropMode::Cover;

    // path on disk
    // let _filepath_example = "example/img/golden-retriever.jpg";
    // base64 string
    // let _filepath_example  = "/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAgACADAREAAhEBAxEB/9sAQwAIBgYHBgUIBwcHCQkICgwUDQwLCwwZEhMPFB0aHx4dGhwcICQuJyAiLCMcHCg3KSwwMTQ0NB8nOT04MjwuMzQy/9sAQwEJCQkMCwwYDQ0YMiEcITIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIyMjIy/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwD3VJ0rPmZXKSfaEAyTgDrT5w5WNM1v1Lx/Umpc4dSuWfRELXlgwIF3b8dcOpx/nFZynTa0kilTqL7LOSstX165JSHSHjQHH76Mrx65Yit+fyOdITxJeeIYPCmqypbCGcQFY3hYKwJ4yCGOCM5z7VjWm4wvsb4eKlUSZyvgq91GfU7uG2c2avaAqs4LlpzhiTuJKqvzKBjpn0UVx0Jxg0k9fwO6vCUk9PQ6C9tvF8lviSSznYc4tpjGTg5GDgeldftltY4XRn0Zv2+u2cibxcKExkM3yg/Qng10aGBz/i68a30i7MF5dXS3SmUQhlZVAAG1cDO1s8gkjg9K4cbN2UU9/wCv1O7BQXM5Poc34VuLu3zqGnxQvc/ZQGhkJQNg4wcZOcAgA8ZHUYrmw9/bJep04nSi36HcQeILWWFTqMBsJiuSpkVh+BUnP5V6bpxfQ81V5R6nGx3+LcSQzW8sg+XJGRtzngKxHbvgfSruY20OU8R6/cajb3bLuSB/KEew/fUhstjqOp6468ZGDXBUpN1ubv8AoejSqpUrdv1DwbdagJoCbhIU2+W0lyQcqzrhUAcfNkY5GPQ9qunStKTWhFaqmop6nV399DDcH7ZKXBwSpJAYjkfJ5ZG4+2OM12XOB+Z//9k=";

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

    match output_mode {
        Output::File => {
            img.save_with_format("golden-retriever.jpeg", format)?;
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
