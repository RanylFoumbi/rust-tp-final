mod robot;
 
use robot::RobotType;
 
struct Harvester {}
 
impl Robot for Harvester {
    fn new() -> Self {
        Harvester {
            robot_type: RobotType::Harvester,
            icon: '🚜',
            x: 0,
            y: 0,
            energy: 200,
            cargo: vec![],
            discovered_map: HashSet::new()
        }
    }
}