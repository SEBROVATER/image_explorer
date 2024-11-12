use std::path::PathBuf;
use anyhow::Context;

use kornia::image::{Image, ImageError, ImageSize};
use kornia::io::functional::read_image_any;
use kornia::io::IoError;
use ndarray::Array3;
use ndarray_npy::read_npy;

use crate::imspect_app::imspection::ImageKind;

pub fn load_images(args: Vec<PathBuf>) -> Result<Vec<ImageKind>, anyhow::Error> {
    let mut imgs = Vec::with_capacity(args.len());

    for img_path in &args {
        // Check for valid file extension
        let extension = img_path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| IoError::InvalidFileExtension(img_path.to_owned()))?;

        if extension == "npy" {
            // Handle .npy file
            let arr: Array3<u8> = read_npy(img_path)
                .with_context(|| format!("Failed to read npy file: {:?}", img_path))?;
            let (h, w, c) = arr.dim();

            // Match the channel count to determine image type
            let image_size = ImageSize { width: w, height: h };
            let data = arr.into_raw_vec_and_offset().0;

            let img = match c {
                1 => ImageKind::OneChannel(Image::<u8, 1>::new(image_size, data)?),
                3 => ImageKind::ThreeChannel(Image::<u8, 3>::new(image_size, data)?),
                _ => return Err(ImageError::InvalidChannelShape(c, 3).into()),
            };
            imgs.push(img);
        } else {
            // Handle non-.npy file
            let img = ImageKind::ThreeChannel(read_image_any(img_path)
                .with_context(|| format!("Failed to read image file: {:?}", img_path))?);
            imgs.push(img);
        }
    }

    Ok(imgs)
}