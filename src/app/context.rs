use super::state::AppState;
use super::states::*;
use super::event::AppEvent;

pub struct AppContext {
  current: AppState,
  update_fn: Option<fn(&mut AppContext) -> ()>,
  on_enter_fn: Option<fn(&mut AppContext) -> ()>,
  on_leave_fn: Option<fn(&mut AppContext) -> ()>
}

impl AppContext {
  pub fn new() -> AppContext {
    let mut ctx = AppContext {
      current: AppState::Initializing,
      update_fn: Some(initializing::update),
      on_enter_fn: Some(initializing::on_enter),
      on_leave_fn: Some(initializing::on_leave)
    };
    (ctx.on_enter_fn.unwrap())(&mut ctx);
    ctx
  }

  pub fn do_stuff(&mut self) {
    println!("Doin' stuff!");
  }
}

pub trait Context {
  fn update(&mut self) -> bool;
  fn trigger(&mut self, event: AppEvent);
}

impl Context for AppContext {
  fn update(&mut self) -> bool {
    match self.update_fn {
      None => { false },
      _ => {
        (self.update_fn.unwrap())(self);
        true
      }
    }
  }

  fn trigger(&mut self, event: AppEvent) {
    match event {
      AppEvent::Play => {
        match self.on_leave_fn {
          None => {},
          _ => {
            (self.on_leave_fn.unwrap())(self);
          }
        }

        self.current = AppState::Playing;
        self.update_fn = Some(playing::update);
        self.on_enter_fn = Some(playing::on_enter);
        self.on_leave_fn = Some(playing::on_leave);

        (self.on_enter_fn.unwrap())(self);
      },
      AppEvent::Quit => {
        self.update_fn = None;
      }
    }
  }
}
