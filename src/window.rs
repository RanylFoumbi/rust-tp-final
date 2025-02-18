use iced::{
    widget::{Container, Text},
    Element, Length, Settings, Theme,
    Application, Command, executor, Font
};
use crate::map::Map;

pub struct MapWindow {
    map_content: String,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for MapWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Map;

    fn new(map: Map) -> (Self, Command<Message>) {
        let mut map_content = String::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let c = map.get(x, y);
                map_content.push(c);
            }
            map_content.push('\n');
        }

        (MapWindow { map_content }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Map Window")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let bas_font = Font::with_name("Segoe UI Emoji");
        Container::new(
            Text::new(&self.map_content)
                .size(18)
                .font(Font{
                    family: bas_font.family,
                    weight: iced::font::Weight::Bold,
                    stretch: iced::font::Stretch::UltraCondensed,
                    monospaced: false
                })
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0)
        .into()
    }
}

pub fn open_window(map: Map) -> Result<(), Box<dyn std::error::Error>> {
    MapWindow::run(Settings::with_flags(map))?;
    Ok(())
}