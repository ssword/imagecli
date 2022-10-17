use std::path::PathBuf;
use super::resize::get_image_files;
use super::error::ImagixError;

pub fn get_stats(src_folder: PathBuf) -> Result<(usize, f64), ImagixError> {
    let image_files = get_image_files(src_folder.to_path_buf())?;
    let size = image_files
        .iter()
        .map(move |f| f.metadata().unwrap().len())
        .sum::<u64>();
    Ok((image_files.len(), (size / 1000000) as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_stats() {
        let image_files = PathBuf::from("./images");
        let (count, size) = get_stats(image_files).unwrap();

        assert_eq!(count, 2);
        assert_eq!(size, 27.0);
    }
}