use crate::imspect_app::imspection::{ImageKind, SingleImspection, ThrSettings, Threshold};
use eframe::epaint::textures::{TextureFilter, TextureOptions};
use eframe::epaint::ColorImage;
use kornia::image::{Image, ImageError, ImageSize};
use kornia::imgproc::threshold::{
    threshold_binary, threshold_binary_inverse, threshold_to_zero, threshold_to_zero_inverse,
    threshold_truncate,
};

pub fn clone_img_as<T, const SRC_C: usize, const DST_C: usize>(
    img: &Image<u8, SRC_C>,
) -> Result<Image<T, DST_C>, ImageError>
where
    T: Clone + Default,
{
    Image::<T, DST_C>::from_size_val(
        ImageSize {
            width: img.width(),
            height: img.height(),
        },
        T::default(),
    )
}

fn apply_threshold_func<F>(img: &Image<u8, 1>, value: u8, threshold_func: F) -> Option<Image<u8, 1>>
where
    F: Fn(&Image<u8, 1>, &mut Image<u8, 1>, u8) -> Result<(), ImageError>,
{
    let mut new_img = clone_img_as::<u8, 1, 1>(img).ok()?;
    threshold_func(img, &mut new_img, value).ok()?;
    Some(new_img)
}
pub fn apply_threshold(image: &ImageKind, thr: &ThrSettings) -> Option<Image<u8, 1>> {
    let img = match &image {
        ImageKind::OneChannel(img) => img,
        _ => return None,
    };

    match thr.kind {
        Threshold::None => None,
        Threshold::Binary => apply_threshold_func(img, thr.value, |src, dst, value| {
            threshold_binary(src, dst, value, u8::MAX)
        }),
        Threshold::BinaryInv => apply_threshold_func(img, thr.value, |src, dst, value| {
            threshold_binary_inverse(src, dst, value, u8::MAX)
        }),
        Threshold::ToZero => apply_threshold_func(img, thr.value, threshold_to_zero),
        Threshold::ToZeroInv => apply_threshold_func(img, thr.value, threshold_to_zero_inverse),
        Threshold::Truncate => apply_threshold_func(img, thr.value, threshold_truncate),
    }
}

pub fn prepare_texture(ctx: &egui::Context, imspection: &mut SingleImspection) {
    if imspection.need_rerender {
        let color_img: ColorImage = match &imspection.image {
            ImageKind::OneChannel(img) => {
                if let Some(thr_img) = apply_threshold(&imspection.image, &imspection.thr) {
                    ColorImage::from_gray([thr_img.width(), thr_img.height()], thr_img.as_slice())
                } else {
                    ColorImage::from_gray([img.width(), img.height()], img.as_slice())
                }
            }
            ImageKind::ThreeChannel(img) => {
                ColorImage::from_rgb([img.width(), img.height()], img.as_slice())
            }
        };

        let options = TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Nearest,
            ..Default::default()
        };

        if let Some(texture) = &mut imspection.texture {
            texture.set(color_img, options);
        } else {
            imspection.texture =
                Some(ctx.load_texture(format!("texture_{}", &imspection.id), color_img, options));
        };
    };
}
