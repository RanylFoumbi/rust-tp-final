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
    let settings = Settings {
        window: iced::window::Settings {
            size: (800, 600),
            resizable: false,
            ..Default::default()
        },
        ..Settings::with_flags(map)
    };

    MapWindow::run(settings)?;
    Ok(())
}