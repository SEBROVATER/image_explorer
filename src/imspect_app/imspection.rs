use std::cmp::PartialEq;
use std::fmt;

use crate::imspect_app::textures::apply_threshold;
use eframe::epaint::TextureHandle;
use kornia::image::{Image, ImageError, ImageSize};
use kornia::imgproc::color;

#[derive(Clone)]
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
    pub id: usize,
    pub need_rerender: bool,
    pub remove_flag: bool,
    pub thr: ThrSettings,
}

impl SingleImspection {
    pub fn apply_threshold(&self) -> Option<ImageKind> {
        apply_threshold(&self.image, &self.thr).map(ImageKind::OneChannel)
    }
    pub fn clone_with_thr(&self, id: usize) -> Self {
        let new_img = if let Some(new_image) = self.apply_threshold() {
            new_image
        } else {
            self.image.to_owned()
        };
        Self {
            image: new_img,
            texture: None,
            id,
            need_rerender: true,
            remove_flag: false,
            thr: Default::default(),
        }
    }

    pub fn new_with_took_channel(
        image: &ImageKind,
        channel_i: usize,
        id: usize,
    ) -> Result<Self, ImageError> {
        let new_img = match &image {
            ImageKind::OneChannel(img) => img.channel(channel_i)?,
            ImageKind::ThreeChannel(img) => img.channel(channel_i)?,
        };
        Ok(SingleImspection {
            image: ImageKind::OneChannel(new_img),
            texture: None,
            id,
            need_rerender: true,
            remove_flag: false,
            thr: Default::default(),
        })
    }
    pub fn new_with_changed_color(
        image: &ImageKind,
        color: ColorSpaceChange,
        id: usize,
    ) -> Result<Self, ImageError> {
        match image {
            ImageKind::OneChannel(img) => {
                if matches!(color, ColorSpaceChange::GRAY2RGB) {
                    let mut new_img = Image::<f32, 3>::from_size_val(
                        ImageSize {
                            width: img.width(),
                            height: img.height(),
                        },
                        0.,
                    )?;
                    color::rgb_from_gray(&img.cast::<f32>().unwrap(), &mut new_img)?;
                    let new_img = new_img.cast::<u8>()?;
                    Ok(SingleImspection {
                        image: ImageKind::ThreeChannel(new_img),
                        texture: None,
                        id,
                        need_rerender: true,
                        remove_flag: false,
                        thr: Default::default(),
                    })
                } else {
                    Err(ImageError::InvalidChannelShape(3, 1))
                }
            }
            ImageKind::ThreeChannel(img) => match color {
                ColorSpaceChange::GRAY2RGB => Err(ImageError::InvalidChannelShape(1, 3)),
                ColorSpaceChange::BGR2RGB => {
                    let mut new_img = Image::<u8, 3>::from_size_val(
                        ImageSize {
                            width: img.width(),
                            height: img.height(),
                        },
                        0,
                    )
                    .unwrap();
                    color::bgr_from_rgb(img, &mut new_img).unwrap();
                    Ok(SingleImspection {
                        image: ImageKind::ThreeChannel(new_img),
                        texture: None,
                        id,
                        need_rerender: true,
                        remove_flag: false,
                        thr: Default::default(),
                    })
                }
                ColorSpaceChange::RGB2GRAY => {
                    let mut new_img = Image::<f32, 1>::from_size_val(
                        ImageSize {
                            width: img.width(),
                            height: img.height(),
                        },
                        0.,
                    )
                    .unwrap();
                    color::gray_from_rgb(&img.cast::<f32>().unwrap(), &mut new_img).unwrap();
                    let new_img = new_img.cast::<u8>().unwrap();
                    Ok(SingleImspection {
                        image: ImageKind::OneChannel(new_img),
                        texture: None,
                        id,
                        need_rerender: true,
                        remove_flag: false,
                        thr: Default::default(),
                    })
                }
                ColorSpaceChange::RGB2HSV => {
                    let mut new_img = Image::<f32, 3>::from_size_val(
                        ImageSize {
                            width: img.width(),
                            height: img.height(),
                        },
                        0.,
                    )
                    .unwrap();

                    color::hsv_from_rgb(&img.cast::<f32>().unwrap(), &mut new_img).unwrap();
                    let new_img = new_img.cast::<u8>().unwrap();
                    Ok(SingleImspection {
                        image: ImageKind::ThreeChannel(new_img),
                        texture: None,
                        id,
                        need_rerender: true,
                        remove_flag: false,
                        thr: Default::default(),
                    })
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

#[derive(PartialEq, Debug)]
pub enum Threshold {
    None,
    Binary,
    BinaryInv,
    ToZero,
    ToZeroInv,
    Truncate,
    // InRange,
}

impl fmt::Display for Threshold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self::None
    }
}
