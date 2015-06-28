use super::{Context, GameContext, Event};

pub fn update(ctx: &mut GameContext) {
  println!("Calling Update from init function.");
  ctx.do_stuff();

  ctx.trigger(Event::Play);
}

pub fn on_enter(ctx: &mut GameContext) {
  println!("Entering init function.");
}

pub fn on_leave(ctx: &mut GameContext) {
  println!("Leaving init function.");
}
