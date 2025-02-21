use iced::{
    executor,
    widget::{Button, Column, Container, Row, Text},
    Application, Command, Element, Font, Length, Settings, Theme,
};
use crate::map::Map;

pub struct MapWindow {
    map_content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateExplorer,
    CreateHarvester,
    CreateScientist,
}

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
        match _message {
            Message::CreateExplorer => {
                // Create an explorer robot
            }
            Message::CreateHarvester => {
                // Create a harvester robot
            }
            Message::CreateScientist => {
                // Create a scientist robot
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let bas_font = Font::with_name("Segoe UI Emoji");

        let controls = Column::new()
            .spacing(10)
            .padding(10)
            .width(Length::FillPortion(2))
            .push(create_button("Create Explorer", Message::CreateExplorer))
            .push(create_button("Create Harvester", Message::CreateHarvester))
            .push(create_button("Create Scientist", Message::CreateScientist));

        let map = Container::new(Text::new(&self.map_content).size(18).font(Font {
            family: bas_font.family,
            weight: iced::font::Weight::Bold,
            stretch: iced::font::Stretch::UltraCondensed,
            monospaced: false,
        }))
        .width(Length::FillPortion(8));

        Container::new(Row::new().push(map).push(controls).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }
}

pub fn create_button(label: &str, message: Message) -> Button<Message> {
    Button::new(
        Text::new(label)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .on_press(message)
}

pub fn open_window(map: Map) -> Result<(), Box<dyn std::error::Error>> {
    let tile_size = 18;
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
