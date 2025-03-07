use iced::widget::{Column, Container, Row, Space, Text};
use iced::{executor, time, Application, Command, Element, Font, Length, Subscription, Theme};

use crate::robots::robot::RobotType;
use crate::simulation::simulation::Simulation;

use super::map_grid::MapGrid;
use super::utils::create_button;

pub struct MapWindow {
    simulation: Simulation,
    map_grid: MapGrid,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    CreateExplorer,
    CreateHarvester,
    Pause,
    Play,
    UpSpeed,
    DownSpeed,
}

impl Application for MapWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Simulation;

    fn new(simulation: Simulation) -> (Self, Command<Message>) {
        let bas_font = Font::with_name("Segoe UI Emoji");
        let map_grid = MapGrid::new(simulation.map.clone(), bas_font);

        (
            MapWindow {
                map_grid,
                simulation,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Map Window")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                if let Ok(mut map) = self.simulation.map.write() {
                    self.map_grid.update(&mut map);
                } else {
                    eprintln!("Failed to lock map for update");
                }
            }
            Message::CreateExplorer => {
                self.simulation.create_robot(RobotType::Explorer);
            }
            Message::CreateHarvester => {
                self.simulation.create_robot(RobotType::Harvester);
            }
            Message::Pause => self.simulation.pause(),
            Message::Play => self.simulation.play(),
            Message::UpSpeed => self.simulation.increase_speed(),
            Message::DownSpeed => self.simulation.decrease_speed(),
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(33)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        let bas_font = Font::with_name("Segoe UI Emoji");

        let simulation_status = format!(
            "Simulation status\nEnergy: {}\nResources: {}",
            self.simulation.energy_count, self.simulation.resource_count,
        );

        let toggle_simulation_state = || -> Message {
            match self
                .simulation
                .running
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                false => Message::Play,
                true => Message::Pause,
            }
        };

        let controls = Column::new()
            .spacing(10)
            .padding(10)
            .width(Length::FillPortion(2)) 
            .push(Text::new(simulation_status).font(bas_font))
            .push(Space::with_height(20))
            .push(create_button("Create Explorer", Message::CreateExplorer))
            .push(create_button("Create Harvester", Message::CreateHarvester))
            .push(Space::with_height(20))
            .push(create_button("Play/Pause", toggle_simulation_state()))
            .push(Row::new()
                .push(create_button("Speed +", Message::UpSpeed))
                .push(Space::with_width(10))
                .push(create_button("Speed -", Message::DownSpeed))
            );

        let map = self.map_grid.view().map(|_| Message::Tick);

        Container::new(
            Row::new()
                .push(Container::new(map).width(Length::FillPortion(8)))
                .push(controls)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0)
        .into()
    }
}
