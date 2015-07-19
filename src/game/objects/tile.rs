pub enum TileType {
  Dirt,
  Grass
}

pub struct Tile {
  walkable: bool,
  tile_type: TileType
}

impl Tile {
  pub fn new(walkable: bool, tile_type: TileType) -> Tile {
    return Tile {
      walkable: walkable,
      tile_type: tile_type
    }
  }

  pub fn is_walkable(&self) -> bool { self.walkable }
}
