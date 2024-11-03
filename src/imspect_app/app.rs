use std::default::Default;

use eframe::egui;
use egui::load::SizedTexture;
use egui::{Align, Layout, Ui, Vec2};
use kornia::image::Image;

use crate::imspect_app::imspection::SingleImspection;
use crate::imspect_app::textures::prepare_texture;

#[derive(Default)]
pub struct ImspectApp {
    imspections: Vec<SingleImspection>,
}

impl ImspectApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, imgs: Vec<Image<u8, 3>>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_pixels_per_point(1.0);

        let imspections_vec: Vec<SingleImspection> = imgs
            .into_iter()
            .enumerate()
            .map(|(i, img)| SingleImspection {
                image: img,
                texture: None,
                idx: i,
                need_rerender: true,
                remove_flag: false,
            })
            .collect();

        Self {
            imspections: imspections_vec,
        }
    }
    fn render_single_imspection(
        &mut self,
        ctx: &egui::Context,
        ui: &mut Ui,
        idx: &usize,
        outer_size: &Vec2,
    ) {
        let img_count = self.imspections.len();
        let full_width = outer_size.x;
        let full_height = outer_size.y;

        let imspection: &mut SingleImspection = self
            .imspections
            .get_mut(*idx)
            .expect("single imspection struct");
        prepare_texture(ctx, imspection);

        egui::Resize::default()
            .id_salt(&imspection.idx)
            .default_size(Vec2::new(
                full_width / img_count as f32 - 5.,
                full_height - 5.,
            ))
            .max_size(Vec2::new(full_width - 5., full_height - 2.))
            .show(ui, |ui| {
                // TODO: choose layout depending on aspect ratio
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let inner_width = ui.available_width();

                    if let Some(texture) = &imspection.texture {
                        ui.add(egui::Image::new(SizedTexture::new(
                            texture.id(),
                            Vec2::new(
                                inner_width,
                                inner_width / imspection.image.width() as f32
                                    * imspection.image.height() as f32,
                            ),
                        )));
                    };
                    ui.heading("test text");
                });
            });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let img_count = self.imspections.len();
            let outer_size = ui.available_size();

            ui.horizontal_top(|ui| {
                for idx in 0..img_count {
                    self.render_single_imspection(ctx, ui, &idx, &outer_size);
                }
            });
        });
    }
}

impl eframe::App for ImspectApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for idx in (0..self.imspections.len()).rev() {
            if self.imspections[idx].remove_flag {
                self.imspections.remove(idx);
            }
        }
        self.render_central_panel(ctx);
    }
}
