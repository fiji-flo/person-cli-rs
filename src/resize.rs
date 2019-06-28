use failure::Error;
use image::DynamicImage;
use image::FilterType;
use image::GenericImageView;
use image::ImageResult;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum ImageProcessingError {
    #[fail(display = "invalid data uri")]
    InvalidDataUri,
    #[fail(display = "invalid image format")]
    InvalidFormat,
    #[fail(display = "invalid base 64")]
    InvalidBase64,
}

pub struct Avatars {
    pub x264: Vec<u8>,
    pub x100: Vec<u8>,
    pub x40: Vec<u8>,
}

impl Avatars {
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let img = open_magic(path)?;
        let (w, h) = img.dimensions();
        let ratio = f64::from(w) / f64::from(h);
        if ratio < 0.95 || ratio > 1.05 {
            return Err(format_err!("wrong ascpect ratio: {}", ratio));
        }
        Ok(Avatars {
            x264: downsize(264, &img)?,
            x100: downsize(100, &img)?,
            x40: downsize(40, &img)?,
        })
    }
    pub fn save(path: &str) -> Result<(), Error> {
        Ok(())
    }
}

fn downsize(size: u32, img: &DynamicImage) -> Result<Vec<u8>, Error> {
    let down_sized = img.resize_to_fill(size, size, FilterType::CatmullRom);
    let mut buf: Vec<u8> = Vec::new();
    down_sized.write_to(&mut buf, image::ImageOutputFormat::PNG)?;
    Ok(buf)
}

fn open_magic(path: &PathBuf) -> ImageResult<DynamicImage> {
    let fin = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(image::ImageError::IoError(err)),
    };
    let mut fin = BufReader::new(fin);

    let format = image::guess_format(fin.fill_buf().map_err(image::ImageError::from)?)?;
    image::load(fin, format)
}
