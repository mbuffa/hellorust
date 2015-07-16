use super::super::entities::unit::ConcreteUnit;

pub trait Component {
  fn update(&mut self, unit: &mut ConcreteUnit);
}
