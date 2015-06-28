use super::{Context, GameContext, Event};

pub fn update(ctx: &mut GameContext) {
  println!("Calling Update from play function.");
  ctx.do_stuff();

  ctx.trigger(Event::Quit);
}

pub fn on_enter(ctx: &mut GameContext) {
  println!("Entering play function.");
}

pub fn on_leave(ctx: &mut GameContext) {
  println!("Leaving play function.");
}
