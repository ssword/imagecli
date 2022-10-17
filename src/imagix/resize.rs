use std::path::PathBuf;
use std::result::Result;
use image::ImageFormat;
use std::time::{Duration, Instant};
use std::{fmt, fs, io};
use std::str::FromStr;

use super::error::ImagixError;

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_1000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_1000),
            (s, n) if s < 10 => write!(out, "{}.{:2} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

impl FromStr for SizeOption {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Ok(SizeOption::Small),
            //default
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Single,
    All,
}

impl FromStr for Mode {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::UserInputError("Wrong value for mode".to_string())),
        }
    }
}

pub fn process_resize_request (
    size: SizeOption,
    mode: Mode,
    src_folder: &mut PathBuf,
) -> Result<(), ImagixError> {
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };
    let _ = match mode {
        Mode::All => resize_all(size, src_folder)?,
        Mode::Single => resize_single(size, src_folder)?,
    };
    Ok(())
}

fn resize_single(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    let mut src_folder = src_folder;
    // Get file stem from src_folder
    resize_image(size, &mut src_folder)?;
    Ok(())
}

fn resize_all(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    if let Ok(entries) = get_image_files(src_folder.to_path_buf()) {
        for mut entry in entries {
            resize_image(size, &mut entry)?;
        }
    };
    Ok(())
}

pub fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
    let entries = fs::read_dir(src_folder)
        .map_err(|_e| ImagixError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
            || r.extension() == Some("jpg".as_ref())
            || r.extension() == Some("PNG".as_ref())
            || r.extension() == Some("png".as_ref())
        })
        .collect();
    Ok(entries)
}

fn resize_image(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    // Construct destination filename with .png extension
    let new_file_name = src_folder
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(std::io::ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f));

    // Construct path to destination folder i.e. create /tmp
    // under source folder if not exists
    let mut dest_folder = src_folder.clone();
    dest_folder.pop();
    dest_folder.push("tmp/");
    if !dest_folder.exists() {
        fs::create_dir(&dest_folder)?;
    }
    dest_folder.pop();
    dest_folder.push("tmp/tmp.png");
    dest_folder.set_file_name(new_file_name?.as_str());

    let timer = Instant::now();
    let img = image::open(&src_folder)?;
    let scaled = img.thumbnail(size, size);
    let mut output = fs::File::create(&dest_folder)?;
    scaled.write_to(&mut output, ImageFormat::Png)?;
    println!(
        "Thumbnailed file: {:?} to size {} X {} in {}. Output file in {:?}",
        src_folder, size, size, Elapsed::from(&timer), dest_folder
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_image_resize() {
        let mut path = PathBuf::from("./images/image1.jpg");
        let destination_path = PathBuf::from("./images/tmp/image1.png");
        match process_resize_request(SizeOption::Small, Mode::Single, &mut path) {
            Ok(_) => println!("Successful resize of single image"),
            Err(e) => println!("Error in single image: {:?}", e),
        }
        assert_eq!(true, destination_path.exists());
    }
    #[test]
    fn test_multiple_image_resize() {
        let mut path = PathBuf::from("./images/");
        let _res = process_resize_request(SizeOption::Small, Mode::All, &mut path);
        let destination_path1 = PathBuf::from("./images/tmp/image1.png");
        let destination_path2 = PathBuf::from("./images/tmp/image2.png");
        assert_eq!(true, destination_path1.exists());
        assert_eq!(true, destination_path2.exists());
    }
}