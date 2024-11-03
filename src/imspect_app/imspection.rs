use eframe::epaint::TextureHandle;
use kornia::image::Image;

pub struct SingleImspection {
    pub image: Image<u8, 3>,
    pub texture: Option<TextureHandle>,
    pub idx: usize,
    pub need_rerender: bool,
    pub remove_flag: bool,
}
