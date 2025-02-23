use iced::widget::{Column, Container, Row, Space, Text};
use iced::{
    executor, Application, Command, Element, Font, Length, Theme
};
use crate::environment::map::Map;
use crate::simulation::{Simulation, SimulationState};

use super::utils::create_button;

pub struct MapWindow {
    map_content: String,
    simulation: Simulation
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateExplorer,
    CreateHarvester,
    CreateScientist,
    Pause,
    Play
}

impl Application for MapWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Map;

    fn new(map: Map) -> (Self, Command<Message>) {
        let mut map_content = String::new();
        let mut simulation = Simulation::new(&map);
        
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = map.get(x, y);
                map_content.push(tile.char);
            }
            map_content.push('\n');
        }

        (MapWindow { map_content, simulation }, Command::none())
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
            Message::Pause => self.simulation.state = SimulationState::Pause,
            Message::Play => self.simulation.state = SimulationState::Play
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let bas_font = Font::with_name("Segoe UI Emoji");
        
        let simulation_status  = format!(
            "Simulation status\nEnergy: {}\nResources: {}\nScientist Areas: {}",
            self.simulation.energy_count,
            self.simulation.resource_count,
            self.simulation.scientist_area_count
        );

        let toggle_simulation_state = || -> Message {
            match self.simulation.state {
                SimulationState::Pause => Message::Play,
                SimulationState::Play => Message::Pause,
            }
        };

        let controls = Column::new()
            .spacing(10)
            .padding(10)
            .width(Length::FillPortion(2))
            .push(Text::new(simulation_status))
            .push(Space::with_height(20))
            .push(create_button("Create Explorer", Message::CreateExplorer))
            .push(create_button("Create Harvester", Message::CreateHarvester))
            .push(Space::with_height(20))
            .push(create_button("Play/Pause", toggle_simulation_state()));

        let map = Container::new(
            Text::new(&self.map_content)
                .size(16)
                .font(Font {
                    family: bas_font.family,
                    weight: iced::font::Weight::Bold,
                    stretch: iced::font::Stretch::UltraCondensed,
                    monospaced: false,
                })
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center)
        )
        .width(Length::FillPortion(8))
        .center_x()
        .center_y();

        Container::new(Row::new().push(map).push(controls).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }
}
