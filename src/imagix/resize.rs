use std::path::PathBuf;
use std::result::Result;

pub fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
    let entries = fs::read_dir(src_folder)
        .map_err(|e| ImagixError::UserInputError("Invalid source folder".to_string()))?
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
    if !dest_folder_exists() {
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
        let destination_path = PathBuf::from("./images/tmp/images1.png");
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