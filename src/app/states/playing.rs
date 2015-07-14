use app::context::*;
use app::event::AppEvent;

pub fn update(ctx: &mut AppContext) {
  println!("Calling Update from play function.");

  ctx.update_game();

  ctx.trigger(AppEvent::Quit);
}

pub fn on_enter(ctx: &mut AppContext) {
  println!("Entering play function.");
}

pub fn on_leave(ctx: &mut AppContext) {
  println!("Leaving play function.");
}
