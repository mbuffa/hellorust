use super::super::entities::unit::Unit;

pub trait Component {
  fn update(&mut self, unit: &mut Unit);
}
