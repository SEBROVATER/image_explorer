use std::env;
use std::path::PathBuf;

use imspect_app::run::imspect_kornia_images;
use input::load_images;

mod input;

mod imspect_app;

fn main() -> eframe::Result<()> {
    let args: Vec<PathBuf> = env::args().skip(1).map(PathBuf::from).collect();

    if args.is_empty() {
        println!("Provide at least one image path");
        return Ok(());
    };

    if let Ok(imgs) = load_images(args) {
        return imspect_kornia_images(imgs);
    };
    Ok(())
}
