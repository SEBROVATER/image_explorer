mod imspect_app;

use imspect_app::app::ImspectApp;
use kornia::image::{Image, ImageSize};
use kornia::io::functional::read_image_any;
use ndarray::{Array3, Axis, Ix3};
use ndarray_npy::{read_npy, write_npy};
use numpy::{PyReadonlyArrayDyn, PyUntypedArrayMethods};
use pyo3::exceptions::{PyIOError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use temp_dir::TempDir;
use crate::imspect_app::imspection::ImageKind;

/// accept kornia images
fn imspect_kornia_images(imgs: Vec<ImageKind>) -> eframe::Result {
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

/// to run in a python shell
#[pyfunction]
#[pyo3(name = "imspect", signature = (* imgs))]
fn _imspect_for_shell<'py>(_py: Python<'py>, imgs: &Bound<'py, PyTuple>) -> PyResult<()> {
    let mut imgs_vec: Vec<Array3<u8>> = Vec::with_capacity(imgs.len());

    for img in imgs.iter() {
        let py_arr: PyReadonlyArrayDyn<'py, u8> = img
            .extract()
            .map_err(|_| PyTypeError::new_err("Only 'uint8' array images can be accepted"))?;
        let rs_arr: Array3<u8> = match py_arr.ndim() {
            0..2 => {
                return Err(PyTypeError::new_err(
                    "Only array images with 2 or 3 dimensions can be accepted",
                ));
            }
            3 => py_arr
                .as_array()
                .into_dimensionality::<Ix3>()
                .unwrap()
                .to_owned(),
            2 => py_arr
                .as_array()
                .insert_axis(Axis(2))
                .into_dimensionality::<Ix3>()
                .unwrap()
                .to_owned(),
            4.. => {
                return Err(PyTypeError::new_err(
                    "Only array images with 2 or 3 dimensions can be accepted",
                ));
            }
        };
        imgs_vec.push(rs_arr);
    }
    let mut img_paths: Vec<PathBuf> = Vec::with_capacity(imgs_vec.len());
    let tmp_dir = TempDir::new().map_err(|_| {
        PyValueError::new_err(
            "Can't pass images to the separate process. Temp dir is not accessible",
        )
    })?;
    for (i, img) in imgs_vec.iter().enumerate() {
        let f = tmp_dir.child(format!("imspect_img_{}.npy", i));
        write_npy(&f, img).map_err(|_| {
            PyValueError::new_err(
                "Can't pass images to the separate process. Temp dir is not accessible",
            )
        })?;

        img_paths.push(f);
    }

    let status = match Command::new("imspect")
        .args(&img_paths)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Err(_) => Err(PyRuntimeError::new_err(
            "Can't find 'imspect' script to run",
        )),
        Ok(_) => Ok(()),
    };

    thread::sleep(Duration::from_millis(500));

    status
}

/// to run in a command prompt
#[pyfunction]
fn _imspect_script() -> PyResult<()> {
    let args: Vec<PathBuf> = env::args().skip(2).map(PathBuf::from).collect();

    let mut imgs: Vec<ImageKind> = Vec::with_capacity(args.len());
    for img_path in args.iter() {
        if img_path.extension().ok_or(PyIOError::new_err(
            "Files without extension aren't supported",
        ))? == "npy"
        {
            let arr: Array3<u8> =
                read_npy(img_path).map_err(|_| PyIOError::new_err("Can't read 'npy' file"))?;
            let (h, w, c) = arr.dim();
            if c == 1 {
                let img = ImageKind::OneChannel(Image::<u8,1>::new(
                    ImageSize {
                        width: w,
                        height: h,
                    },
                    arr.into_raw_vec_and_offset().0,
                )
                    .unwrap());
                imgs.push(img)
            } else if c == 3 {
                let img = ImageKind::ThreeChannel(Image::<u8,3>::new(
                    ImageSize {
                        width: w,
                        height: h,
                    },
                    arr.into_raw_vec_and_offset().0,
                )
                    .unwrap());
                imgs.push(img)

            } else {
                return Err(PyTypeError::new_err("Can accept only images with 1 or 3 channels"))
            }


        } else {
            let img = ImageKind::ThreeChannel(read_image_any(img_path)
                .map_err(|_| PyIOError::new_err(format!("Can't read {:?} image", &img_path)))?);
            imgs.push(img);
        }
    }
    if let Err(err) = imspect_kornia_images(imgs) {
        return Err(PyRuntimeError::new_err(err.to_string()));
    };
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn imspect(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_imspect_for_shell, m)?)?;
    m.add_function(wrap_pyfunction!(_imspect_script, m)?)?;

    Ok(())
}
