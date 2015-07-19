use super::super::objects::entity::ConcreteEntity;

pub trait Component {
  fn update(&mut self, entity: &mut ConcreteEntity);
}
