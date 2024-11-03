use crate::imspect_app::imspection::SingleImspection;
use eframe::epaint::textures::{TextureFilter, TextureOptions};
use eframe::epaint::ColorImage;

pub fn prepare_texture(ctx: &egui::Context, imspection: &mut SingleImspection) {
    // TODO: account channels count
    if imspection.need_rerender {
        let img = ColorImage::from_rgb(
            [imspection.image.width(), imspection.image.height()],
            imspection.image.as_slice(),
        );

        let mut options = TextureOptions::default();
        options.magnification = TextureFilter::Nearest;
        options.minification = TextureFilter::Nearest;

        if let Some(texture) = &mut imspection.texture {
            texture.set(img, options);
        } else {
            imspection.texture =
                Some(ctx.load_texture(format!("texture_{}", &imspection.idx), img, options));
        };
    };
}
