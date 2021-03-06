use app::context::*;
use app::event::AppEvent;

pub fn update(ctx: &mut AppContext) {
  println!("Calling Update from init function.");
  ctx.do_stuff();

  ctx.trigger(AppEvent::Play);
}

pub fn on_enter(ctx: &mut AppContext) {
  println!("Entering init function.");
}

pub fn on_leave(ctx: &mut AppContext) {
  println!("Leaving init function.");
}
