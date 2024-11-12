use crate::imspect_app::app::ImspectApp;
use crate::imspect_app::imspection::ImageKind;

pub fn imspect_kornia_images(imgs: Vec<ImageKind>) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "imspect",
        native_options,
        Box::new(|cc| Ok(Box::new(ImspectApp::new(cc, imgs)))),
    )
}