use super::tile::Tile;
use super::tile::TileType;

pub struct Map {
  tiles: Vec<Vec<Tile>>,
  width: usize,
  depth: usize
}

impl Map {
  pub fn new(width: usize, depth: usize) -> Map {
    let mut tiles = Vec::with_capacity(width);
    let mut real_width = 0;
    let mut real_depth = 0;

    for _ in 0..width {
      let mut vector: Vec<Tile> = Vec::with_capacity(depth);

      for _ in 0..depth {
        vector.push(Tile::new(true, TileType::Dirt));
      }
      real_depth = vector.len();
      tiles.push(vector);
    }
    real_width = tiles.len();

    Map {
      tiles: tiles,
      width: real_width,
      depth: real_depth
    }
  }

  pub fn is_initialized(&self) -> bool {
    self.tiles.len() == self.width &&
    self.tiles[self.depth - 1].len() == self.depth
  }

  pub fn tile_at(&self, width: usize, depth: usize) -> &Tile {
    &self.tiles[width][depth]
  }
}

#[test]
fn map_initialized() {
  let map = Map::new(16, 16);
  assert_eq!(map.is_initialized(), true);
}

#[test]
fn map_right_size() {
  let map = Map::new(5, 21);
  assert_eq!(map.width, 5);
  assert_eq!(map.depth, 21);
}