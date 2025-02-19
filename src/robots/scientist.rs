mod robot;
 
use robot::RobotType;
 
struct Scientist {}
 
impl Robot for Scientist {
    fn new() -> Self {
        Scientist {
            robot_type: RobotType::Scientist,
            icon: '🔬',
            x: 0,
            y: 0,
            energy: 250,
            cargo: vec![],
            discovered_map: HashSet::new()
        }
    }
}