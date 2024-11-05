use crate::imspect_app::imspection::{ImageKind, SingleImspection};
use eframe::epaint::textures::{TextureFilter, TextureOptions};
use eframe::epaint::ColorImage;

pub fn prepare_texture(ctx: &egui::Context, imspection: &mut SingleImspection) {

    if imspection.need_rerender {
        let color_img: ColorImage = match &imspection.image {
            ImageKind::OneChannel(img) => {
                ColorImage::from_gray([img.width(), img.height()], img.as_slice())
            },
            ImageKind::ThreeChannel(img) => {
                ColorImage::from_rgb([img.width(), img.height()], img.as_slice())
            },
        };

        let mut options = TextureOptions::default();
        options.magnification = TextureFilter::Nearest;
        options.minification = TextureFilter::Nearest;

        if let Some(texture) = &mut imspection.texture {
            texture.set(color_img, options);
        } else {
            imspection.texture =
                Some(ctx.load_texture(format!("texture_{}", &imspection.id), color_img, options));
        };
    };
}
