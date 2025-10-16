#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use image::{GenericImageView, ImageReader};
use slint::{Image, Model, ModelRc, SharedPixelBuffer};

slint::include_modules!();

// mod controllers;
mod model;
mod mock;
mod view_model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let state = model::State::new();
    
    // Erstelle ContentVM - UI muss stark referenziert bleiben
    let _content_vm = view_model::content::ContentVM::new(ui.as_weak(), state.clone());
    let _messages_vm = view_model::messages::MessagesVM::new(ui.as_weak(), state.clone());
    
    // Zugriff auf Content Global
    let _content_global = ui.global::<Content>();
    
    // Lade Test-Daten
    state.dispatch(model::actions::Action::LoadLibraryEntry(0));


    // let controller = controllers::ContentController::new(ui.clone(), state);

    ui.global::<Helper>().on_get_image(|image: ModelRc<i32>| {
        let cb = move || {
            let data_i32 = image.iter().collect::<Vec<i32>>();
            let data_u8: Vec<u8> = data_i32.iter().map(|&v| v as u8).collect();
            let img = ImageReader::new(std::io::Cursor::new(data_u8))
                .with_guessed_format()
                .map_err(|e| slint::platform::PlatformError::from(format!("{:?}", e)))?
                .decode()
                .map_err(|e| slint::platform::PlatformError::from(format!("{:?}", e)))?;

            let (width, height) = img.dimensions();

            Ok(Image::from_rgb8(SharedPixelBuffer::clone_from_slice(
                img.as_bytes(),
                width,
                height,
            )))
        };
        cb().unwrap_or_else(|_err: slint::platform::PlatformError| Image::default())
    });

    // let img = ImageReader::new(std::io::Cursor::new(decoded_bytes))
    //     .with_guessed_format()?
    //     .decode()?;
    //
    // // let (width, height) = img.dimensions();
    //
    // let image_data = slint::Image::from_rgb8(SharedPixelBuffer::clone_from_slice(
    //     img.as_bytes(),
    //     180,
    //     180,
    // ));
    // ui.set_image_url(image_data);

    ui.run()?;

    Ok(())
}
