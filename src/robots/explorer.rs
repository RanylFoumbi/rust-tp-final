mod robot;
 
use robot::RobotType;
 
struct Explorer {}
 
impl Robot for Explorer {
    fn new() -> Self {
        Explorer {
            robot_type: RobotType::Explorer,
            icon: '🤖',
            x: 0,
            y: 0,
            energy: 150,
            cargo: vec![],
            discovered_map: HashSet::new()
        }
    }
}