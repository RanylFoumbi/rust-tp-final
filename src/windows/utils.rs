use iced::{widget::{Button, Text}, Application, Length, Settings};

use crate::map::Map;

use super::graphic_ui::{MapWindow, Message};

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

pub fn open_window(map: Map) -> Result<(), Box<dyn std::error::Error>> {
    let tile_size = 16;
    let control_width = 200;
    let padding = 20;

    let window_width = (map.width as u32 * tile_size) + control_width + padding;
    let window_height = (map.height as u32 * tile_size) + padding;

    println!("Fenêtre ajustée : {}x{}", window_width, window_height);

    let settings = Settings {
        window: iced::window::Settings {
            size: (window_width, window_height),
            resizable: false,
            ..Default::default()
        },
        ..Settings::with_flags(map)
    };


    MapWindow::run(settings)?;
    Ok(())
}