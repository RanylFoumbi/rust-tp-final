use iced::{widget::{Button, Text}, Application, Length, Settings};
use crate::simulation::Simulation;
use super::graphic_ui::{MapWindow, Message};

pub fn create_button(label: &str, message: Message) -> Button<Message> {
    Button::new(Text::new(label)).width(Length::Fill).on_press(message)
}

pub fn open_window(simulation: Simulation) -> iced::Result {
    let settings = Settings::with_flags(simulation);
    MapWindow::run(settings)?;
    Ok(())
}
