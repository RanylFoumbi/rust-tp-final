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
    SendExplorer,
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
                self.simulation.compute_fps();
                if let Ok(mut map) = self.simulation.map.write() {
                    self.map_grid.update(&mut map);
                } else {
                    eprintln!("Failed to lock map for update");
                }
            }
            Message::SendExplorer => {
                self.simulation.send_robot(RobotType::Explorer, |_| {});
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
        let is_running = self
            .simulation
            .running
            .load(std::sync::atomic::Ordering::SeqCst);

        let located_resources_count = {
            let located_resources = self.simulation.located_resources.lock().unwrap();
            located_resources.len()
        };
        let energy_count = self.simulation.energy_count.lock().unwrap();
        let resource_count = self.simulation.resource_count.lock().unwrap();
        let simulation_status = format!(
            "Simulation status\nFPS: {}\nResources located: {}\nResources harvested: {}\nEnergy: {}",
            self.simulation.fps, located_resources_count, resource_count, energy_count,
        );

        let toggle_simulation_state = || -> Message {
            match is_running {
                false => Message::Play,
                true => Message::Pause,
            }
        };

        let controls = Column::new()
            .spacing(10)
            .padding(10)
            .width(Length::FillPortion(2))
            .push(Text::new(simulation_status).font(self.map_grid.font))
            .push(Space::with_height(20))
            .push(create_button(
                "Send Explorer",
                Message::SendExplorer,
                is_running,
            ))
            .push(Space::with_height(20))
            .push(create_button("Play/Pause", toggle_simulation_state(), true))
            .push(
                Row::new()
                    .push(create_button("Speed +", Message::UpSpeed, true))
                    .push(Space::with_width(10))
                    .push(create_button("Speed -", Message::DownSpeed, true)),
            );

        let map = self.map_grid.view().map(|_| Message::Tick);

        Container::new(
            Row::new()
                .push(Container::new(map)
                .width(Length::FillPortion(8)))
                .push(controls),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0)
        .into()
    }
}
