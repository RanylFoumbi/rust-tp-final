use iced::{widget::{Button, Text}, Application, Length, Settings};

use crate::environment::map::Map;

use super::graphic_ui::{MapWindow, Message};
use std::sync::{Arc, Mutex};

pub fn create_button(label: &str, message: Message) -> Button<Message> {
    Button::new(
        Text::new(label)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .width(Length::FillPortion(10)),
    )
    .width(Length::Fill)
    .on_press(message)
}

pub fn open_window(map: Arc<Mutex<Map>>) -> Result<(), Box<dyn std::error::Error>> {
    let tile_size = 16;
    let control_width = 200;
    let padding = 20;

    let (window_width, window_height) = {
        let map_guard = map.lock().unwrap();
        let width = (map_guard.width as u32 * tile_size) + control_width + padding;
        let height = (map_guard.height as u32 * tile_size) + padding;
        (width, height)
    };
    
    println!("Fenêtre ajustée : {}x{}", window_width, window_height);

    let settings = Settings {
        window: iced::window::Settings {
            size: (window_width, window_height),
            resizable: false,
            ..Default::default()
        },
        antialiasing: true,
        exit_on_close_request: true,
        ..Settings::with_flags(map)
    };


    MapWindow::run(settings)?;
    Ok(())
}