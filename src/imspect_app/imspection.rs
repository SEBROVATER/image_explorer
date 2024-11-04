use std::cmp::PartialEq;
use eframe::epaint::TextureHandle;
use kornia::image::{Image, ImageSize};
use kornia::imgproc::color;


pub enum ImageKind {
    OneChannel(Image<u8, 1>),
    ThreeChannel(Image<u8, 3>),
}

impl ImageKind {
    pub fn num_channels(&self) -> usize {
        match self {
            ImageKind::OneChannel(_) => 1,
            ImageKind::ThreeChannel(_) => 3,
        }
    }
    pub fn width(&self) -> usize {
        match self {
            ImageKind::OneChannel(img) => img.width(),
            ImageKind::ThreeChannel(img) => img.width(),
        }
    }
    pub fn height(&self) -> usize {
        match self {
            ImageKind::OneChannel(img) => img.height(),
            ImageKind::ThreeChannel(img) => img.height(),
        }
    }
}

pub struct SingleImspection {
    pub image: ImageKind,
    pub texture: Option<TextureHandle>,
    pub idx: usize,
    pub need_rerender: bool,
    pub remove_flag: bool,
    pub thr: ThrSettings,
}



impl SingleImspection {
    pub fn new_with_changed_color(&self, color: ColorSpaceChange, id: usize) -> Option<Self> {
        match &self.image {
            ImageKind::OneChannel(img) => {
                if matches!(color, ColorSpaceChange::GRAY2RGB) {
                    let mut new_img = Image::<f32, 3>::from_size_slice(
                        ImageSize {width: img.width(), height: img.height()},
                        &Vec::<f32>::with_capacity(img.width() * img.height() * 3)
                    ).unwrap();
                    color::rgb_from_gray(&img.cast::<f32>().unwrap(), &mut new_img).unwrap();
                    let new_img = new_img.cast::<u8>().unwrap();
                    Some(SingleImspection{
                        image: ImageKind::ThreeChannel(new_img),
                        texture: None,
                        idx: id,
                        need_rerender: true,
                        remove_flag: false,
                        thr: Default::default(),
                    })

                } else {
                    return None
                }
            },
            ImageKind::ThreeChannel(img) => {
                match color {
                    ColorSpaceChange::GRAY2RGB => {
                        return None
                    },
                    ColorSpaceChange::BGR2RGB => {
                        let mut new_img = Image::<u8, 3>::from_size_slice(
                            ImageSize {width: img.width(), height: img.height()},
                            &Vec::<u8>::with_capacity(img.width() * img.height() * 3)
                        ).unwrap();
                        color::bgr_from_rgb(img, &mut new_img).unwrap();
                        Some(SingleImspection{
                            image: ImageKind::ThreeChannel(new_img),
                            texture: None,
                            idx: id,
                            need_rerender: true,
                            remove_flag: false,
                            thr: Default::default(),
                        })

                    },
                    ColorSpaceChange::RGB2GRAY => {
                        let mut new_img = Image::<f32, 1>::from_size_slice(
                            ImageSize {width: img.width(), height: img.height()},
                            &Vec::<f32>::with_capacity(img.width() * img.height())
                        ).unwrap();
                        color::gray_from_rgb(&img.cast::<f32>().unwrap(), &mut new_img).unwrap();
                        let new_img = new_img.cast::<u8>().unwrap();
                        Some(SingleImspection{
                            image: ImageKind::OneChannel(new_img),
                            texture: None,
                            idx: id,
                            need_rerender: true,
                            remove_flag: false,
                            thr: Default::default(),
                        })
                    },
                    ColorSpaceChange::RGB2HSV => {
                        let mut new_img = Image::<f32, 3>::from_size_slice(
                            ImageSize {width: img.width(), height: img.height()},
                            &Vec::<f32>::with_capacity(img.width() * img.height() * 3)
                        ).unwrap();

                        color::hsv_from_rgb(&img.cast::<f32>().unwrap(), &mut new_img).unwrap();
                        let new_img = new_img.cast::<u8>().unwrap();
                        Some(SingleImspection{
                            image: ImageKind::ThreeChannel(new_img),
                            texture: None,
                            idx: id,
                            need_rerender: true,
                            remove_flag: false,
                            thr: Default::default(),
                        })
                    },
                }
            },
        }

    }
}
pub enum ColorSpaceChange {
    BGR2RGB,
    RGB2GRAY,
    RGB2HSV,
    GRAY2RGB,
}

#[derive(Default)]
pub struct ThrSettings {
    pub kind: Threshold,
    pub value: u8,

}

#[derive(PartialEq)]
pub enum Threshold {
    None,
    Binary,
    BinaryInv,


}
impl Default for Threshold {
    fn default() -> Self {
        Self::None
    }
}