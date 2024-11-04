use std::default::Default;

use eframe::egui;
use egui::{Align, Layout, Slider, Ui, Vec2};
use egui::load::SizedTexture;

use crate::imspect_app::imspection::{ColorSpaceChange, ImageKind, SingleImspection, Threshold};
use crate::imspect_app::textures::prepare_texture;

#[derive(Default)]
pub struct ImspectApp {
    imspections: Vec<SingleImspection>,
    new_imspections: Vec<SingleImspection>,
}

impl ImspectApp {

    pub fn next_available_id(&self) -> usize {

        let mut idx = if let Some(imspection) = self.imspections.last() {
          imspection.idx.clone().overflowing_add(1).0
        } else {
            return 0
        };
        let exitsting_idxes: Vec<usize> = self.imspections.iter().map(
            | imspection | imspection.idx.clone()
        ).collect();

        loop {
            if !exitsting_idxes.contains(&idx) {
                break
            }
            idx = idx.overflowing_add(1).0

        }
        idx
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, imgs: Vec<ImageKind>) -> Self {
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
                thr: Default::default(),
            })
            .collect();

        Self {
            imspections: imspections_vec,
            new_imspections: vec![],
        }
    }
    fn new_single_imspection_by_color_conversion(&self, idx: &usize, color: ColorSpaceChange) -> Option<SingleImspection> {
        let new_idx = self.next_available_id();
        if let Some(new_imspection) = self.imspections.get(*idx).unwrap().new_with_changed_color(color, new_idx) {
            Some(new_imspection)
        } else {
            None
        }


    }
    fn render_thresholding(&mut self, ui: &mut Ui, idx: &usize) {
        let imsp = self.imspections.get(*idx).unwrap();
        if let ImageKind::OneChannel(img) = &imsp.image {

            let imspection: &mut SingleImspection = self.imspections.get_mut(*idx).unwrap();
            ui.horizontal_top(|ui| {
                ui.heading("Threshold");
                ui.radio_value(&mut imspection.thr.kind, Threshold::None, "None");
                ui.radio_value(&mut imspection.thr.kind, Threshold::Binary, "Binary");
                ui.radio_value(&mut imspection.thr.kind, Threshold::BinaryInv, "BinaryInv");
            });
            if (imspection.thr.kind == Threshold::Binary) ||
                (imspection.thr.kind == Threshold::BinaryInv) {
                ui.add(Slider::new(&mut imspection.thr.value, 0..=255));
                // TODO: trigger on slider value change
            };
        };
    }

    fn render_color_conversions(&mut self, ui: &mut Ui, idx: &usize) {
        ui.menu_button("Change color space", |ui| {

            match &self.imspections.get(*idx).unwrap().image {
                ImageKind::OneChannel(img) => {
                    if ui.button("GRAY => RGB").clicked() {
                        let new_imspection = self.new_single_imspection_by_color_conversion(idx, ColorSpaceChange::GRAY2RGB);
                        if let Some(new_imsp) = new_imspection {
                            self.imspections.push(new_imsp);
                        }
                        ui.close_menu();
                    }
                }
                ImageKind::ThreeChannel(img) => {
                    if ui.button("BGR => RGB").clicked() {
                        // TODO: convert
                        ui.close_menu();
                    }
                    if ui.button("RGB => GRAY").clicked() {
                        // TODO: convert
                        ui.close_menu();
                    }
                    if ui.button("RGB => HSV").clicked() {
                        // TODO: convert
                        ui.close_menu();
                    }
                }
            }
        });
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

        // let mut new_imspection: Option<SingleImspection> = None;

        egui::Resize::default()
            .id_salt(self.next_available_id())
            .default_size(Vec2::new(
                full_width / img_count as f32 - 5.,
                full_height - 5.,
            ))
            .max_size(Vec2::new(full_width - 5., full_height - 2.))
            .show(ui, |ui| {
                // TODO: choose layout depending on aspect ratio
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let inner_width = ui.available_width();

                    let imspection = self
                        .imspections
                        .get_mut(*idx)
                        .expect("single imspection struct");

                    prepare_texture(ctx, imspection);
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
                    if ui.small_button("X").clicked() {
                        imspection.remove_flag = true;
                    };
                    ui.label(format!("Channels count: {}", imspection.image.num_channels()));

                    self.render_thresholding(ui, idx);
                    self.render_color_conversions(ui, idx);


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
    fn remove_marked_imspections(&mut self) {
        for idx in (0..self.imspections.len()).rev() {
            if self.imspections[idx].remove_flag {
                self.imspections.remove(idx);
            }
        }
    }
}

impl eframe::App for ImspectApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.remove_marked_imspections();

        self.render_central_panel(ctx);
    }
}
