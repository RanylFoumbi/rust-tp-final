#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Default,
    Base,
    Resource(Resource),
}

#[derive(Debug, Clone, Copy)]
pub struct MapTile {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
    pub char: char,
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Energy,
    Mineral,
}

#[derive(Debug, Clone, Copy)]
pub struct Resource {
    scale: f64,
    resource_type: ResourceType,
}

impl Resource {
    pub fn new(scale: f64, resource_type: ResourceType) -> Self {
        Resource { scale, resource_type }
    }
}

impl MapTile {
    pub fn new(x: usize, y: usize, char: char, tile_type: TileType) -> Self {
        MapTile { x, y, char, tile_type }
    }
}