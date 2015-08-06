use app::context::*;
use app::event::AppEvent;

pub fn update(ctx: &mut AppContext) {
  println!("Calling Update from init function.");

  ctx.trigger(AppEvent::Play);
}

pub fn on_enter(ctx: &mut AppContext) {
  println!("Entering init function.");

  ctx.get_settings().load();
}

pub fn on_leave(ctx: &mut AppContext) {
  println!("Leaving init function.");
}
