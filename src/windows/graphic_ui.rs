use iced::widget::{Column, Container, Row, Space, Text};
use iced::{
    executor, Application, Command, Element, Font, Length, Subscription, Theme, time
};

use crate::robots::robot::RobotType;
use crate::simulation::simulation::Simulation;


use super::utils::create_button;

pub struct MapWindow {
    simulation: Simulation,
    map_content: String
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    CreateExplorer,
    CreateHarvester,
    Pause,
    Play
}

impl Application for MapWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Simulation;

    fn new(simulation: Simulation) -> (Self, Command<Message>) {
        let mut map_content = String::new();
        
        if let Ok(map_guard) = simulation.map.lock() {
            for y in 0..map_guard.height {
                for x in 0..map_guard.width {
                    let tile = map_guard.get(x, y).tile;
                    map_content.push(tile.char());
                }
                map_content.push('\n');
            }
        }

        (MapWindow { map_content, simulation }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Map Window")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if let Ok(map_guard) = self.simulation.map.try_lock() {
                    let mut map_content = String::new();
                    for y in 0..map_guard.height {
                        for x in 0..map_guard.width {
                            let tile = map_guard.get(x, y).tile;
                            map_content.push(tile.char());
                        }
                        map_content.push('\n');
                    }
                    self.map_content = map_content;
                }
            }
            Message::CreateExplorer => {
                self.simulation.create_robot(RobotType::Explorer);
            },
            Message::CreateHarvester => {
                self.simulation.create_robot(RobotType::Harvester);
            },
            Message::Pause => self.simulation.pause(),
            Message::Play => self.simulation.play(),
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(33)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        let bas_font = Font::with_name("Segoe UI Emoji");
        
        let simulation_status =  format!(
            "Simulation status\nEnergy: {}\nResources: {}",
            self.simulation.energy_count,
            self.simulation.resource_count,
        );

        let toggle_simulation_state = || -> Message {
            match self.simulation.running.load(std::sync::atomic::Ordering::SeqCst) {
                false => Message::Play,
                true => Message::Pause,
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
                .size(20)
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
