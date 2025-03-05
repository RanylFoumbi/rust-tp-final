use iced::widget::{Column, Container, Row, Text};
use iced::{executor, Application, Command, Element, Length, Subscription, Theme, time};
use crate::simulation::Simulation;
use super::utils::create_button;
use crate::robots::robot::Robot;
use crate::robots::explorer::Explorer;
use crate::robots::harvester::Harvester;

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
                    map_content.push(map_guard.get(x, y).tile.char());
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
                if let Ok(map_guard) = self.simulation.map.lock() {
                    let mut map_content = String::new();
                    for y in 0..map_guard.height {
                        for x in 0..map_guard.width {
                            map_content.push(map_guard.get(x, y).tile.char());
                        }
                        map_content.push('\n');
                    }
                    self.map_content = map_content;
                }
            }
            Message::CreateExplorer => {
                self.simulation.add_robot(Box::new(Explorer::new(2, 2)));
            },
            Message::CreateHarvester => {
                self.simulation.add_robot(Box::new(Harvester::new(3, 3)));
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
        let controls = Column::new()
            .spacing(10)
            .padding(10)
            .push(create_button("Create Explorer", Message::CreateExplorer))
            .push(create_button("Create Harvester", Message::CreateHarvester))
            .push(create_button("Play/Pause", Message::Play));

        let map = Container::new(Text::new(&self.map_content).size(20));

        Container::new(Row::new().push(map).push(controls).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }
}
